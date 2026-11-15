//! Cache boundary and JSON helpers.

use std::time::Duration;

use serde::{Serialize, de::DeserializeOwned};
use thiserror::Error;

/// Cache boundary consumed by application code.
#[async_trait::async_trait]
pub trait Store: Send + Sync {
    /// Returns a cached value and whether it existed.
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError>;
    /// Stores a value with a mandatory positive TTL.
    async fn set(&self, key: &str, value: &[u8], ttl: Duration) -> Result<(), CacheError>;
    /// Deletes a cached value.
    async fn delete(&self, key: &str) -> Result<(), CacheError>;
    /// Checks cache connectivity.
    async fn ping(&self) -> Result<(), CacheError>;
}

/// Cache errors.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("cache key must not be empty")]
    InvalidKey,
    #[error("cache ttl must be positive")]
    InvalidTtl,
    #[error("cache encode: {0}")]
    Encode(String),
    #[error("cache decode: {0}")]
    Decode(String),
    #[error("redis: {0}")]
    Redis(#[from] ::redis::RedisError),
}

/// Reads and decodes a JSON cache value.
pub async fn get_json<T, S>(store: &S, key: &str) -> Result<Option<T>, CacheError>
where
    T: DeserializeOwned,
    S: Store + ?Sized,
{
    validate_key(key)?;

    let Some(bytes) = store.get(key).await? else {
        return Ok(None);
    };

    serde_json::from_slice(&bytes)
        .map(Some)
        .map_err(|error| CacheError::Decode(error.to_string()))
}

/// Encodes and writes a JSON cache value.
pub async fn set_json<T, S>(
    store: &S,
    key: &str,
    value: &T,
    ttl: Duration,
) -> Result<(), CacheError>
where
    T: Serialize + Sync,
    S: Store + ?Sized,
{
    validate_key(key)?;
    validate_ttl(ttl)?;

    let bytes = serde_json::to_vec(value).map_err(|error| CacheError::Encode(error.to_string()))?;

    store.set(key, &bytes, ttl).await
}

/// Rejects an empty cache key.
pub fn validate_key(key: &str) -> Result<(), CacheError> {
    if key.trim().is_empty() {
        return Err(CacheError::InvalidKey);
    }

    Ok(())
}

/// Requires an explicit positive TTL.
pub fn validate_ttl(ttl: Duration) -> Result<(), CacheError> {
    if ttl.is_zero() {
        return Err(CacheError::InvalidTtl);
    }

    Ok(())
}

pub mod redis;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_keys_and_ttls() {
        assert!(validate_key("user:1").is_ok());
        assert!(validate_key("  ").is_err());
        assert!(validate_ttl(Duration::from_secs(1)).is_ok());
        assert!(validate_ttl(Duration::ZERO).is_err());
    }
}
