use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;
use uuid::Uuid;

use crate::error::AppError;

/// JWT claims used by access tokens.
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
    pub iss: String,
    pub aud: String,
}

/// Issues a signed access token.
pub fn access_token(
    user_id: Uuid,
    role: &str,
    secret: &str,
    issuer: &str,
    audience: &str,
    ttl_seconds: i64,
) -> Result<String, AppError> {
    let now = OffsetDateTime::now_utc();
    let claims = Claims {
        sub: user_id,
        role: role.to_owned(),
        exp: (now.unix_timestamp() + ttl_seconds) as usize,
        iat: now.unix_timestamp() as usize,
        iss: issuer.to_owned(),
        aud: audience.to_owned(),
    };

    let key = EncodingKey::from_secret(secret.as_bytes())
        .map_err(|_| AppError::internal("token signing key failed"))?;

    encode(&Header::new(Algorithm::HS256), &claims, &key)
        .map_err(|_| AppError::internal("token signing failed"))
}

/// Verifies an access token and returns its claims.
pub fn verify_access(
    token: &str,
    secret: &str,
    issuer: &str,
    audience: &str,
) -> Result<Claims, AppError> {
    let mut validation = Validation::new(Algorithm::HS256);
    validation.set_issuer(&[issuer]);
    validation.set_audience(&[audience]);

    let key = DecodingKey::from_secret(secret.as_bytes())
        .map_err(|_| AppError::internal("token verification key failed"))?;

    decode::<Claims>(token, &key, &validation)
        .map(|data| data.claims)
        .map_err(|_| AppError::unauthorized("A valid access token is required."))
}

/// Creates an opaque refresh token and its storage hash.
pub fn refresh_token() -> (String, String) {
    let raw = Uuid::new_v4().to_string();
    (raw.clone(), hash_refresh(&raw))
}

/// Hashes a refresh token before storage or lookup.
pub fn hash_refresh(raw: &str) -> String {
    let digest = Sha256::digest(raw.as_bytes());
    hex::encode(digest)
}
