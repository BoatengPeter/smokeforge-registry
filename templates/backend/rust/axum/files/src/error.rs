use axum::http::StatusCode;
use thiserror::Error;

use crate::response::{self, FieldError};

/// Application error that converts into an RFC 9457 response.
#[derive(Debug, Error)]
#[error("{title}: {detail}")]
#[must_use]
pub struct AppError {
    status: StatusCode,
    code: &'static str,
    title: &'static str,
    detail: String,
    fields: Vec<FieldError>,
}

impl AppError {
    /// Creates a typed application error.
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
            fields: Vec::new(),
        }
    }

    /// Creates a 422 validation error with field-level details.
    pub fn validation(fields: Vec<FieldError>) -> Self {
        Self {
            status: StatusCode::UNPROCESSABLE_ENTITY,
            code: "validation_failed",
            title: "Validation failed",
            detail: "One or more fields are invalid.".to_owned(),
            fields,
        }
    }

    /// Creates a 401 unauthorized error.
    pub fn unauthorized(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::UNAUTHORIZED,
            "unauthorized",
            "Unauthorized",
            detail,
        )
    }

    /// Creates a 404 not-found error.
    pub fn not_found(detail: impl Into<String>) -> Self {
        Self::new(StatusCode::NOT_FOUND, "not_found", "Not found", detail)
    }

    /// Creates a 500 internal error.
    pub fn internal(detail: impl Into<String>) -> Self {
        Self::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_error",
            "Internal server error",
            detail,
        )
    }
}

impl axum::response::IntoResponse for AppError {
    fn into_response(self) -> axum::response::Response {
        let mut problem =
            response::ProblemDetails::new(self.status, self.code, self.title, self.detail);

        if !self.fields.is_empty() {
            problem = problem.with_errors(self.fields);
        }

        response::problem_response(problem)
    }
}
