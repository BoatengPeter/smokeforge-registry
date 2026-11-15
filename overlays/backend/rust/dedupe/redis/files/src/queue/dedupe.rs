//! Redis-backed success marker storage for queue jobs.

use redis::aio::ConnectionManager;
use uuid::Uuid;

pub use super::Deduplicator;
use super::{Job, QueueError};
use crate::config::Config;

const MARKER_VALUE: &str = "processed";

/// Redis-backed deduplication marker store.
#[derive(Clone)]
pub struct RedisDeduplicator {
    connection: ConnectionManager,
    prefix: String,
    ttl_seconds: u64,
}

impl RedisDeduplicator {
    pub async fn connect(config: &Config) -> Result<Self, QueueError> {
        let client = redis::Client::open(config.redis_url.as_str())
            .map_err(|error| QueueError::Message(format!("redis configuration: {error}")))?;
        let connection = ConnectionManager::new(client)
            .await
            .map_err(|error| QueueError::Message(format!("redis connection: {error}")))?;

        Ok(Self {
            connection,
            prefix: key_prefix(&config.redis_key_prefix, &config.service_name),
            ttl_seconds: config.dedupe_ttl_seconds,
        })
    }

    fn key(&self, job: &Job) -> String {
        dedupe_key(&self.prefix, job.id)
    }
}

#[async_trait::async_trait]
impl Deduplicator for RedisDeduplicator {
    async fn seen(&self, job: &Job) -> Result<bool, QueueError> {
        let key = self.key(job);
        let mut connection = self.connection.clone();
        let exists: usize = redis::cmd("EXISTS")
            .arg(key)
            .query_async(&mut connection)
            .await
            .map_err(|error| QueueError::Message(format!("check dedupe marker: {error}")))?;

        Ok(exists > 0)
    }

    async fn mark(&self, job: &Job) -> Result<(), QueueError> {
        let key = self.key(job);
        let mut connection = self.connection.clone();
        let _: () = redis::cmd("SET")
            .arg(key)
            .arg(MARKER_VALUE)
            .arg("EX")
            .arg(self.ttl_seconds)
            .query_async(&mut connection)
            .await
            .map_err(|error| QueueError::Message(format!("write dedupe marker: {error}")))?;

        Ok(())
    }
}

fn key_prefix(configured: &str, service_name: &str) -> String {
    let configured = configured.trim();
    let service_name = service_name.trim();
    let prefix = if configured.is_empty() {
        service_name
    } else {
        configured
    };

    prefix.trim_end_matches(':').to_owned()
}

fn dedupe_key(prefix: &str, job_id: Uuid) -> String {
    format!("{prefix}:dedupe:{job_id}")
}

#[cfg(test)]
mod tests {
    use super::{dedupe_key, key_prefix};
    use uuid::Uuid;

    #[test]
    fn uses_service_name_when_prefix_is_blank() {
        assert_eq!(key_prefix("   ", "orders"), "orders");
    }

    #[test]
    fn trims_trailing_colons_from_prefix() {
        assert_eq!(key_prefix("tenant:jobs:", "orders"), "tenant:jobs");
    }

    #[test]
    fn builds_dedupe_key_from_job_id() {
        let job_id = Uuid::nil();

        assert_eq!(
            dedupe_key("orders", job_id),
            "orders:dedupe:00000000-0000-0000-0000-000000000000"
        );
    }
}
