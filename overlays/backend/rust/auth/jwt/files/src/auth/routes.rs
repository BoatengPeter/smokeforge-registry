use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Query,
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Redirect, Response},
    routing::{get, post},
};
use serde::Deserialize;
use utoipa::OpenApi;

use super::{
    service::{AuthService, LoginOutcome, TokenResponse},
    user::UserResponse,
};
use crate::error::AppError;

/// Builds the auth router.
pub fn router(service: Arc<AuthService>) -> Router {
    let router = Router::new()
        .route("/api/v1/auth/register", post(register))
        .route("/api/v1/auth/login", post(login))
        .route("/api/v1/auth/login/totp", post(login_totp))
        .route("/api/v1/auth/refresh", post(refresh))
        .route("/api/v1/auth/logout", post(logout))
        .route("/api/v1/auth/verify-email", post(verify_email))
        .route("/api/v1/auth/forgot-password", post(forgot_password))
        .route("/api/v1/auth/reset-password", post(reset_password))
        .route("/api/v1/auth/oauth/google", get(google_start))
        .route("/api/v1/auth/oauth/google/callback", get(google_callback))
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

#[utoipa::path(post, path = "/api/v1/auth/register", request_body = RegisterRequest, responses((status = 201, body = UserResponse), (status = 409, body = crate::response::ProblemDetails)))]
async fn register(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<UserResponse>), AppError> {
    let user = service.register(input.email, input.password).await?;

    Ok((StatusCode::CREATED, Json(user)))
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login",
    request_body = LoginRequest,
    responses(
        (status = 200, body = TokenResponse),
        (status = 202, body = MfaChallenge),
        (status = 401, body = crate::response::ProblemDetails)
    )
)]
async fn login(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<LoginRequest>,
) -> Result<Response, AppError> {
    match service.login(input.email, input.password).await? {
        LoginOutcome::Tokens(tokens) => Ok((StatusCode::OK, Json(tokens)).into_response()),
        LoginOutcome::Challenge {
            challenge,
            expires_in,
        } => Ok((
            StatusCode::ACCEPTED,
            Json(MfaChallenge {
                challenge,
                expires_in,
            }),
        )
            .into_response()),
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema)]
pub struct MfaChallenge {
    pub challenge: String,
    pub expires_in: i64,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct CompleteMfaRequest {
    pub challenge: String,
    pub code: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/login/totp",
    request_body = CompleteMfaRequest,
    responses(
        (status = 200, body = TokenResponse),
        (status = 401, body = crate::response::ProblemDetails)
    )
)]
async fn login_totp(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<CompleteMfaRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(
        service.complete_mfa(input.challenge, input.code).await?,
    ))
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct TokenRequest {
    pub token: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ForgotPasswordRequest {
    pub email: String,
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct ResetPasswordRequest {
    pub token: String,
    pub password: String,
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/verify-email",
    request_body = TokenRequest,
    responses(
        (status = 204),
        (status = 400, body = crate::response::ProblemDetails)
    )
)]
async fn verify_email(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<TokenRequest>,
) -> Result<StatusCode, AppError> {
    service.verify_email(input.token).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/forgot-password",
    request_body = ForgotPasswordRequest,
    responses((status = 202), (status = 422, body = crate::response::ProblemDetails))
)]
async fn forgot_password(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<ForgotPasswordRequest>,
) -> Result<StatusCode, AppError> {
    service.forgot_password(input.email).await;

    Ok(StatusCode::ACCEPTED)
}

#[utoipa::path(
    post,
    path = "/api/v1/auth/reset-password",
    request_body = ResetPasswordRequest,
    responses(
        (status = 204),
        (status = 400, body = crate::response::ProblemDetails),
        (status = 422, body = crate::response::ProblemDetails)
    )
)]
async fn reset_password(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<ResetPasswordRequest>,
) -> Result<StatusCode, AppError> {
    service.reset_password(input.token, input.password).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[derive(Debug, Deserialize, utoipa::ToSchema)]
pub struct GoogleCallbackQuery {
    pub code: String,
    pub state: String,
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/oauth/google",
    responses((status = 307), (status = 503, body = crate::response::ProblemDetails))
)]
async fn google_start(
    Extension(service): Extension<Arc<AuthService>>,
) -> Result<Redirect, AppError> {
    Ok(Redirect::temporary(&service.google_auth_url()?))
}

#[utoipa::path(
    get,
    path = "/api/v1/auth/oauth/google/callback",
    params(
        ("code" = String, Query, description = "Google authorization code"),
        ("state" = String, Query, description = "Signed OAuth state")
    ),
    responses(
        (status = 200, body = TokenResponse),
        (status = 502, body = crate::response::ProblemDetails)
    )
)]
async fn google_callback(
    Extension(service): Extension<Arc<AuthService>>,
    Query(query): Query<GoogleCallbackQuery>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(
        service.complete_google(query.code, query.state).await?,
    ))
}

#[utoipa::path(post, path = "/api/v1/auth/refresh", request_body = RefreshRequest, responses((status = 200, body = TokenResponse), (status = 401, body = crate::response::ProblemDetails)))]
async fn refresh(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<Json<TokenResponse>, AppError> {
    Ok(Json(service.refresh(input.refresh_token).await?))
}

#[utoipa::path(post, path = "/api/v1/auth/logout", request_body = RefreshRequest, responses((status = 204), (status = 401, body = crate::response::ProblemDetails)))]
async fn logout(
    Extension(service): Extension<Arc<AuthService>>,
    Json(input): Json<RefreshRequest>,
) -> Result<StatusCode, AppError> {
    service.logout(input.refresh_token).await?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(get, path = "/api/v1/auth/me", security(("bearerAuth" = [])), responses((status = 200, body = UserResponse), (status = 401, body = crate::response::ProblemDetails)))]
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
    paths(
        register, login, login_totp, refresh, logout, me,
        verify_email, forgot_password, reset_password, google_start, google_callback
    ),
    components(
        schemas(
            RegisterRequest,
            LoginRequest,
            CompleteMfaRequest,
            MfaChallenge,
            RefreshRequest,
            TokenRequest,
            ForgotPasswordRequest,
            ResetPasswordRequest,
            GoogleCallbackQuery,
            TokenResponse,
            UserResponse,
            crate::response::ProblemDetails
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

#[allow(unused_mut)] // feature overlays merge additional documents
pub fn document() -> utoipa::openapi::OpenApi {
    let mut document = AuthApi::openapi();
    // smokeforge:anchor auth-openapi-extra
    document
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
