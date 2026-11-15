use axum::{Json, extract::State, http::StatusCode};
use serde::Serialize;
use utoipa::ToSchema;

use crate::{error::AppError, state::AppState};

/// Liveness response.
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub status: String,
    pub service: String,
}

/// Liveness probe.
#[utoipa::path(get, path = "/healthz", responses((status = 200, body = HealthResponse)))]
pub async fn health_check(State(state): State<AppState>) -> Json<HealthResponse> {
    Json(HealthResponse {
        status: "ok".to_owned(),
        service: state.service_name().to_owned(),
    })
}

/// Readiness probe.
#[utoipa::path(
    get,
    path = "/readyz",
    responses(
        (status = 200, body = HealthResponse),
        (status = 503, body = crate::response::ProblemDetails)
    )
)]
pub async fn ready_check(State(state): State<AppState>) -> Result<Json<HealthResponse>, AppError> {
    state.ready().await?;

    Ok(Json(HealthResponse {
        status: "ready".to_owned(),
        service: state.service_name().to_owned(),
    }))
}

/// Returns the HTTP status used by readiness failures.
pub const NOT_READY: StatusCode = StatusCode::SERVICE_UNAVAILABLE;
