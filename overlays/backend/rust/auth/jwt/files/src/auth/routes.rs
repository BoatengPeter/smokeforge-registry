use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use serde::Deserialize;
use utoipa::OpenApi;

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

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RegisterRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct RefreshRequest {
    pub refresh_token: String,
}

#[utoipa::path(post, path = "/api/v1/auth/register", request_body = RegisterRequest, responses((status = 201, body = TokenResponse), (status = 409, body = crate::error::ProblemDetails)))]
async fn register(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<TokenResponse>), AppError> {
    let tokens = service.register(input.email, input.password).await?;

    Ok((StatusCode::CREATED, Json(tokens)))
}

#[utoipa::path(post, path = "/api/v1/auth/login", request_body = LoginRequest, responses((status = 200, body = TokenResponse), (status = 401, body = crate::error::ProblemDetails)))]
async fn login(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<LoginRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(service.login(input.email, input.password).await?))
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", request_body = RefreshRequest, responses((status = 200, body = TokenResponse), (status = 401, body = crate::error::ProblemDetails)))]
async fn refresh(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(service.refresh(input.refresh_token).await?))
}

#[utoipa::path(post, path = "/api/v1/auth/logout", request_body = RefreshRequest, responses((status = 204), (status = 401, body = crate::error::ProblemDetails)))]
async fn logout(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    service.logout(input.refresh_token).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/auth/me", security(("bearerAuth" = [])), responses((status = 200, body = UserResponse), (status = 401, body = crate::error::ProblemDetails)))]
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

#[derive(utoipa::OpenApi)]
#[openapi(
    paths(register, login, refresh, logout, me),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            RefreshRequest,
            TokenResponse,
            UserResponse,
            crate::error::ProblemDetails
        )
    ),
    modifiers(&SecurityAddon)
)]
pub struct AuthApi;

struct SecurityAddon;

impl utoipa::Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "bearerAuth",
                utoipa::openapi::security::SecurityScheme::Http(
                    utoipa::openapi::security::Http::new(
                        utoipa::openapi::security::HttpAuthScheme::Bearer,
                    ),
                ),
            );
        }
    }
}

pub fn document() -> utoipa::openapi::OpenApi {
    AuthApi::openapi()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn auth_document_has_paths() {
        let document = document();

        assert!(document.paths.paths.contains_key("/api/v1/auth/login"));
        assert!(document.paths.paths.contains_key("/api/v1/auth/me"));
    }
}
