use sqlx::PgPool;
use uuid::Uuid;

use super::user::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, role, created_at
         FROM users WHERE lower(email) = lower($1)",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, role, created_at
         FROM users WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(pool)
    .await
}

pub async fn insert_user(
    pool: &PgPool,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "INSERT INTO users (id, email, password_hash)
         VALUES ($1, $2, $3)
         RETURNING id, email, password_hash, role, created_at",
    )
    .bind(Uuid::new_v4())
    .bind(email)
    .bind(password_hash)
    .fetch_one(pool)
    .await
}

pub async fn store_refresh(
    pool: &PgPool,
    user_id: Uuid,
    token_hash: &str,
    expires_at: time::OffsetDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO refresh_tokens (id, user_id, token_hash, expires_at)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn consume_refresh(pool: &PgPool, token_hash: &str) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>(
        "UPDATE refresh_tokens SET used_at = now()
         WHERE token_hash = $1 AND used_at IS NULL AND expires_at > now()
         RETURNING user_id",
    )
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

pub async fn revoke_refresh(pool: &PgPool, token_hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE refresh_tokens SET used_at = now() WHERE token_hash = $1")
        .bind(token_hash)
        .execute(pool)
        .await?;

    Ok(())
}
