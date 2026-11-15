use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use serde::Deserialize;

use super::{
    service::{AuthService, TokenResponse},
    user::UserResponse,
};
use crate::error::AppError;

/// Builds the auth router.
pub fn router(service: Arc<AuthService>) -> Router {
    let router = Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/refresh", post(refresh))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/me", get(me));

    // smokeforge:anchor auth-totp-routes
    router.layer(Extension(service))
}

#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

async fn register(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), AppError> {
    let tokens = service.register(input.email, input.password).await?;

    Ok((StatusCode::CREATED, Json(tokens)))
}

async fn login(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(service.login(input.email, input.password).await?))
}

async fn refresh(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(service.refresh(input.refresh_token).await?))
}

async fn logout(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    service.logout(input.refresh_token).await?;

    Ok(StatusCode::NO_CONTENT)
}

async fn me(
    Extension(service): Extension<Arc<AuthService>>,
    headers: HeaderMap,
) -> Result<Json<UserResponse>, AppError> {
    let token = bearer(&headers)?;

    Ok(Json(service.current_user(token).await?))
}

fn bearer(headers: &HeaderMap) -> Result<&str, AppError> {
    let value = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))?;

    value
        .strip_prefix("Bearer ")
        .filter(|token| !token.is_empty())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))
}
