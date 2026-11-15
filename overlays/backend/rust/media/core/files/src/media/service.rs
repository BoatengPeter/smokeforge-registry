use serde::Serialize;
use sqlx::PgPool;
use uuid::Uuid;

use super::{
    repository::{self, Media},
    s3::S3,
};
use crate::{config::Config, error::AppError};

pub struct MediaService {
    pool: PgPool,
    s3: S3,
    config: Config,
}

#[derive(Debug, Serialize)]
pub struct UploadResponse {
    pub media: MediaResponse,
    pub upload_url: String,
    pub upload_headers: std::collections::HashMap<String, String>,
    pub expires_in: u64,
}

#[derive(Debug, Serialize)]
pub struct MediaResponse {
    pub id: Uuid,
    pub filename: String,
    pub content_type: String,
    pub size_bytes: i64,
    pub status: String,
}

impl From<Media> for MediaResponse {
    fn from(media: Media) -> Self {
        Self {
            id: media.id,
            filename: media.filename,
            content_type: media.content_type,
            size_bytes: media.byte_size,
            status: media.status,
        }
    }
}

impl MediaService {
    pub fn new(pool: PgPool, config: &Config) -> Result<Self, AppError> {
        Ok(Self {
            pool,
            s3: S3::new(config)?,
            config: config.clone(),
        })
    }

    pub async fn create_upload(
        &self,
        owner_id: Uuid,
        filename: String,
        content_type: String,
        byte_size: i64,
    ) -> Result<UploadResponse, AppError> {
        let content_type = validate_upload(&self.config, &filename, &content_type, byte_size)?;
        let id = Uuid::new_v4();
        let media = Media {
            id,
            owner_id,
            object_key: format!("users/{owner_id}/media/{id}{}", extension(&content_type)),
            content_type: content_type.clone(),
            byte_size,
            filename,
            status: "pending".to_owned(),
            created_at: time::OffsetDateTime::now_utc(),
            updated_at: time::OffsetDateTime::now_utc(),
        };

        repository::create(&self.pool, &media).await?;

        let upload_url = self
            .s3
            .presign_put(&media.object_key, self.config.media_upload_ttl_seconds)?;
        let mut headers = std::collections::HashMap::new();
        headers.insert("Content-Type".to_owned(), content_type);

        Ok(UploadResponse {
            media: media.into(),
            upload_url,
            upload_headers: headers,
            expires_in: self.config.media_upload_ttl_seconds,
        })
    }

    pub async fn complete(&self, owner_id: Uuid, id: Uuid) -> Result<MediaResponse, AppError> {
        let media = repository::get(&self.pool, id, owner_id).await?;

        if media.status == "ready" {
            return Ok(media.into());
        }

        let (size, content_type) = self.s3.head(&media.object_key).await?;
        if size != media.byte_size {
            return Err(AppError::new(
                reqwest::StatusCode::UNPROCESSABLE_ENTITY,
                "size_mismatch",
                "Upload size mismatch",
                "The uploaded object size does not match the reservation.",
            ));
        }

        if !content_type.is_empty() && content_type != media.content_type {
            return Err(AppError::new(
                reqwest::StatusCode::UNPROCESSABLE_ENTITY,
                "content_type_mismatch",
                "Upload content type mismatch",
                "The uploaded object content type does not match the reservation.",
            ));
        }

        Ok(repository::mark_ready(&self.pool, id, owner_id)
            .await?
            .into())
    }

    pub async fn get(&self, owner_id: Uuid, id: Uuid) -> Result<MediaResponse, AppError> {
        Ok(repository::get(&self.pool, id, owner_id).await?.into())
    }

    pub async fn url(&self, owner_id: Uuid, id: Uuid) -> Result<String, AppError> {
        let media = repository::get(&self.pool, id, owner_id).await?;
        if media.status != "ready" {
            return Err(AppError::new(
                reqwest::StatusCode::UNPROCESSABLE_ENTITY,
                "media_pending",
                "Image pending",
                "Complete the upload before requesting a URL.",
            ));
        }

        self.s3
            .presign_get(&media.object_key, self.config.media_download_ttl_seconds)
    }

    pub fn download_ttl(&self) -> u64 {
        self.config.media_download_ttl_seconds
    }

    pub async fn delete(&self, owner_id: Uuid, id: Uuid) -> Result<(), AppError> {
        let media = repository::get(&self.pool, id, owner_id).await?;
        self.s3.delete(&media.object_key).await?;
        repository::delete(&self.pool, id, owner_id).await
    }
}

fn validate_upload(
    config: &Config,
    filename: &str,
    content_type: &str,
    byte_size: i64,
) -> Result<String, AppError> {
    if filename.trim().is_empty() || filename.len() > 255 {
        return Err(invalid("filename must contain 1 to 255 characters"));
    }

    if byte_size < 1 || byte_size > config.media_max_bytes {
        return Err(invalid("upload size is outside the configured limit"));
    }

    let content_type = content_type.trim().to_ascii_lowercase();
    let allowed = config
        .media_allowed_content_types
        .split(',')
        .map(str::trim)
        .any(|value| value.eq_ignore_ascii_case(&content_type));

    if !allowed {
        return Err(invalid("content type is not allowed"));
    }

    Ok(content_type)
}

fn invalid(detail: &str) -> AppError {
    AppError::new(
        reqwest::StatusCode::UNPROCESSABLE_ENTITY,
        "validation_failed",
        "Validation failed",
        detail,
    )
}

fn extension(content_type: &str) -> &'static str {
    match content_type {
        "image/jpeg" => ".jpg",
        "image/png" => ".png",
        "image/webp" => ".webp",
        "image/gif" => ".gif",
        _ => "",
    }
}
