use argon2::{Argon2, PasswordHasher, PasswordVerifier, password_hash::phc::PasswordHash};

use crate::error::AppError;

/// Hashes a password with Argon2id.
pub fn hash(password: &str) -> Result<String, AppError> {
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|_| AppError::internal("password hashing failed"))
}

/// Verifies a password against a stored Argon2 hash.
pub fn verify(password: &str, encoded: &str) -> Result<bool, AppError> {
    let parsed = PasswordHash::new(encoded)
        .map_err(|_| AppError::internal("stored password hash is invalid"))?;

    Ok(Argon2::default()
        .verify_password(password.as_bytes(), &parsed)
        .is_ok())
}
