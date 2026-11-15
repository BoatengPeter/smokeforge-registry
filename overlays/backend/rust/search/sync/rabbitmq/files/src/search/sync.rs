//! Queue-backed search indexing and worker handlers.

use std::sync::Arc;

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::{
    queue::{Handler, Job, Publisher, QueueError, Registry},
    search::{Document, Indexer, SearchError, meilisearch::Meilisearch},
};

/// Job kind that indexes or replaces search documents.
pub const UPSERT_JOB_TYPE: &str = "search.documents.upsert";

/// Job kind that removes search documents by ID.
pub const DELETE_JOB_TYPE: &str = "search.documents.delete";

/// Queue payload for document indexing.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UpsertPayload {
    pub documents: Vec<Document>,
}

/// Queue payload for document deletion.
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DeletePayload {
    pub ids: Vec<String>,
}

/// Search indexer that publishes indexing jobs instead of writing synchronously.
pub struct QueueIndexer {
    publisher: Arc<dyn Publisher>,
}

impl QueueIndexer {
    /// Creates a queue-backed indexer from a publisher such as
    /// [`crate::queue::rabbitmq::Rabbit`].
    pub fn new<P>(publisher: Arc<P>) -> Self
    where
        P: Publisher + 'static,
    {
        Self { publisher }
    }
}

impl Indexer for QueueIndexer {
    async fn upsert(&self, documents: Vec<Document>) -> Result<(), SearchError> {
        if documents.is_empty() {
            return Ok(());
        }

        let job = new_job(UPSERT_JOB_TYPE, &UpsertPayload { documents })
            .map_err(|error| SearchError::Invalid(format!("encode search upsert job: {error}")))?;

        self.publisher
            .publish(job)
            .await
            .map_err(|error| SearchError::Unavailable(format!("publish search upsert: {error}")))
    }

    async fn delete(&self, ids: Vec<String>) -> Result<(), SearchError> {
        if ids.is_empty() {
            return Ok(());
        }

        let job = new_job(DELETE_JOB_TYPE, &DeletePayload { ids })
            .map_err(|error| SearchError::Invalid(format!("encode search delete job: {error}")))?;

        self.publisher
            .publish(job)
            .await
            .map_err(|error| SearchError::Unavailable(format!("publish search delete: {error}")))
    }
}

/// Registers the search upsert and delete handlers on a worker registry.
pub fn register<I>(registry: &mut Registry, indexer: Arc<I>)
where
    I: Indexer + 'static,
{
    registry.register(
        UPSERT_JOB_TYPE,
        Arc::new(UpsertHandler {
            indexer: Arc::clone(&indexer),
        }),
    );
    registry.register(DELETE_JOB_TYPE, Arc::new(DeleteHandler { indexer }));
}

struct UpsertHandler<I: Indexer> {
    indexer: Arc<I>,
}

#[async_trait]
impl<I> Handler for UpsertHandler<I>
where
    I: Indexer + 'static,
{
    async fn handle(&self, job: Job) -> Result<(), QueueError> {
        let payload: UpsertPayload = serde_json::from_value(job.payload)
            .map_err(|error| QueueError::Codec(format!("decode search upsert job: {error}")))?;

        if payload.documents.is_empty() {
            return Ok(());
        }

        self.indexer
            .upsert(payload.documents)
            .await
            .map_err(|error| QueueError::Message(format!("apply search upsert: {error}")))
    }
}

struct DeleteHandler<I: Indexer> {
    indexer: Arc<I>,
}

#[async_trait]
impl<I> Handler for DeleteHandler<I>
where
    I: Indexer + 'static,
{
    async fn handle(&self, job: Job) -> Result<(), QueueError> {
        let payload: DeletePayload = serde_json::from_value(job.payload)
            .map_err(|error| QueueError::Codec(format!("decode search delete job: {error}")))?;

        if payload.ids.is_empty() {
            return Ok(());
        }

        self.indexer
            .delete(payload.ids)
            .await
            .map_err(|error| QueueError::Message(format!("apply search delete: {error}")))
    }
}

fn new_job<T: Serialize>(kind: &str, payload: &T) -> Result<Job, serde_json::Error> {
    Ok(Job::new(kind, serde_json::to_value(payload)?))
}

impl Indexer for Meilisearch {
    async fn upsert(&self, documents: Vec<Document>) -> Result<(), SearchError> {
        Meilisearch::upsert(self, documents).await
    }

    async fn delete(&self, ids: Vec<String>) -> Result<(), SearchError> {
        Meilisearch::delete(self, ids).await
    }
}

#[cfg(test)]
mod tests;
