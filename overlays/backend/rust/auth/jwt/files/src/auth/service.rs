use std::sync::{Arc, RwLock};

use async_trait::async_trait;
use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::{
    password, repository,
    token::{access_token, hash_refresh, mfa_token, refresh_token, verify_access, verify_mfa},
    user::{User, UserResponse},
};
use crate::{config::Config, error::AppError};

/// A configured second factor used during password login.
#[async_trait]
pub trait SecondFactor: Send + Sync {
    /// Reports whether the user must complete a second factor.
    async fn required(&self, user_id: Uuid) -> Result<bool, AppError>;

    /// Verifies a TOTP or recovery code for the user.
    async fn verify(&self, user_id: Uuid, code: &str) -> Result<(), AppError>;
}

/// Result of a password login.
pub enum LoginOutcome {
    /// Authentication completed and tokens were issued.
    Tokens(TokenResponse),
    /// A second-factor challenge is required.
    Challenge { challenge: String, expires_in: i64 },
}

const MFA_CHALLENGE_TTL_SECONDS: i64 = 300;

/// Authentication use cases.
pub struct AuthService {
    pool: PgPool,
    jwt_secret: String,
    issuer: String,
    audience: String,
    access_ttl_seconds: i64,
    refresh_ttl_seconds: i64,
    public_base_url: String,
    require_email_verification: bool,
    google_client_id: String,
    google_client_secret: String,
    google_redirect_url: String,
    oauth_success_redirect: String,
    resend_api_key: String,
    email_from: String,
    second_factor: RwLock<Option<Arc<dyn SecondFactor>>>,
}

/// Tokens returned by register, login, and refresh.
#[derive(Debug, serde::Serialize, utoipa::ToSchema)]
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
            public_base_url: config.public_base_url.trim_end_matches('/').to_owned(),
            require_email_verification: config.require_email_verification,
            google_client_id: config.google_client_id.clone(),
            google_client_secret: config.google_client_secret.clone(),
            google_redirect_url: config.google_redirect_url.clone(),
            oauth_success_redirect: config.oauth_success_redirect.clone(),
            resend_api_key: config.resend_api_key.clone(),
            email_from: config.email_from.clone(),
            second_factor: RwLock::new(None),
        }
    }

    /// Adds a second factor to password login.
    pub fn set_second_factor(&self, factor: Arc<dyn SecondFactor>) {
        if let Ok(mut guard) = self.second_factor.write() {
            *guard = Some(factor);
        }
    }

    fn second_factor(&self) -> Option<Arc<dyn SecondFactor>> {
        self.second_factor
            .read()
            .ok()
            .and_then(|guard| guard.clone())
    }

    /// Returns the shared pool for feature modules.
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub async fn register(
        &self,
        email: String,
        password: String,
    ) -> Result<UserResponse, AppError> {
        let email = normalize_email(&email)?;
        validate_password(&password)?;

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

        if self.require_email_verification
            && let Err(error) = self.send_verification(&user).await
        {
            tracing::error!(error = %error, user_id = %user.id, "send verification email");
        }

        Ok(user.into())
    }

    pub async fn login(&self, email: String, password: String) -> Result<LoginOutcome, AppError> {
        let user = repository::find_by_email(&self.pool, &email)
            .await
            .map_err(|_| AppError::internal("could not load account"))?
            .ok_or_else(|| AppError::unauthorized("Invalid email or password."))?;

        if !password::verify(&password, &user.password_hash)? {
            return Err(AppError::unauthorized("Invalid email or password."));
        }

        if self.require_email_verification && user.email_verified_at.is_none() {
            return Err(AppError::new(
                axum::http::StatusCode::FORBIDDEN,
                "email_not_verified",
                "Email not verified",
                "Verify your email address before signing in.",
            ));
        }

        if let Some(factor) = self.second_factor()
            && factor.required(user.id).await?
        {
            let challenge = mfa_token(
                user.id,
                &self.jwt_secret,
                &self.issuer,
                &self.audience,
                MFA_CHALLENGE_TTL_SECONDS,
            )?;

            return Ok(LoginOutcome::Challenge {
                challenge,
                expires_in: MFA_CHALLENGE_TTL_SECONDS,
            });
        }

        self.issue_tokens(&user).await.map(LoginOutcome::Tokens)
    }

    /// Completes a second-factor challenge.
    pub async fn complete_mfa(
        &self,
        challenge: String,
        code: String,
    ) -> Result<TokenResponse, AppError> {
        let user_id = verify_mfa(&challenge, &self.jwt_secret, &self.issuer, &self.audience)?;

        let factor = self
            .second_factor()
            .ok_or_else(|| AppError::unauthorized("Second factor is not configured."))?;

        factor.verify(user_id, &code).await?;

        let user = repository::find_by_id(&self.pool, user_id)
            .await
            .map_err(|_| AppError::internal("could not load account"))?
            .ok_or_else(|| AppError::unauthorized("The account is unavailable."))?;

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

    /// Verifies an email address using a one-time token.
    pub async fn verify_email(&self, raw: String) -> Result<(), AppError> {
        let user_id =
            repository::consume_auth_token(&self.pool, "email_verify", &hash_refresh(&raw))
                .await
                .map_err(|_| AppError::internal("could not verify email"))?
                .ok_or_else(|| {
                    AppError::new(
                        axum::http::StatusCode::BAD_REQUEST,
                        "invalid_token",
                        "Invalid token",
                        "The verification token is invalid or expired.",
                    )
                })?;

        repository::mark_email_verified(&self.pool, user_id)
            .await
            .map_err(|_| AppError::internal("could not verify email"))
    }

    /// Sends a password reset link without revealing whether the account exists.
    pub async fn forgot_password(&self, email: String) {
        let email = match normalize_email(&email) {
            Ok(email) => email,
            Err(_) => return,
        };

        let user = match repository::find_by_email(&self.pool, &email).await {
            Ok(Some(user)) => user,
            Ok(None) => return,
            Err(error) => {
                tracing::error!(error = %error, "look up password reset user");

                return;
            }
        };

        if let Err(error) = self.send_password_reset(&user).await {
            tracing::error!(error = %error, user_id = %user.id, "send password reset email");
        }
    }

    /// Resets a password using a one-time token.
    pub async fn reset_password(&self, raw: String, password: String) -> Result<(), AppError> {
        validate_password(&password)?;

        let user_id =
            repository::consume_auth_token(&self.pool, "password_reset", &hash_refresh(&raw))
                .await
                .map_err(|_| AppError::internal("could not reset password"))?
                .ok_or_else(|| {
                    AppError::new(
                        axum::http::StatusCode::BAD_REQUEST,
                        "invalid_token",
                        "Invalid token",
                        "The reset token is invalid or expired.",
                    )
                })?;

        let hash = password::hash(&password)?;
        repository::update_password(&self.pool, user_id, &hash)
            .await
            .map_err(|_| AppError::internal("could not reset password"))?;
        repository::revoke_all_refresh(&self.pool, user_id)
            .await
            .map_err(|_| AppError::internal("could not revoke sessions"))
    }

    /// Resolves a Google identity and issues tokens.
    pub async fn login_external(
        &self,
        email: String,
        provider_id: String,
    ) -> Result<TokenResponse, AppError> {
        let email = normalize_email(&email)?;
        let user_id = repository::find_oauth_user(&self.pool, "google", &provider_id)
            .await
            .map_err(|_| AppError::internal("could not load OAuth account"))?;

        let user = if let Some(user_id) = user_id {
            repository::find_by_id(&self.pool, user_id)
                .await
                .map_err(|_| AppError::internal("could not load OAuth user"))?
                .ok_or_else(|| AppError::unauthorized("The OAuth account is unavailable."))?
        } else {
            let user = match repository::find_by_email(&self.pool, &email)
                .await
                .map_err(|_| AppError::internal("could not load OAuth user"))?
            {
                Some(user) => user,
                None => {
                    let random = Uuid::new_v4().to_string();
                    let hash = password::hash(&random)?;
                    repository::insert_user(&self.pool, &email, &hash)
                        .await
                        .map_err(|_| AppError::internal("could not create OAuth user"))?
                }
            };

            repository::link_oauth(&self.pool, user.id, "google", &provider_id, &email)
                .await
                .map_err(|_| AppError::internal("could not link OAuth account"))?;
            user
        };

        self.issue_tokens(&user).await
    }

    /// Returns whether Google OAuth is configured.
    pub fn google_configured(&self) -> bool {
        !self.google_client_id.is_empty() && !self.google_client_secret.is_empty()
    }

    /// Builds the Google authorization URL and a signed state value.
    pub fn google_auth_url(&self) -> Result<String, AppError> {
        if !self.google_configured() {
            return Err(AppError::new(
                axum::http::StatusCode::SERVICE_UNAVAILABLE,
                "oauth_unavailable",
                "OAuth unavailable",
                "Google OAuth is not configured.",
            ));
        }

        let state = mfa_token(
            Uuid::new_v4(),
            &self.jwt_secret,
            &self.issuer,
            &self.audience,
            600,
        )?;
        let redirect = if self.google_redirect_url.is_empty() {
            format!("{}/api/v1/auth/oauth/google/callback", self.public_base_url)
        } else {
            self.google_redirect_url.clone()
        };
        let mut url = url::Url::parse("https://accounts.google.com/o/oauth2/v2/auth")
            .map_err(|_| AppError::internal("could not build OAuth URL"))?;
        url.query_pairs_mut()
            .append_pair("client_id", &self.google_client_id)
            .append_pair("redirect_uri", &redirect)
            .append_pair("response_type", "code")
            .append_pair("scope", "openid email profile")
            .append_pair("state", &state);

        Ok(url.into())
    }

    /// Completes Google OAuth and issues tokens.
    pub async fn complete_google(
        &self,
        code: String,
        state: String,
    ) -> Result<TokenResponse, AppError> {
        verify_mfa(&state, &self.jwt_secret, &self.issuer, &self.audience)?;

        let redirect = if self.google_redirect_url.is_empty() {
            format!("{}/api/v1/auth/oauth/google/callback", self.public_base_url)
        } else {
            self.google_redirect_url.clone()
        };
        let client = reqwest::Client::new();
        let token: serde_json::Value = client
            .post("https://oauth2.googleapis.com/token")
            .form(&[
                ("code", code),
                ("client_id", self.google_client_id.clone()),
                ("client_secret", self.google_client_secret.clone()),
                ("redirect_uri", redirect),
                ("grant_type", "authorization_code".to_owned()),
            ])
            .send()
            .await
            .map_err(|_| {
                AppError::new(
                    axum::http::StatusCode::BAD_GATEWAY,
                    "oauth_exchange_failed",
                    "OAuth exchange failed",
                    "Google OAuth could not be completed.",
                )
            })?
            .json()
            .await
            .map_err(|_| {
                AppError::new(
                    axum::http::StatusCode::BAD_GATEWAY,
                    "oauth_exchange_failed",
                    "OAuth exchange failed",
                    "Google OAuth returned an invalid response.",
                )
            })?;

        let access_token = token
            .get("access_token")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| {
                AppError::unauthorized("Google OAuth did not return an access token.")
            })?;

        let profile: GoogleProfile = client
            .get("https://www.googleapis.com/oauth2/v3/userinfo")
            .bearer_auth(access_token)
            .send()
            .await
            .map_err(|_| {
                AppError::new(
                    axum::http::StatusCode::BAD_GATEWAY,
                    "oauth_profile_failed",
                    "OAuth profile failed",
                    "Google user information could not be loaded.",
                )
            })?
            .json()
            .await
            .map_err(|_| {
                AppError::new(
                    axum::http::StatusCode::BAD_GATEWAY,
                    "oauth_profile_failed",
                    "OAuth profile failed",
                    "Google user information was invalid.",
                )
            })?;

        if !profile.email_verified {
            return Err(AppError::new(
                axum::http::StatusCode::FORBIDDEN,
                "email_not_verified",
                "Email not verified",
                "The Google account email is not verified.",
            ));
        }

        self.login_external(profile.email, profile.sub).await
    }

    /// Returns the configured post-OAuth redirect.
    pub fn oauth_success_redirect(&self) -> String {
        if self.oauth_success_redirect.is_empty() {
            self.public_base_url.clone()
        } else {
            self.oauth_success_redirect.clone()
        }
    }

    async fn send_verification(&self, user: &User) -> Result<(), AppError> {
        let raw = Uuid::new_v4().to_string();
        repository::store_auth_token(
            &self.pool,
            user.id,
            "email_verify",
            &hash_refresh(&raw),
            OffsetDateTime::now_utc() + time::Duration::hours(24),
        )
        .await
        .map_err(|_| AppError::internal("could not create verification token"))?;

        self.send_email(
            &user.email,
            "Verify your email address",
            &format!(
                "Verify your email address by opening this link:\n\n{}/verify-email?token={raw}",
                self.public_base_url
            ),
        )
        .await
    }

    async fn send_password_reset(&self, user: &User) -> Result<(), AppError> {
        let raw = Uuid::new_v4().to_string();
        repository::store_auth_token(
            &self.pool,
            user.id,
            "password_reset",
            &hash_refresh(&raw),
            OffsetDateTime::now_utc() + time::Duration::hours(1),
        )
        .await
        .map_err(|_| AppError::internal("could not create reset token"))?;

        self.send_email(
            &user.email,
            "Reset your password",
            &format!(
                "Reset your password by opening this link:\n\n{}/reset-password?token={raw}",
                self.public_base_url
            ),
        )
        .await
    }

    async fn send_email(&self, to: &str, subject: &str, body: &str) -> Result<(), AppError> {
        if self.resend_api_key.trim().is_empty() || self.email_from.trim().is_empty() {
            tracing::info!(to, subject, body, "development email");

            return Ok(());
        }

        let response = reqwest::Client::new()
            .post("https://api.resend.com/emails")
            .bearer_auth(&self.resend_api_key)
            .json(&serde_json::json!({
                "from": self.email_from,
                "to": [to],
                "subject": subject,
                "text": body,
            }))
            .send()
            .await
            .map_err(|_| AppError::internal("could not send email"))?;

        if !response.status().is_success() {
            return Err(AppError::internal("email provider rejected the message"));
        }

        Ok(())
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

fn normalize_email(value: &str) -> Result<String, AppError> {
    let email = value.trim().to_ascii_lowercase();

    if email.len() > 320 || !email.contains('@') {
        return Err(AppError::new(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            "A valid email address is required.",
        ));
    }

    Ok(email)
}

fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 12 || password.len() > 256 {
        return Err(AppError::new(
            axum::http::StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            "Password must contain 12 to 256 characters.",
        ));
    }

    Ok(())
}

#[derive(Debug, serde::Deserialize)]
struct GoogleProfile {
    sub: String,
    email: String,
    #[serde(default)]
    email_verified: bool,
}
