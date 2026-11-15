use sqlx::PgPool;
use time::OffsetDateTime;
use uuid::Uuid;

use super::user::User;

pub async fn find_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, role, email_verified_at, created_at
         FROM users WHERE lower(email) = lower($1)",
    )
    .bind(email)
    .fetch_optional(pool)
    .await
}

pub async fn find_by_id(pool: &PgPool, id: Uuid) -> Result<Option<User>, sqlx::Error> {
    sqlx::query_as::<_, User>(
        "SELECT id, email, password_hash, role, email_verified_at, created_at
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
         RETURNING id, email, password_hash, role, email_verified_at, created_at",
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
    expires_at: OffsetDateTime,
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

pub async fn revoke_all_refresh(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE refresh_tokens SET used_at = now() WHERE user_id = $1 AND used_at IS NULL")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn store_auth_token(
    pool: &PgPool,
    user_id: Uuid,
    kind: &str,
    token_hash: &str,
    expires_at: OffsetDateTime,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO auth_tokens (id, user_id, kind, token_hash, expires_at)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(kind)
    .bind(token_hash)
    .bind(expires_at)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn consume_auth_token(
    pool: &PgPool,
    kind: &str,
    token_hash: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar::<_, Uuid>(
        "UPDATE auth_tokens SET used_at = now()
         WHERE kind = $1 AND token_hash = $2
           AND used_at IS NULL AND expires_at > now()
         RETURNING user_id",
    )
    .bind(kind)
    .bind(token_hash)
    .fetch_optional(pool)
    .await
}

pub async fn mark_email_verified(pool: &PgPool, user_id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET email_verified_at = now(), updated_at = now() WHERE id = $1")
        .bind(user_id)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn update_password(pool: &PgPool, user_id: Uuid, hash: &str) -> Result<(), sqlx::Error> {
    sqlx::query("UPDATE users SET password_hash = $2, updated_at = now() WHERE id = $1")
        .bind(user_id)
        .bind(hash)
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn find_oauth_user(
    pool: &PgPool,
    provider: &str,
    provider_id: &str,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar(
        "SELECT user_id FROM oauth_accounts
         WHERE provider = $1 AND provider_account_id = $2",
    )
    .bind(provider)
    .bind(provider_id)
    .fetch_optional(pool)
    .await
}

pub async fn link_oauth(
    pool: &PgPool,
    user_id: Uuid,
    provider: &str,
    provider_id: &str,
    email: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        "INSERT INTO oauth_accounts (id, user_id, provider, provider_account_id, email)
         VALUES ($1, $2, $3, $4, $5)
         ON CONFLICT (provider, provider_account_id) DO NOTHING",
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(provider)
    .bind(provider_id)
    .bind(email)
    .execute(pool)
    .await?;

    Ok(())
}
