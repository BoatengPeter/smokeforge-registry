use async_trait::async_trait;

use super::{Job, QueueError, rabbitmq::Rabbit};

/// Publishes jobs from application code.
#[async_trait]
pub trait Publisher: Send + Sync {
    async fn publish(&self, job: Job) -> Result<(), QueueError>;
}

#[async_trait]
impl Publisher for Rabbit {
    async fn publish(&self, job: Job) -> Result<(), QueueError> {
        Rabbit::publish(self, &job).await
    }
}
