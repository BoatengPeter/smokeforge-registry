//! Redis cache adapter.

use std::sync::Arc;

use redis::aio::ConnectionManager;
use thiserror::Error;
use tokio::sync::Mutex;

use crate::config::Config;

/// Redis cache errors.
#[derive(Debug, Error)]
pub enum CacheError {
    #[error("redis configuration: {0}")]
    Config(String),
    #[error("redis operation: {0}")]
    Operation(#[from] redis::RedisError),
}

/// A cloneable Redis cache handle.
#[derive(Clone)]
pub struct RedisCache {
    connection: Arc<Mutex<ConnectionManager>>,
    prefix: String,
}

impl RedisCache {
    pub async fn connect(config: &Config) -> Result<Self, CacheError> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|error| CacheError::Config(error.to_string()))?;
        let connection = ConnectionManager::new(client).await?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            prefix: config.redis_key_prefix.clone(),
        })
    }

    pub async fn ping(&self) -> Result<(), CacheError> {
        let mut connection = self.connection.lock().await;
        let _: String = redis::cmd("PING").query_async(&mut *connection).await?;

        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<String>, CacheError> {
        let key = self.key(key);
        let mut connection = self.connection.lock().await;

        Ok(redis::cmd("GET")
            .arg(key)
            .query_async::<Option<String>>(&mut *connection)
            .await?)
    }

    pub async fn set(&self, key: &str, value: &str, ttl_seconds: u64) -> Result<(), CacheError> {
        let key = self.key(key);
        let mut connection = self.connection.lock().await;

        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("EX")
            .arg(ttl_seconds)
            .query_async(&mut *connection)
            .await?;

        Ok(())
    }

    pub async fn delete(&self, key: &str) -> Result<(), CacheError> {
        let key = self.key(key);
        let mut connection = self.connection.lock().await;

        let _: usize = redis::cmd("DEL")
            .arg(key)
            .query_async(&mut *connection)
            .await?;

        Ok(())
    }

    fn key(&self, key: &str) -> String {
        format!("{}{}", self.prefix, key)
    }
}
