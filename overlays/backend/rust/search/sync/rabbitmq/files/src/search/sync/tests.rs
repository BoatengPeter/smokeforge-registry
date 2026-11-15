use std::sync::{Arc, Mutex};

use async_trait::async_trait;

use super::*;
use crate::{
    queue::{Job, Publisher, QueueError, Registry},
    search::{Document, Indexer, SearchError},
};

#[derive(Default)]
struct FakePublisher {
    jobs: Mutex<Vec<Job>>,
}

#[async_trait]
impl Publisher for FakePublisher {
    async fn publish(&self, job: Job) -> Result<(), QueueError> {
        self.jobs.lock().unwrap().push(job);
        Ok(())
    }
}

#[derive(Default)]
struct FakeIndexer {
    documents: Mutex<Vec<Document>>,
    ids: Mutex<Vec<String>>,
    fail: bool,
}

impl Indexer for FakeIndexer {
    async fn upsert(&self, documents: Vec<Document>) -> Result<(), SearchError> {
        if self.fail {
            return Err(SearchError::Unavailable("engine down".to_owned()));
        }

        *self.documents.lock().unwrap() = documents;
        Ok(())
    }

    async fn delete(&self, ids: Vec<String>) -> Result<(), SearchError> {
        if self.fail {
            return Err(SearchError::Unavailable("engine down".to_owned()));
        }

        *self.ids.lock().unwrap() = ids;
        Ok(())
    }
}

fn document(id: &str, title: &str) -> Document {
    let mut fields = serde_json::Map::new();
    fields.insert("title".to_owned(), serde_json::json!(title));

    Document {
        id: id.to_owned(),
        fields,
    }
}

#[test]
fn job_construction_uses_kinds_and_payloads() {
    let upsert = new_job(
        UPSERT_JOB_TYPE,
        &UpsertPayload {
            documents: vec![document("doc-1", "Design")],
        },
    )
    .unwrap();

    assert_eq!(upsert.kind, UPSERT_JOB_TYPE);

    let payload: UpsertPayload = serde_json::from_value(upsert.payload).unwrap();
    assert_eq!(payload.documents.len(), 1);
    assert_eq!(payload.documents[0].id, "doc-1");
    assert_eq!(
        payload.documents[0].fields.get("title"),
        Some(&serde_json::json!("Design"))
    );

    let delete = new_job(
        DELETE_JOB_TYPE,
        &DeletePayload {
            ids: vec!["doc-1".to_owned(), "doc-2".to_owned()],
        },
    )
    .unwrap();

    assert_eq!(delete.kind, DELETE_JOB_TYPE);

    let payload: DeletePayload = serde_json::from_value(delete.payload).unwrap();
    assert_eq!(payload.ids, ["doc-1", "doc-2"]);
}

#[tokio::test]
async fn queue_indexer_publishes_upsert_job() {
    let publisher = Arc::new(FakePublisher::default());
    let indexer = QueueIndexer::new(Arc::clone(&publisher));

    Indexer::upsert(&indexer, vec![document("doc-1", "Design")])
        .await
        .unwrap();

    let jobs = publisher.jobs.lock().unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].kind, UPSERT_JOB_TYPE);

    let payload: UpsertPayload = serde_json::from_value(jobs[0].payload.clone()).unwrap();
    assert_eq!(payload.documents.len(), 1);
    assert_eq!(payload.documents[0].id, "doc-1");
}

#[tokio::test]
async fn queue_indexer_publishes_delete_job() {
    let publisher = Arc::new(FakePublisher::default());
    let indexer = QueueIndexer::new(Arc::clone(&publisher));

    Indexer::delete(&indexer, vec!["doc-1".to_owned(), "doc-2".to_owned()])
        .await
        .unwrap();

    let jobs = publisher.jobs.lock().unwrap();
    assert_eq!(jobs.len(), 1);
    assert_eq!(jobs[0].kind, DELETE_JOB_TYPE);

    let payload: DeletePayload = serde_json::from_value(jobs[0].payload.clone()).unwrap();
    assert_eq!(payload.ids, ["doc-1", "doc-2"]);
}

#[tokio::test]
async fn queue_indexer_skips_empty_batches() {
    let publisher = Arc::new(FakePublisher::default());
    let indexer = QueueIndexer::new(Arc::clone(&publisher));

    Indexer::upsert(&indexer, Vec::new()).await.unwrap();
    Indexer::delete(&indexer, Vec::new()).await.unwrap();

    assert!(publisher.jobs.lock().unwrap().is_empty());
}

#[tokio::test]
async fn registered_upsert_handler_applies_documents() {
    let indexer = Arc::new(FakeIndexer::default());
    let mut registry = Registry::new();
    register(&mut registry, Arc::clone(&indexer));

    let handler = registry.lookup(UPSERT_JOB_TYPE).unwrap();
    let job = Job::new(
        UPSERT_JOB_TYPE,
        serde_json::json!({"documents": [{"id": "doc-1", "title": "Design"}]}),
    );

    handler.handle(job).await.unwrap();

    let documents = indexer.documents.lock().unwrap();
    assert_eq!(documents.len(), 1);
    assert_eq!(documents[0].id, "doc-1");
    assert_eq!(
        documents[0].fields.get("title"),
        Some(&serde_json::json!("Design"))
    );
}

#[tokio::test]
async fn registered_delete_handler_applies_ids() {
    let indexer = Arc::new(FakeIndexer::default());
    let mut registry = Registry::new();
    register(&mut registry, Arc::clone(&indexer));

    let handler = registry.lookup(DELETE_JOB_TYPE).unwrap();
    let job = Job::new(
        DELETE_JOB_TYPE,
        serde_json::json!({"ids": ["doc-1", "doc-2"]}),
    );

    handler.handle(job).await.unwrap();

    assert_eq!(*indexer.ids.lock().unwrap(), ["doc-1", "doc-2"]);
}

#[tokio::test]
async fn handlers_return_errors_from_the_indexer() {
    let indexer = Arc::new(FakeIndexer {
        fail: true,
        ..FakeIndexer::default()
    });
    let mut registry = Registry::new();
    register(&mut registry, Arc::clone(&indexer));

    let upsert = registry.lookup(UPSERT_JOB_TYPE).unwrap();
    assert!(
        upsert
            .handle(Job::new(
                UPSERT_JOB_TYPE,
                serde_json::json!({"documents": [{"id": "doc-1"}]}),
            ))
            .await
            .is_err()
    );

    let delete = registry.lookup(DELETE_JOB_TYPE).unwrap();
    assert!(
        delete
            .handle(Job::new(
                DELETE_JOB_TYPE,
                serde_json::json!({"ids": ["doc-1"]}),
            ))
            .await
            .is_err()
    );
}

#[tokio::test]
async fn handlers_reject_invalid_payloads() {
    let indexer = Arc::new(FakeIndexer::default());
    let mut registry = Registry::new();
    register(&mut registry, Arc::clone(&indexer));

    let upsert = registry.lookup(UPSERT_JOB_TYPE).unwrap();
    assert!(
        upsert
            .handle(Job::new(
                UPSERT_JOB_TYPE,
                serde_json::json!({"documents": "not-an-array"}),
            ))
            .await
            .is_err()
    );

    let delete = registry.lookup(DELETE_JOB_TYPE).unwrap();
    assert!(
        delete
            .handle(Job::new(DELETE_JOB_TYPE, serde_json::json!({"ids": 42}),))
            .await
            .is_err()
    );
}
