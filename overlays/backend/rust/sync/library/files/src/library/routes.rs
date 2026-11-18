use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    body::Bytes,
    extract::{Path, Query},
    http::{HeaderMap, StatusCode, header},
    response::{IntoResponse, Response},
    routing::{get, post},
};
use serde::Deserialize;
use sha2::{Digest, Sha256};
use utoipa::OpenApi;
use uuid::Uuid;

use super::service::{Changes, Item, LibraryService, Mutation, Snapshot};
use crate::{auth::AuthService, error::AppError};

#[derive(Debug, Deserialize)]
struct ChangesQuery {
    cursor: Option<i64>,
    limit: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct CreateRequest {
    id: Uuid,
    title: String,
    description: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
struct UpdateRequest {
    title: String,
    description: Option<String>,
    status: Option<String>,
    tags: Option<Vec<String>>,
}

/// Builds the Library router.
pub fn router(pool: sqlx::PgPool, auth: Arc<AuthService>) -> Router {
    Router::new()
        .route("/api/v1/library/snapshot", get(snapshot))
        .route("/api/v1/library/changes", get(changes))
        .route(
            "/api/v1/library/{id}",
            get(get_item).patch(update).delete(delete_item),
        )
        .route("/api/v1/library", post(create))
        .layer(Extension(Arc::new(LibraryService::new(pool))))
        .layer(Extension(auth))
}

async fn snapshot(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
) -> Result<Json<Snapshot>, AppError> {
    let user = current_user(&auth, &headers).await?;
    Ok(Json(service.snapshot(user).await?))
}

async fn changes(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Query(query): Query<ChangesQuery>,
) -> Result<Json<Changes>, AppError> {
    let user = current_user(&auth, &headers).await?;
    Ok(Json(
        service
            .changes(user, query.cursor.unwrap_or(0), query.limit.unwrap_or(200))
            .await?,
    ))
}

async fn get_item(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<Item>, AppError> {
    let user = current_user(&auth, &headers).await?;
    Ok(Json(service.get(user, id).await?))
}

async fn create(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    body: Bytes,
) -> Result<Response, AppError> {
    let user = current_user(&auth, &headers).await?;
    let input: CreateRequest = serde_json::from_slice(&body).map_err(|_| validation())?;
    validate(&input.title, input.status.as_deref(), input.tags.as_deref())?;
    let key = idempotency_key(&headers)?;
    let hash = request_hash("POST", "/api/v1/library", &body);
    let (status, body, replayed) = service
        .create(
            user,
            input.id,
            input.title.trim().to_owned(),
            input.description.unwrap_or_default(),
            input.status.unwrap_or_else(|| "draft".to_owned()),
            input.tags.unwrap_or_default(),
            key,
            hash,
        )
        .await?;
    Ok(mutation_response(status, body, replayed))
}

async fn update(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
    body: Bytes,
) -> Result<Response, AppError> {
    let user = current_user(&auth, &headers).await?;
    let input: UpdateRequest = serde_json::from_slice(&body).map_err(|_| validation())?;
    validate(&input.title, input.status.as_deref(), input.tags.as_deref())?;
    let version = parse_version(&headers)?;
    let key = idempotency_key(&headers)?;
    let hash = request_hash("PATCH", &format!("/api/v1/library/{id}"), &body);
    let (status, body, replayed) = service
        .update(
            user,
            id,
            version,
            input.title.trim().to_owned(),
            input.description.unwrap_or_default(),
            input.status.unwrap_or_else(|| "draft".to_owned()),
            input.tags.unwrap_or_default(),
            key,
            hash,
        )
        .await?;
    Ok(mutation_response(status, body, replayed))
}

async fn delete_item(
    Extension(service): Extension<Arc<LibraryService>>,
    Extension(auth): Extension<Arc<AuthService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Response, AppError> {
    let user = current_user(&auth, &headers).await?;
    let version = parse_version(&headers)?;
    let key = idempotency_key(&headers)?;
    let hash = request_hash("DELETE", &format!("/api/v1/library/{id}"), &[]);
    let (status, body, replayed) = service.delete(user, id, version, key, hash).await?;
    Ok(mutation_response(status, body, replayed))
}

async fn current_user(auth: &AuthService, headers: &HeaderMap) -> Result<Uuid, AppError> {
    let token = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|value| !value.is_empty())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))?;

    Ok(auth.current_user(token).await?.id)
}

fn idempotency_key(headers: &HeaderMap) -> Result<String, AppError> {
    headers
        .get("idempotency-key")
        .and_then(|value| value.to_str().ok())
        .map(str::trim)
        .filter(|value| !value.is_empty())
        .map(str::to_owned)
        .ok_or_else(|| {
            AppError::new(
                StatusCode::BAD_REQUEST,
                "idempotency_key_required",
                "Idempotency key required",
                "Send an Idempotency-Key header.",
            )
        })
}

fn parse_version(headers: &HeaderMap) -> Result<i64, AppError> {
    headers
        .get(header::IF_MATCH)
        .and_then(|value| value.to_str().ok())
        .map(|value| value.trim().trim_matches('"'))
        .and_then(|value| value.parse::<i64>().ok())
        .ok_or_else(|| {
            AppError::new(
                StatusCode::PRECONDITION_REQUIRED,
                "version_required",
                "Version required",
                "Send a valid If-Match version.",
            )
        })
}

fn validation() -> AppError {
    AppError::validation(Vec::new())
}

fn validate(title: &str, status: Option<&str>, tags: Option<&[String]>) -> Result<(), AppError> {
    if title.trim().is_empty() || title.len() > 160 {
        return Err(validation());
    }
    if let Some(status) = status {
        if !matches!(status, "draft" | "active" | "archived") {
            return Err(validation());
        }
    }
    if let Some(tags) = tags {
        if tags.len() > 12
            || tags
                .iter()
                .any(|tag| tag.trim().is_empty() || tag.len() > 32)
        {
            return Err(validation());
        }
    }
    Ok(())
}

fn request_hash(method: &str, path: &str, body: &[u8]) -> String {
    let mut hasher = Sha256::new();
    hasher.update(method);
    hasher.update(b"\n");
    hasher.update(path);
    hasher.update(b"\n");
    hasher.update(body);
    hasher
        .finalize()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn mutation_response(status: i32, body: String, replayed: bool) -> Response {
    let status = StatusCode::from_u16(status as u16).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
    let mut response = (status, [(header::CONTENT_TYPE, "application/json")], body).into_response();
    if replayed {
        response
            .headers_mut()
            .insert("idempotent-replayed", "true".parse().unwrap());
    }
    response
}

#[derive(OpenApi)]
#[openapi(
    info(title = "Library sync", version = "0.1.0"),
    components(schemas(Item, Snapshot, Changes, Mutation, crate::response::ProblemDetails))
)]
struct LibraryApi;

/// Returns the Library OpenAPI document.
pub fn document() -> utoipa::openapi::OpenApi {
    LibraryApi::openapi()
}
