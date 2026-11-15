use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

#[derive(Debug, sqlx::FromRow)]
pub struct Media {
    pub id: Uuid,
    pub owner_id: Uuid,
    pub object_key: String,
    pub content_type: String,
    pub byte_size: i64,
    pub filename: String,
    pub status: String,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
}

pub async fn create(pool: &PgPool, media: &Media) -> Result<(), AppError> {
    sqlx::query(
        "INSERT INTO media_objects
         (id, owner_id, object_key, content_type, byte_size, filename, status)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(media.id)
    .bind(media.owner_id)
    .bind(&media.object_key)
    .bind(&media.content_type)
    .bind(media.byte_size)
    .bind(&media.filename)
    .bind(&media.status)
    .execute(pool)
    .await
    .map_err(|_| AppError::internal("could not create media row"))?;

    Ok(())
}

pub async fn get(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<Media, AppError> {
    sqlx::query_as::<_, Media>(
        "SELECT id, owner_id, object_key, content_type, byte_size, filename, status,
                created_at, updated_at
         FROM media_objects WHERE id = $1 AND owner_id = $2",
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::internal("could not load media"))?
    .ok_or_else(|| AppError::not_found("The image does not exist."))
}

pub async fn mark_ready(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<Media, AppError> {
    sqlx::query_as::<_, Media>(
        "UPDATE media_objects SET status = 'ready', updated_at = now()
         WHERE id = $1 AND owner_id = $2 AND status = 'pending'
         RETURNING id, owner_id, object_key, content_type, byte_size, filename, status,
                   created_at, updated_at",
    )
    .bind(id)
    .bind(owner_id)
    .fetch_optional(pool)
    .await
    .map_err(|_| AppError::internal("could not mark media ready"))?
    .ok_or_else(|| AppError::not_found("The image does not exist."))
}

pub async fn delete(pool: &PgPool, id: Uuid, owner_id: Uuid) -> Result<(), AppError> {
    sqlx::query("DELETE FROM media_objects WHERE id = $1 AND owner_id = $2")
        .bind(id)
        .bind(owner_id)
        .execute(pool)
        .await
        .map_err(|_| AppError::internal("could not delete media"))?;

    Ok(())
}
