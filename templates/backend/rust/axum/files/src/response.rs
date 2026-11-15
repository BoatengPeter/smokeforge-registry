//! HTTP response conventions shared by every route.
//!
//! Successful responses use the helpers in this module. Errors use RFC 9457
//! problem details with stable Go-parity extensions (`code`, `request_id`,
//! `instance`, and field errors).

use axum::{
    Json,
    body::{Body, to_bytes},
    extract::Request,
    http::{HeaderValue, StatusCode, header},
    middleware::Next,
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use serde_json::Value;

/// Content type for RFC 9457 problem responses.
pub const PROBLEM_CONTENT_TYPE: &str = "application/problem+json; charset=utf-8";

const MAX_PROBLEM_BYTES: usize = 64 * 1024;

/// One field-level validation failure.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct FieldError {
    pub field: String,
    pub message: String,
}

impl FieldError {
    /// Creates a field error.
    #[must_use]
    pub fn new(field: impl Into<String>, message: impl Into<String>) -> Self {
        Self {
            field: field.into(),
            message: message.into(),
        }
    }
}

/// RFC 9457 problem details with stable smokeforge extensions.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, utoipa::ToSchema)]
pub struct ProblemDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub title: String,
    pub status: u16,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instance: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub errors: Vec<FieldError>,
}

impl ProblemDetails {
    /// Creates a problem response without request context.
    #[must_use]
    pub fn new(
        status: StatusCode,
        code: impl Into<String>,
        title: impl Into<String>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            kind: "about:blank".to_owned(),
            title: title.into(),
            status: status.as_u16(),
            detail: Some(detail.into()),
            instance: None,
            code: Some(code.into()),
            request_id: None,
            errors: Vec::new(),
        }
    }

    /// Adds the request path.
    #[must_use]
    pub fn with_instance(mut self, instance: impl Into<String>) -> Self {
        self.instance = Some(instance.into());
        self
    }

    /// Adds the request ID.
    #[must_use]
    pub fn with_request_id(mut self, request_id: impl Into<String>) -> Self {
        self.request_id = Some(request_id.into());
        self
    }

    /// Adds field-level validation errors.
    #[must_use]
    pub fn with_errors(mut self, errors: Vec<FieldError>) -> Self {
        self.errors = errors;
        self
    }
}

/// Returns a typed JSON success response.
pub fn ok<T: Serialize>(body: T) -> Json<T> {
    Json(body)
}

/// Returns a typed JSON response with an explicit status.
pub fn json<T: Serialize>(status: StatusCode, body: T) -> (StatusCode, Json<T>) {
    (status, Json(body))
}

/// Returns a `201 Created` JSON response.
pub fn created<T: Serialize>(body: T) -> (StatusCode, Json<T>) {
    json(StatusCode::CREATED, body)
}

/// Returns a `202 Accepted` JSON response.
pub fn accepted<T: Serialize>(body: T) -> (StatusCode, Json<T>) {
    json(StatusCode::ACCEPTED, body)
}

/// Returns a `204 No Content` response.
#[must_use]
pub fn no_content() -> StatusCode {
    StatusCode::NO_CONTENT
}

/// Converts problem details into an `application/problem+json` response.
#[must_use]
pub fn problem_response(problem: ProblemDetails) -> Response {
    let status = StatusCode::from_u16(problem.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut response = Json(problem).into_response();

    *response.status_mut() = status;
    response.headers_mut().insert(
        header::CONTENT_TYPE,
        HeaderValue::from_static(PROBLEM_CONTENT_TYPE),
    );

    response
}

/// Adds request path and ID to any RFC 9457 response produced downstream.
pub async fn finalize_problem(request: Request, next: Next) -> Response {
    let instance = request.uri().path().to_owned();
    let request_id = request
        .headers()
        .get("x-request-id")
        .and_then(|value| value.to_str().ok())
        .map(ToOwned::to_owned);

    let response = next.run(request).await;
    if !is_problem_response(&response) {
        return response;
    }

    let status = response.status();
    let (mut parts, body) = response.into_parts();
    let bytes = match to_bytes(body, MAX_PROBLEM_BYTES).await {
        Ok(bytes) => bytes,
        Err(_) => {
            let mut problem = ProblemDetails::new(
                StatusCode::INTERNAL_SERVER_ERROR,
                "response_read_failed",
                "Internal server error",
                "The response could not be finalized.",
            )
            .with_instance(instance.clone());

            if let Some(request_id) = request_id.clone() {
                problem = problem.with_request_id(request_id);
            }

            return problem_response(problem);
        }
    };

    let Ok(mut value) = serde_json::from_slice::<Value>(&bytes) else {
        return Response::from_parts(parts, Body::from(bytes));
    };

    if let Some(object) = value.as_object_mut() {
        object
            .entry("instance")
            .or_insert_with(|| Value::String(instance));

        if let Some(request_id) = request_id {
            object
                .entry("request_id")
                .or_insert_with(|| Value::String(request_id));
        }
    }

    let Ok(body) = serde_json::to_vec(&value) else {
        return Response::from_parts(parts, Body::from(bytes));
    };

    parts.status = status;
    parts.headers.remove(header::CONTENT_LENGTH);

    Response::from_parts(parts, Body::from(body))
}

fn is_problem_response(response: &Response) -> bool {
    response
        .headers()
        .get(header::CONTENT_TYPE)
        .and_then(|value| value.to_str().ok())
        .is_some_and(|value| value.starts_with("application/problem+json"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn problem_details_serialize_extensions() {
        let problem = ProblemDetails::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            "Validation failed",
            "One or more fields are invalid.",
        )
        .with_instance("/api/v1/items")
        .with_request_id("request-1")
        .with_errors(vec![FieldError::new("name", "Required.")]);

        let value = serde_json::to_value(problem).expect("serialize problem");

        assert_eq!(value["type"], "about:blank");
        assert_eq!(value["instance"], "/api/v1/items");
        assert_eq!(value["request_id"], "request-1");
        assert_eq!(value["errors"][0]["field"], "name");
    }

    #[test]
    fn success_helpers_use_expected_statuses() {
        assert_eq!(json(StatusCode::OK, "body").0, StatusCode::OK);
        assert_eq!(created("body").0, StatusCode::CREATED);
        assert_eq!(accepted("body").0, StatusCode::ACCEPTED);
        assert_eq!(no_content(), StatusCode::NO_CONTENT);
    }
}
