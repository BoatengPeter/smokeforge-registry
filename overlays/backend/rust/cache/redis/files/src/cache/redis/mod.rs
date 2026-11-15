//! Redis cache adapter.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use redis::aio::ConnectionManager;
use tokio::sync::Mutex;

use super::{CacheError, Store};
use crate::config::Config;

/// A cloneable Redis cache handle.
#[derive(Clone)]
pub struct RedisCache {
    connection: Arc<Mutex<ConnectionManager>>,
    prefix: String,
}

impl RedisCache {
    pub async fn connect(config: &Config) -> Result<Self, CacheError> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|error| CacheError::Encode(error.to_string()))?;
        let connection = ConnectionManager::new(client).await?;

        Ok(Self {
            connection: Arc::new(Mutex::new(connection)),
            prefix: config.redis_key_prefix.trim_end_matches(':').to_owned(),
        })
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
            .arg(ttl_seconds.max(1))
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

    pub async fn ping(&self) -> Result<(), CacheError> {
        let mut connection = self.connection.lock().await;
        let _: String = redis::cmd("PING").query_async(&mut *connection).await?;

        Ok(())
    }

    fn key(&self, key: &str) -> String {
        if self.prefix.is_empty() {
            key.to_owned()
        } else {
            format!("{}:{key}", self.prefix)
        }
    }
}

#[async_trait]
impl Store for RedisCache {
    async fn get(&self, key: &str) -> Result<Option<Vec<u8>>, CacheError> {
        let key = self.key(key);
        let mut connection = self.connection.lock().await;
        let value: Option<Vec<u8>> = redis::cmd("GET")
            .arg(key)
            .query_async(&mut *connection)
            .await?;

        Ok(value)
    }

    async fn set(&self, key: &str, value: &[u8], ttl: Duration) -> Result<(), CacheError> {
        super::validate_key(key)?;
        super::validate_ttl(ttl)?;

        let key = self.key(key);
        let mut connection = self.connection.lock().await;
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(value)
            .arg("PX")
            .arg(ttl.as_millis().max(1) as u64)
            .query_async(&mut *connection)
            .await?;

        Ok(())
    }

    async fn delete(&self, key: &str) -> Result<(), CacheError> {
        RedisCache::delete(self, key).await
    }

    async fn ping(&self) -> Result<(), CacheError> {
        RedisCache::ping(self).await
    }
}
