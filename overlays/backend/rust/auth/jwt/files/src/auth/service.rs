use sqlx::PgPool;
use time::OffsetDateTime;

use super::{
    password, repository,
    token::{access_token, refresh_token, verify_access},
    user::{User, UserResponse},
};
use crate::{config::Config, error::AppError};

/// Authentication use cases.
pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
    issuer: String,
    audience: String,
    access_ttl_seconds: i64,
    refresh_ttl_seconds: i64,
}

/// Tokens returned by register, login, and refresh.
#[derive(Debug, serde::Serialize)]
pub struct TokenResponse {
    pub access_token: String,
    pub refresh_token: String,
    pub token_type: &'static str,
    pub expires_in: i64,
}

impl AuthService {
    pub fn new(pool: PgPool, config: &Config) -> Self {
        Self {
            pool,
            jwt_secret: config.jwt_secret.clone(),
            issuer: config.jwt_issuer.clone(),
            audience: config.jwt_audience.clone(),
            access_ttl_seconds: config.access_token_ttl_seconds,
            refresh_ttl_seconds: config.refresh_token_ttl_seconds,
        }
    }

    /// Returns the shared pool for feature modules.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
    ) -> Result<TokenResponse, AppError> {
        validate_credentials(&email, &password)?;

        let hash = password::hash(&password)?;
        let user = repository::insert_user(&self.pool, &email, &hash)
            .await
            .map_err(|error| {
                if error
                    .as_database_error()
                    .is_some_and(|e| e.is_unique_violation())
                {
                    AppError::new(
                        axum::http::StatusCode::CONFLICT,
                        "email_taken",
                        "Email already registered",
                        "An account with that email already exists.",
                    )
                } else {
                    AppError::internal("could not create account")
                }
            })?;

        self.issue_tokens(&user).await
    }

    pub async fn login(&self, email: String, password: String) -> Result<TokenResponse, AppError> {
        let user = repository::find_by_email(&self.pool, &email)
            .await
            .map_err(|_| AppError::internal("could not load account"))?
            .ok_or_else(|| AppError::unauthorized("Invalid email or password."))?;

        if !password::verify(&password, &user.password_hash)? {
            return Err(AppError::unauthorized("Invalid email or password."));
        }

        self.issue_tokens(&user).await
    }

    pub async fn refresh(&self, raw: String) -> Result<TokenResponse, AppError> {
        let token_hash = super::token::hash_refresh(&raw);
        let user_id = repository::consume_refresh(&self.pool, &token_hash)
            .await
            .map_err(|_| AppError::internal("could not rotate refresh token"))?
            .ok_or_else(|| AppError::unauthorized("The refresh token is invalid or expired."))?;

        let user = repository::find_by_id(&self.pool, user_id)
            .await
            .map_err(|_| AppError::internal("could not load account"))?
            .ok_or_else(|| AppError::unauthorized("The account is unavailable."))?;

        self.issue_tokens(&user).await
    }

    pub async fn logout(&self, raw: String) -> Result<(), AppError> {
        repository::revoke_refresh(&self.pool, &super::token::hash_refresh(&raw))
            .await
            .map_err(|_| AppError::internal("could not revoke refresh token"))?;

        Ok(())
    }

    pub async fn current_user(&self, access_token: &str) -> Result<UserResponse, AppError> {
        let claims = verify_access(access_token, &self.jwt_secret, &self.issuer, &self.audience)?;

        let user = repository::find_by_id(&self.pool, claims.sub)
            .await
            .map_err(|_| AppError::internal("could not load account"))?
            .ok_or_else(|| AppError::unauthorized("The account is unavailable."))?;

        Ok(user.into())
    }

    async fn issue_tokens(&self, user: &User) -> Result<TokenResponse, AppError> {
        let access = access_token(
            user.id,
            &user.role,
            &self.jwt_secret,
            &self.issuer,
            &self.audience,
            self.access_ttl_seconds,
        )?;

        let (raw_refresh, refresh_hash) = refresh_token();
        let expires_at =
            OffsetDateTime::now_utc() + time::Duration::seconds(self.refresh_ttl_seconds);

        repository::store_refresh(&self.pool, user.id, &refresh_hash, expires_at)
            .await
            .map_err(|_| AppError::internal("could not store refresh token"))?;

        Ok(TokenResponse {
            access_token: access,
            refresh_token: raw_refresh,
            token_type: "Bearer",
            expires_in: self.access_ttl_seconds,
        })
    }
}

fn validate_credentials(email: &str, password: &str) -> Result<(), AppError> {
    if !email.contains('@') || email.len() > 320 {
        return Err(AppError::new(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            "A valid email address is required.",
        ));
    }

    if password.len() < 8 || password.len() > 128 {
        return Err(AppError::new(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            "Password must contain 8 to 128 characters.",
        ));
    }

    Ok(())
}
