//! TOTP enrollment and verification.

use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    routing::post,
};
use hmac::{Hmac, KeyInit, Mac};
use serde::{Deserialize, Serialize};
use sha1::Sha1;
use uuid::Uuid;

use super::AuthService;
use crate::error::AppError;

const DIGITS: u32 = 6;
const PERIOD: u64 = 30;

pub fn router() -> Router {
    Router::new()
        .route("/api/v1/auth/totp/setup", post(setup))
        .route("/api/v1/auth/totp/enable", post(enable))
        .route("/api/v1/auth/totp/verify", post(verify))
}

#[derive(Debug, Serialize)]
pub struct SetupResponse {
    pub secret: String,
    pub digits: u32,
    pub period: u64,
}

#[derive(Debug, Deserialize)]
pub struct CodeRequest {
    pub code: String,
}

async fn setup(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
) -> Result<Json<SetupResponse>, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let secret = generate_secret();
    let pool = service.pool();

    sqlx::query(
        "INSERT INTO user_totp (user_id, secret, enabled)
         VALUES ($1, $2, false)
         ON CONFLICT (user_id) DO UPDATE SET secret = EXCLUDED.secret, enabled = false",
    )
    .bind(user.id)
    .bind(&secret)
    .execute(pool)
    .await
    .map_err(|_| AppError::internal("could not store TOTP secret"))?;

    Ok(Json(SetupResponse {
        secret,
        digits: DIGITS,
        period: PERIOD,
    }))
}

async fn enable(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(input): Json<CodeRequest>,
) -> Result<StatusCode, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let pool = service.pool();

    let secret: Option<String> =
        sqlx::query_scalar("SELECT secret FROM user_totp WHERE user_id = $1 AND enabled = false")
            .bind(user.id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::internal("could not load TOTP secret"))?;

    let secret = secret.ok_or_else(|| {
        AppError::new(
            StatusCode::CONFLICT,
            "totp_not_pending",
            "TOTP not pending",
            "Start TOTP setup before enabling it.",
        )
    })?;

    if !verify_code(&secret, &input.code, unix_time())? {
        return Err(AppError::unauthorized("Invalid TOTP code."));
    }

    sqlx::query("UPDATE user_totp SET enabled = true WHERE user_id = $1")
        .bind(user.id)
        .execute(pool)
        .await
        .map_err(|_| AppError::internal("could not enable TOTP"))?;

    Ok(StatusCode::NO_CONTENT)
}

async fn verify(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Json(input): Json<CodeRequest>,
) -> Result<StatusCode, AppError> {
    let user = service.current_user(bearer(&headers)?).await?;
    let pool = service.pool();

    let secret: Option<String> =
        sqlx::query_scalar("SELECT secret FROM user_totp WHERE user_id = $1 AND enabled = true")
            .bind(user.id)
            .fetch_optional(pool)
            .await
            .map_err(|_| AppError::internal("could not load TOTP secret"))?;

    let secret = secret.ok_or_else(|| AppError::unauthorized("TOTP is not enabled."))?;

    if !verify_code(&secret, &input.code, unix_time())? {
        return Err(AppError::unauthorized("Invalid TOTP code."));
    }

    Ok(StatusCode::NO_CONTENT)
}

fn bearer(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))
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
}
