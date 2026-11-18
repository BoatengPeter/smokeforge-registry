use serde::{Deserialize, Serialize};
use sqlx::{FromRow, PgPool, Postgres, Row, Transaction};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

const IDEMPOTENCY_TTL_HOURS: i32 = 24;

#[derive(Debug, Clone, Serialize, Deserialize, utoipa::ToSchema)]
pub struct Item {
    pub id: Uuid,
    pub title: String,
    pub description: String,
    pub status: String,
    pub tags: Vec<String>,
    pub version: i64,
    pub created_at: OffsetDateTime,
    pub updated_at: OffsetDateTime,
    pub deleted_at: Option<OffsetDateTime>,
}

#[derive(Debug, FromRow)]
struct ItemRow {
    id: Uuid,
    title: String,
    description: String,
    status: String,
    tags: String,
    version: i64,
    created_at: OffsetDateTime,
    updated_at: OffsetDateTime,
    deleted_at: Option<OffsetDateTime>,
}

impl From<ItemRow> for Item {
    fn from(row: ItemRow) -> Self {
        Self {
            id: row.id,
            title: row.title,
            description: row.description,
            status: row.status,
            tags: serde_json::from_str(&row.tags).unwrap_or_default(),
            version: row.version,
            created_at: row.created_at,
            updated_at: row.updated_at,
            deleted_at: row.deleted_at,
        }
    }
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Change {
    pub seq: i64,
    pub operation: String,
    pub item_id: Uuid,
    pub version: i64,
    pub item: Item,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Snapshot {
    pub items: Vec<Item>,
    pub cursor: i64,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Changes {
    pub changes: Vec<Change>,
    pub cursor: i64,
    pub has_more: bool,
}

#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct Mutation {
    pub item: Item,
    pub change_seq: i64,
}

#[derive(Debug, Clone)]
pub struct LibraryService {
    pool: PgPool,
}

impl LibraryService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn snapshot(&self, user_id: Uuid) -> Result<Snapshot, AppError> {
        let mut tx = self.pool.begin().await.map_err(internal)?;
        let rows = sqlx::query_as::<_, ItemRow>(
            "SELECT id, title, description, status, tags::text AS tags, version, created_at, updated_at, deleted_at
             FROM library_items
             WHERE user_id = $1 AND deleted_at IS NULL
             ORDER BY updated_at DESC",
        )
        .bind(user_id)
        .fetch_all(&mut *tx)
        .await
        .map_err(internal)?;
        let cursor: i64 = sqlx::query_scalar::<_, i64>(
            "SELECT COALESCE(MAX(seq), 0)::bigint FROM library_changes WHERE user_id = $1",
        )
        .bind(user_id)
        .fetch_one(&mut *tx)
        .await
        .map_err(internal)?;
        tx.commit().await.map_err(internal)?;

        Ok(Snapshot {
            items: rows.into_iter().map(Item::from).collect(),
            cursor,
        })
    }

    pub async fn changes(
        &self,
        user_id: Uuid,
        cursor: i64,
        limit: i64,
    ) -> Result<Changes, AppError> {
        let limit = limit.clamp(1, 500);
        let rows = sqlx::query(
            "SELECT seq, operation, item_id, version, payload::text AS payload
             FROM library_changes
             WHERE user_id = $1 AND seq > $2
             ORDER BY seq
             LIMIT $3",
        )
        .bind(user_id)
        .bind(cursor)
        .bind(limit + 1)
        .fetch_all(&self.pool)
        .await
        .map_err(internal)?;

        let mut changes = Vec::with_capacity(rows.len());
        let mut next = cursor;
        for row in rows {
            let payload: String = row.try_get("payload").map_err(internal)?;
            let item: Item = serde_json::from_str(&payload).map_err(internal)?;
            let seq: i64 = row.try_get("seq").map_err(internal)?;
            next = next.max(seq);
            changes.push(Change {
                seq,
                operation: row.try_get("operation").map_err(internal)?,
                item_id: row.try_get("item_id").map_err(internal)?,
                version: row.try_get("version").map_err(internal)?,
                item,
            });
        }
        let has_more = changes.len() as i64 > limit;
        if has_more {
            changes.truncate(limit as usize);
            next = changes.last().map_or(cursor, |change| change.seq);
        }

        Ok(Changes {
            changes,
            cursor: next,
            has_more,
        })
    }

    pub async fn get(&self, user_id: Uuid, item_id: Uuid) -> Result<Item, AppError> {
        let row = sqlx::query_as::<_, ItemRow>(
            "SELECT id, title, description, status, tags::text AS tags, version, created_at, updated_at, deleted_at
             FROM library_items
             WHERE id = $1 AND user_id = $2 AND deleted_at IS NULL",
        )
        .bind(item_id)
        .bind(user_id)
        .fetch_optional(&self.pool)
        .await
        .map_err(internal)?
        .ok_or_else(not_found)?;
        Ok(row.into())
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        title: String,
        description: String,
        status: String,
        tags: Vec<String>,
        key: String,
        hash: String,
    ) -> Result<(i32, String, bool), AppError> {
        self.mutate(user_id, key, hash, move |tx| {
            Box::pin(async move {
                let tags = serde_json::to_string(&tags).map_err(internal)?;
                let row = sqlx::query_as::<_, ItemRow>(
                    "INSERT INTO library_items (id, user_id, title, description, status, tags)
                     VALUES ($1, $2, $3, $4, $5, $6::jsonb)
                     RETURNING id, title, description, status, tags::text AS tags, version, created_at, updated_at, deleted_at",
                )
                .bind(item_id)
                .bind(user_id)
                .bind(title)
                .bind(description)
                .bind(status)
                .bind(tags)
                .fetch_one(&mut **tx)
                .await
                .map_err(|error| {
                    if is_unique_violation(&error) { conflict("item_exists", "An item with that identifier already exists.") } else { internal(error) }
                })?;
                let item = Item::from(row);
                let seq = append_change(tx, user_id, &item, "create").await?;
                Ok((201, Mutation { item, change_seq: seq }))
            })
        })
        .await
    }

    pub async fn update(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        version: i64,
        title: String,
        description: String,
        status: String,
        tags: Vec<String>,
        key: String,
        hash: String,
    ) -> Result<(i32, String, bool), AppError> {
        self.mutate(user_id, key, hash, move |tx| {
            Box::pin(async move {
                let tags = serde_json::to_string(&tags).map_err(internal)?;
                let row = sqlx::query_as::<_, ItemRow>(
                    "UPDATE library_items
                     SET title = $1, description = $2, status = $3, tags = $4::jsonb, version = version + 1, updated_at = now()
                     WHERE id = $5 AND user_id = $6 AND version = $7 AND deleted_at IS NULL
                     RETURNING id, title, description, status, tags::text AS tags, version, created_at, updated_at, deleted_at",
                )
                .bind(title).bind(description).bind(status).bind(tags)
                .bind(item_id).bind(user_id).bind(version)
                .fetch_optional(&mut **tx)
                .await
                .map_err(internal)?
                .ok_or_else(|| conflict("version_conflict", "The item was updated by another client."))?;
                let item = Item::from(row);
                let seq = append_change(tx, user_id, &item, "update").await?;
                Ok((200, Mutation { item, change_seq: seq }))
            })
        })
        .await
    }

    pub async fn delete(
        &self,
        user_id: Uuid,
        item_id: Uuid,
        version: i64,
        key: String,
        hash: String,
    ) -> Result<(i32, String, bool), AppError> {
        self.mutate(user_id, key, hash, move |tx| {
            Box::pin(async move {
                let row = sqlx::query_as::<_, ItemRow>(
                    "UPDATE library_items
                     SET deleted_at = now(), updated_at = now(), version = version + 1
                     WHERE id = $1 AND user_id = $2 AND version = $3 AND deleted_at IS NULL
                     RETURNING id, title, description, status, tags::text AS tags, version, created_at, updated_at, deleted_at",
                )
                .bind(item_id).bind(user_id).bind(version)
                .fetch_optional(&mut **tx)
                .await
                .map_err(internal)?
                .ok_or_else(|| conflict("version_conflict", "The item was updated by another client."))?;
                let item = Item::from(row);
                let seq = append_change(tx, user_id, &item, "delete").await?;
                Ok((200, Mutation { item, change_seq: seq }))
            })
        })
        .await
    }

    async fn mutate<F>(
        &self,
        user_id: Uuid,
        key: String,
        hash: String,
        apply: F,
    ) -> Result<(i32, String, bool), AppError>
    where
        F: for<'a> FnOnce(
            &'a mut Transaction<'_, Postgres>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = Result<(i32, Mutation), AppError>> + Send + 'a>,
        >,
    {
        if key.is_empty() {
            return Err(AppError::new(
                axum::http::StatusCode::BAD_REQUEST,
                "idempotency_key_required",
                "Idempotency key required",
                "Send an Idempotency-Key header.",
            ));
        }
        let mut tx = self.pool.begin().await.map_err(internal)?;
        let cached = sqlx::query(
            "SELECT request_hash, status_code, response_body::text AS response_body
             FROM idempotency_keys WHERE user_id = $1 AND key = $2 AND expires_at > now()",
        )
        .bind(user_id)
        .bind(&key)
        .fetch_optional(&mut *tx)
        .await
        .map_err(internal)?;
        if let Some(row) = cached {
            let stored_hash: String = row.try_get("request_hash").map_err(internal)?;
            if stored_hash != hash {
                return Err(conflict(
                    "idempotency_key_reused",
                    "The idempotency key was reused with a different request.",
                ));
            }
            let status: i32 = row.try_get("status_code").map_err(internal)?;
            let body: String = row.try_get("response_body").map_err(internal)?;
            tx.commit().await.map_err(internal)?;
            return Ok((status, body, true));
        }
        let (status, response) = apply(&mut tx).await?;
        let body = serde_json::to_string(&response).map_err(internal)?;
        sqlx::query(
            "INSERT INTO idempotency_keys (user_id, key, request_hash, status_code, response_body, expires_at)
             VALUES ($1, $2, $3, $4, $5::jsonb, now() + ($6 || ' hours')::interval)",
        )
        .bind(user_id).bind(key).bind(hash).bind(status).bind(&body).bind(IDEMPOTENCY_TTL_HOURS)
        .execute(&mut *tx).await.map_err(internal)?;
        tx.commit().await.map_err(internal)?;
        Ok((status, body, false))
    }
}

async fn append_change(
    tx: &mut Transaction<'_, Postgres>,
    user_id: Uuid,
    item: &Item,
    operation: &str,
) -> Result<i64, AppError> {
    let payload = serde_json::to_string(item).map_err(internal)?;
    let seq = sqlx::query_scalar::<_, i64>(
        "INSERT INTO library_changes (user_id, item_id, operation, version, payload)
         VALUES ($1, $2, $3, $4, $5::jsonb) RETURNING seq",
    )
    .bind(user_id)
    .bind(item.id)
    .bind(operation)
    .bind(item.version)
    .bind(payload)
    .fetch_one(&mut **tx)
    .await
    .map_err(internal)?;
    Ok(seq)
}

fn is_unique_violation(error: &sqlx::Error) -> bool {
    matches!(error, sqlx::Error::Database(database) if database.code().as_deref() == Some("23505"))
}

fn internal(error: impl std::fmt::Display) -> AppError {
    tracing::error!(error = %error, "library operation failed");
    AppError::internal("The library operation could not be completed.")
}

fn not_found() -> AppError {
    AppError::not_found("The requested library item does not exist.")
}

fn conflict(code: &'static str, detail: &str) -> AppError {
    AppError::new(axum::http::StatusCode::CONFLICT, code, "Conflict", detail)
}
