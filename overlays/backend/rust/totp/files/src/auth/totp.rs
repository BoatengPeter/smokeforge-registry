//! TOTP enrollment, recovery codes, and second-factor verification.

use std::sync::Arc;

use async_trait::async_trait;
use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use sha2::{Digest, Sha256};
use sqlx::{PgPool, Postgres, Transaction};
use utoipa::OpenApi;
use uuid::Uuid;

use super::{AuthService, SecondFactor};
use crate::error::AppError;

const DIGITS: u32 = 6;
const PERIOD: u64 = 30;
const RECOVERY_CODE_COUNT: usize = 10;
const RECOVERY_CODE_BYTES: usize = 5;
const ISSUER: &str = "smokeforge";

/// TOTP routes and persistence.
pub struct TotpService {
    pool: PgPool,
}

/// Builds the TOTP router.
pub fn router() -> Router {
    Router::new()
        .route("/api/v1/auth/totp/status", get(status))
        .route("/api/v1/auth/totp/setup", post(setup))
        .route("/api/v1/auth/totp/enable", post(enable))
        .route("/api/v1/auth/totp/disable", post(disable))
        .route("/api/v1/auth/totp/verify", post(verify))
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct SetupResponse {
    pub secret: String,
    pub otpauth_url: String,
    pub digits: u32,
    pub period: u64,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct EnableResponse {
    pub codes: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, utoipa::ToSchema)]
pub struct StatusResponse {
    pub enabled: bool,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CodeRequest {
    pub code: String,
}

impl TotpService {
    /// Creates a TOTP service backed by the shared database pool.
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn status(&self, user_id: Uuid) -> Result<bool, AppError> {
        sqlx::query_scalar::<_, bool>("SELECT enabled FROM user_totp WHERE user_id = $1")
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map(|value| value.unwrap_or(false))
            .map_err(|_| AppError::internal("could not load TOTP state"))
    }

    async fn setup(&self, user_id: Uuid, email: &str) -> Result<SetupResponse, AppError> {
        if self.status(user_id).await? {
            return Err(AppError::new(
                StatusCode::CONFLICT,
                "totp_enabled",
                "TOTP already enabled",
                "Disable TOTP before starting a new enrollment.",
            ));
        }

        let secret = generate_secret();
        let otpauth_url = format!(
            "otpauth://totp/{ISSUER}:{email}?secret={secret}&issuer={ISSUER}&digits={DIGITS}&period={PERIOD}"
        );

        sqlx::query(
            "INSERT INTO user_totp (user_id, secret, enabled)
             VALUES ($1, $2, false)
             ON CONFLICT (user_id) DO UPDATE SET secret = EXCLUDED.secret, enabled = false",
        )
        .bind(user_id)
        .bind(&secret)
        .execute(&self.pool)
        .await
        .map_err(|_| AppError::internal("could not store TOTP secret"))?;

        Ok(SetupResponse {
            secret,
            otpauth_url,
            digits: DIGITS,
            period: PERIOD,
        })
    }

    async fn enable(&self, user_id: Uuid, code: &str) -> Result<Vec<String>, AppError> {
        let state: Option<(String, bool)> =
            sqlx::query_as("SELECT secret, enabled FROM user_totp WHERE user_id = $1")
                .bind(user_id)
                .fetch_optional(&self.pool)
                .await
                .map_err(|_| AppError::internal("could not load TOTP state"))?;

        let (secret, enabled) = state.ok_or_else(|| {
            AppError::new(
                StatusCode::CONFLICT,
                "setup_required",
                "TOTP setup required",
                "Start TOTP setup before enabling it.",
            )
        })?;

        if enabled {
            return Err(AppError::new(
                StatusCode::CONFLICT,
                "totp_enabled",
                "TOTP already enabled",
                "TOTP is already enabled for this account.",
            ));
        }

        if !verify_code(&secret, code, unix_time())? {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_code",
                "Invalid code",
                "The verification code is invalid.",
            ));
        }

        let codes = recovery_codes();
        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_| AppError::internal("could not start TOTP transaction"))?;

        set_enabled(&mut tx, user_id, true).await?;
        clear_recovery_codes(&mut tx, user_id).await?;
        store_recovery_codes(&mut tx, user_id, &codes).await?;

        tx.commit()
            .await
            .map_err(|_| AppError::internal("could not commit TOTP enrollment"))?;

        Ok(codes)
    }

    async fn disable(&self, user_id: Uuid, code: &str) -> Result<(), AppError> {
        if !self.status(user_id).await? {
            return Err(AppError::new(
                StatusCode::CONFLICT,
                "totp_not_enabled",
                "TOTP not enabled",
                "TOTP is not enabled for this account.",
            ));
        }

        self.verify(user_id, code).await?;

        let mut tx = self
            .pool
            .begin()
            .await
            .map_err(|_| AppError::internal("could not start TOTP transaction"))?;

        set_enabled(&mut tx, user_id, false).await?;
        clear_recovery_codes(&mut tx, user_id).await?;
        tx.commit()
            .await
            .map_err(|_| AppError::internal("could not disable TOTP"))?;

        Ok(())
    }

    async fn verify(&self, user_id: Uuid, code: &str) -> Result<(), AppError> {
        let secret: Option<String> = sqlx::query_scalar(
            "SELECT secret FROM user_totp WHERE user_id = $1 AND enabled = true",
        )
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(|_| AppError::internal("could not load TOTP secret"))?;

        let secret = secret.ok_or_else(|| {
            AppError::new(
                StatusCode::CONFLICT,
                "totp_not_enabled",
                "TOTP not enabled",
                "TOTP is not enabled for this account.",
            )
        })?;

        if let Some(recovery_id) = consume_recovery_code(&self.pool, user_id, code).await? {
            tracing::info!(%recovery_id, %user_id, "consumed a TOTP recovery code");
            return Ok(());
        }

        if !verify_code(&secret, code, unix_time())? {
            return Err(AppError::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                "invalid_code",
                "Invalid code",
                "The verification code is invalid.",
            ));
        }

        Ok(())
    }
}

#[async_trait]
impl SecondFactor for TotpService {
    async fn required(&self, user_id: Uuid) -> Result<bool, AppError> {
        self.status(user_id).await
    }

    async fn verify(&self, user_id: Uuid, code: &str) -> Result<(), AppError> {
        self.verify(user_id, code).await
    }
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/totp/status",
    responses(
        (status = 200, body = StatusResponse),
        (status = 401, body = crate::response::ProblemDetails)
    )
)]
async fn status(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
) -> Result<Json<StatusResponse>, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let totp = TotpService::new(service.pool().clone());

    Ok(Json(StatusResponse {
        enabled: totp.status(user.id).await?,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/totp/setup",
    responses(
        (status = 200, body = SetupResponse),
        (status = 401, body = crate::response::ProblemDetails),
        (status = 409, body = crate::response::ProblemDetails)
    )
)]
async fn setup(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
) -> Result<Json<SetupResponse>, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let totp = TotpService::new(service.pool().clone());

    Ok(Json(totp.setup(user.id, &user.email).await?))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/totp/enable",
    request_body = CodeRequest,
    responses(
        (status = 200, body = EnableResponse),
        (status = 401, body = crate::response::ProblemDetails),
        (status = 409, body = crate::response::ProblemDetails),
        (status = 422, body = crate::response::ProblemDetails)
    )
)]
async fn enable(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(input): Json<CodeRequest>,
) -> Result<Json<EnableResponse>, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let totp = TotpService::new(service.pool().clone());

    Ok(Json(EnableResponse {
        codes: totp.enable(user.id, &input.code).await?,
    }))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/totp/disable",
    request_body = CodeRequest,
    responses(
        (status = 204),
        (status = 401, body = crate::response::ProblemDetails),
        (status = 409, body = crate::response::ProblemDetails),
        (status = 422, body = crate::response::ProblemDetails)
    )
)]
async fn disable(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(input): Json<CodeRequest>,
) -> Result<StatusCode, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let totp = TotpService::new(service.pool().clone());
    totp.disable(user.id, &input.code).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/totp/verify",
    request_body = CodeRequest,
    responses(
        (status = 204),
        (status = 401, body = crate::response::ProblemDetails),
        (status = 409, body = crate::response::ProblemDetails),
        (status = 422, body = crate::response::ProblemDetails)
    )
)]
async fn verify(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(input): Json<CodeRequest>,
) -> Result<StatusCode, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let totp = TotpService::new(service.pool().clone());
    totp.verify(user.id, &input.code).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn set_enabled(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    enabled: bool,
) -> Result<(), AppError> {
    sqlx::query("UPDATE user_totp SET enabled = $2, updated_at = now() WHERE user_id = $1")
        .bind(user_id)
        .bind(enabled)
        .execute(&mut **tx)
        .await
        .map_err(|_| AppError::internal("could not update TOTP state"))?;

    Ok(())
}

async fn clear_recovery_codes(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
) -> Result<(), AppError> {
    sqlx::query("DELETE FROM user_totp_recovery_codes WHERE user_id = $1")
        .bind(user_id)
        .execute(&mut **tx)
        .await
        .map_err(|_| AppError::internal("could not clear recovery codes"))?;

    Ok(())
}

async fn store_recovery_codes(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    codes: &[String],
) -> Result<(), AppError> {
    for code in codes {
        sqlx::query(
            "INSERT INTO user_totp_recovery_codes (id, user_id, code_hash)
             VALUES ($1, $2, $3)",
        )
        .bind(Uuid::new_v4())
        .bind(user_id)
        .bind(hash_recovery_code(code))
        .execute(&mut **tx)
        .await
        .map_err(|_| AppError::internal("could not store recovery code"))?;
    }

    Ok(())
}

async fn consume_recovery_code(
    pool: &PgPool,
    user_id: Uuid,
    code: &str,
) -> Result<Option<Uuid>, AppError> {
    sqlx::query_scalar(
        "UPDATE user_totp_recovery_codes
         SET used_at = now()
         WHERE user_id = $1 AND code_hash = $2 AND used_at IS NULL
         RETURNING id",
    )
    .bind(user_id)
    .bind(hash_recovery_code(code))
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::internal("could not consume recovery code"))
}

fn bearer(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))
}

fn recovery_codes() -> Vec<String> {
    (0..RECOVERY_CODE_COUNT)
        .map(|_| new_recovery_code())
        .collect()
}

fn new_recovery_code() -> String {
    let bytes = Uuid::new_v4();
    hex::encode(&bytes.as_bytes()[..RECOVERY_CODE_BYTES])
}

fn hash_recovery_code(code: &str) -> String {
    hex::encode(Sha256::digest(code.as_bytes()))
}

fn generate_secret() -> String {
    let first = Uuid::new_v4();
    let second = Uuid::new_v4();
    let mut bytes = [0_u8; 20];
    bytes[..16].copy_from_slice(first.as_bytes());
    bytes[16..].copy_from_slice(&second.as_bytes()[..4]);

    base32_encode(&bytes)
}

fn verify_code(secret: &str, code: &str, now: u64) -> Result<bool, AppError> {
    if code.len() != DIGITS as usize || !code.chars().all(|value| value.is_ascii_digit()) {
        return Ok(false);
    }

    let counter = now / PERIOD;
    for offset in [-1_i64, 0, 1] {
        let step = counter.saturating_add_signed(offset);
        let expected = hotp(secret, step)?;

        if expected == code {
            return Ok(true);
        }
    }

    Ok(false)
}

fn hotp(secret: &str, counter: u64) -> Result<String, AppError> {
    let key =
        base32_decode(secret).ok_or_else(|| AppError::internal("stored TOTP secret is invalid"))?;
    let mut mac = Hmac::<Sha1>::new_from_slice(&key)
        .map_err(|_| AppError::internal("TOTP key is invalid"))?;
    mac.update(&counter.to_be_bytes());
    let digest = mac.finalize().into_bytes();
    let offset = (digest[19] & 0x0f) as usize;
    let binary = ((digest[offset] as u32 & 0x7f) << 24)
        | ((digest[offset + 1] as u32) << 16)
        | ((digest[offset + 2] as u32) << 8)
        | digest[offset + 3] as u32;
    let value = binary % 10_u32.pow(DIGITS);

    Ok(format!("{value:0width$}", width = DIGITS as usize))
}

fn unix_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}

fn base32_encode(bytes: &[u8]) -> String {
    const ALPHABET: &[u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";
    let mut output = String::with_capacity(bytes.len().div_ceil(5) * 8);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for byte in bytes {
        buffer = (buffer << 8) | u32::from(*byte);
        bits += 8;

        while bits >= 5 {
            bits -= 5;
            output.push(ALPHABET[((buffer >> bits) & 0x1f) as usize] as char);
        }
    }

    if bits > 0 {
        output.push(ALPHABET[((buffer << (5 - bits)) & 0x1f) as usize] as char);
    }

    output
}

fn base32_decode(value: &str) -> Option<Vec<u8>> {
    let mut output = Vec::with_capacity(value.len() * 5 / 8);
    let mut buffer = 0_u32;
    let mut bits = 0_u8;

    for character in value.bytes() {
        let digit = match character {
            b'A'..=b'Z' => character - b'A',
            b'a'..=b'z' => character - b'a',
            b'2'..=b'7' => character - b'2' + 26,
            b'=' => break,
            _ => return None,
        };
        buffer = (buffer << 5) | u32::from(digit);
        bits += 5;

        if bits >= 8 {
            bits -= 8;
            output.push(((buffer >> bits) & 0xff) as u8);
        }
    }

    Some(output)
}

/// OpenAPI document for TOTP routes.
#[derive(OpenApi)]
#[openapi(
    paths(status, setup, enable, disable, verify),
    components(schemas(SetupResponse, EnableResponse, StatusResponse, CodeRequest))
)]
pub struct TotpApi;

/// Returns the TOTP OpenAPI document.
pub fn document() -> utoipa::openapi::OpenApi {
    TotpApi::openapi()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rfc_totp_vector() {
        assert_eq!(
            hotp("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", 1).unwrap(),
            "287082"
        );
        assert!(verify_code("GEZDGNBVGY3TQOJQGEZDGNBVGY3TQOJQ", "287082", 59).unwrap());
    }

    #[test]
    fn generated_secret_is_base32() {
        let secret = generate_secret();
        assert_eq!(secret.len(), 32);
        assert!(base32_decode(&secret).is_some());
    }

    #[test]
    fn recovery_codes_are_unique_and_hashed() {
        let codes = recovery_codes();
        assert_eq!(codes.len(), RECOVERY_CODE_COUNT);
        assert_ne!(codes[0], codes[1]);
        assert_ne!(codes[0], hash_recovery_code(&codes[0]));
    }
}
