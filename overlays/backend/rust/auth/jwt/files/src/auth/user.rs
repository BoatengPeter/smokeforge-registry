use serde::Serialize;
use sqlx::FromRow;
use time::OffsetDateTime;
use uuid::Uuid;

/// Persisted user row.
#[derive(Debug, FromRow)]
pub struct User {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub role: String,
    pub email_verified_at: Option<OffsetDateTime>,
    pub created_at: OffsetDateTime,
}

/// Public user representation.
#[derive(Debug, Serialize, utoipa::ToSchema)]
pub struct UserResponse {
    pub id: Uuid,
    pub email: String,
    pub role: String,
    pub email_verified: bool,
    pub created_at: OffsetDateTime,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            role: user.role,
            email_verified: user.email_verified_at.is_some(),
            created_at: user.created_at,
        }
    }
}
