# RabbitMQ search index sync

This overlay makes `search::Indexer` publish jobs instead of writing to
Meilisearch synchronously. The queue worker consumes the jobs and applies them
through the Meilisearch adapter.

It requires `search:meilisearch` and `queue:rabbitmq`.

## Flow

1. Domain code calls `QueueIndexer::upsert` or `QueueIndexer::delete`.
2. The API publishes a durable `search.documents.upsert` or
   `search.documents.delete` job.
3. The worker consumes the job and calls `Meilisearch::upsert` or
   `Meilisearch::delete`.
4. Failed jobs use the existing RabbitMQ retry queues and dead-letter behavior.

The query endpoint is unchanged. Indexing is eventually consistent: the publish
call returns after RabbitMQ confirms the job, not after Meilisearch finishes its
task.

`SearchService` remains query-only. Construct `QueueIndexer` where domain writes
happen and use it through the `search::Indexer` trait:

```rust
use std::sync::Arc;

use my_app::{
    queue::rabbitmq::Rabbit,
    search::{Document, Indexer, sync::QueueIndexer},
};

let rabbit = Arc::new(Rabbit::connect(&config).await?);
let indexer = QueueIndexer::new(rabbit);

indexer
    .upsert(vec![Document {
        id: "doc-1".to_owned(),
        fields: [("title".to_owned(), serde_json::json!("Design systems"))]
            .into_iter()
            .collect(),
    }])
    .await?;

indexer.delete(vec!["doc-1".to_owned()]).await?;
```

Both calls are no-ops for empty batches. The worker registers both handlers at
startup, so no route or manual registration is required.

## Operational notes

- The API process must open a RabbitMQ connection when it uses `QueueIndexer`.
- Run `make queue-up`, `make search-up`, and `make worker` for local use.
- Search indexing jobs share `RABBITMQ_QUEUE` and `RABBITMQ_DLQ` with other jobs.
- A Meilisearch outage causes handler failures and dead-letter delivery; the
  public query endpoint returns `503` while the engine is unavailable.
- No additional environment variables are introduced by this overlay.

## Job types

| Type | Payload |
| --- | --- |
| `search.documents.upsert` | `{"documents":[{"id":"doc-1","title":"Design systems"}]}` |
| `search.documents.delete` | `{"ids":["doc-1"]}` |

Rust documents flatten their `fields` map, so payload fields appear next to
`id`. The worker rejects malformed payloads before calling Meilisearch.
