//! Durable job queue used by worker and scheduler binaries.

pub mod codec;
pub mod rabbitmq;
// smokeforge:anchor queue-modules

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use thiserror::Error;
use uuid::Uuid;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: Uuid,
    pub kind: String,
    pub payload: Value,
    pub attempt: u32,
    pub created_at: u64,
}

impl Job {
    pub fn new(kind: impl Into<String>, payload: Value) -> Self {
        Self {
            id: Uuid::new_v4(),
            kind: kind.into(),
            payload,
            attempt: 0,
            created_at: unix_time(),
        }
    }

    pub fn encode(&self) -> Result<Vec<u8>, QueueError> {
        serde_json::to_vec(self).map_err(|error| QueueError::Codec(error.to_string()))
    }

    pub fn decode(data: &[u8]) -> Result<Self, QueueError> {
        serde_json::from_slice(data).map_err(|error| QueueError::Codec(error.to_string()))
    }
}

#[derive(Debug, Error)]
pub enum QueueError {
    #[error("queue: {0}")]
    Message(String),
    #[error("queue codec: {0}")]
    Codec(String),
}

/// Checks and records successfully processed jobs.
#[async_trait]
pub trait Deduplicator: Send + Sync {
    /// Returns true when the job was already processed successfully.
    async fn seen(&self, job: &Job) -> Result<bool, QueueError>;

    /// Records a successfully processed job until the marker expires.
    async fn mark(&self, job: &Job) -> Result<(), QueueError>;
}

#[async_trait]
pub trait Handler: Send + Sync {
    async fn handle(&self, job: Job) -> Result<(), QueueError>;
}

#[derive(Default)]
pub struct Registry {
    handlers: std::collections::HashMap<String, std::sync::Arc<dyn Handler>>,
}

impl Registry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, kind: impl Into<String>, handler: std::sync::Arc<dyn Handler>) {
        self.handlers.insert(kind.into(), handler);
    }

    pub fn lookup(&self, kind: &str) -> Option<std::sync::Arc<dyn Handler>> {
        self.handlers.get(kind).cloned()
    }
}

fn unix_time() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|duration| duration.as_secs())
        .unwrap_or_default()
}
