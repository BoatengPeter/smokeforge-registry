use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use thiserror::Error;

/// RFC 9457 problem details returned by the API.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
}

/// Application error that converts into an RFC 9457 response.
#[derive(Debug, Error)]
#[error("{title}: {detail}")]
pub struct AppError {
    status: StatusCode,
    code: &'static str,
    title: &'static str,
    detail: String,
}

impl AppError {
    pub fn new(
        status: StatusCode,
        code: &'static str,
        title: &'static str,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            status,
            code,
            title,
            detail: detail.into(),
        }
    }

    pub fn unauthorized(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Unauthorized",
            detail,
        )
    }

    pub fn not_found(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", "Not found", detail)
    }

    pub fn internal(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "Internal server error",
            detail,
        )
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let body = ProblemDetails {
            kind: "about:blank".to_owned(),
            title: self.title.to_owned(),
            status: self.status.as_u16(),
            detail: Some(self.detail),
            code: Some(self.code.to_owned()),
        };

        (self.status, Json(body)).into_response()
    }
}
