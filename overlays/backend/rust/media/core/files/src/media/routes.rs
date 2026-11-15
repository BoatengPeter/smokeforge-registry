use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::Path,
    http::{HeaderMap, StatusCode, header},
    routing::{get, post},
};
use serde::Deserialize;
use uuid::Uuid;

use super::service::{MediaResponse, MediaService, UploadResponse};
use crate::{auth::AuthService, error::AppError};

pub fn router(media: Arc<MediaService>, auth: Arc<AuthService>) -> Router {
    Router::new()
        .route("/api/v1/media/uploads", post(create_upload))
        .route("/api/v1/media/{id}/complete", post(complete))
        .route("/api/v1/media/{id}", get(get_media).delete(delete_media))
        .route("/api/v1/media/{id}/url", get(get_url))
        .layer(Extension(media))
        .layer(Extension(auth))
}

#[derive(Debug, Deserialize)]
pub struct CreateUploadRequest {
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
}

async fn create_upload(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(media): Extension<Arc<MediaService>>,
    headers: HeaderMap,
    Json(input): Json<CreateUploadRequest>,
) -> Result<(StatusCode, Json<UploadResponse>), AppError> {
    let user = auth.current_user(bearer(&headers)?).await?;
    let upload = media
        .create_upload(
            user.id,
            input.filename,
            input.content_type,
            input.size_bytes,
        )
        .await?;

    Ok((StatusCode::CREATED, Json(upload)))
}

async fn complete(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(media): Extension<Arc<MediaService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaResponse>, AppError> {
    let user = auth.current_user(bearer(&headers)?).await?;

    Ok(Json(media.complete(user.id, id).await?))
}

async fn get_media(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(media): Extension<Arc<MediaService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<MediaResponse>, AppError> {
    let user = auth.current_user(bearer(&headers)?).await?;

    Ok(Json(media.get(user.id, id).await?))
}

async fn get_url(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(media): Extension<Arc<MediaService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<Json<serde_json::Value>, AppError> {
    let user = auth.current_user(bearer(&headers)?).await?;
    let url = media.url(user.id, id).await?;

    Ok(Json(serde_json::json!({
        "url": url,
        "expires_in": media.download_ttl(),
    })))
}

async fn delete_media(
    Extension(auth): Extension<Arc<AuthService>>,
    Extension(media): Extension<Arc<MediaService>>,
    headers: HeaderMap,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let user = auth.current_user(bearer(&headers)?).await?;
    media.delete(user.id, id).await?;

    Ok(StatusCode::NO_CONTENT)
}

fn bearer(headers: &HeaderMap) -> Result<&str, AppError> {
    headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.strip_prefix("Bearer "))
        .filter(|token| !token.is_empty())
        .ok_or_else(|| AppError::unauthorized("A valid access token is required."))
}
