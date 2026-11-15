# RabbitMQ search index sync

This overlay makes `search.Indexer` publish jobs instead of writing to
Meilisearch synchronously. The queue worker consumes the jobs and applies them
through the Meilisearch adapter.

It requires `search:meilisearch` and `queue:rabbitmq`.

## Flow

1. Domain code calls `searchService.Upsert` or `searchService.Delete`.
2. The API publishes a confirmed `search.documents.upsert` or
   `search.documents.delete` job.
3. The worker consumes the job and writes to Meilisearch.
4. Failed jobs use the existing RabbitMQ retry and dead-letter behavior.

The query endpoint is unchanged. Indexing is eventually consistent: the publish
call returns after RabbitMQ confirms the job, not after Meilisearch finishes its
task.

## Operational notes

- The API process now opens a RabbitMQ connection and declares the existing
  topology.
- Run `make queue-up`, `make search-up`, and `make worker` for local use.
- Search indexing jobs share `RABBITMQ_QUEUE`, retry queues, and the DLQ with
  other jobs.
- Keep indexing batches below `RABBITMQ_MAX_MESSAGE_BYTES`; oversized jobs are
  rejected before publish.
- A Meilisearch outage causes worker retries and then DLQ delivery; the public
  query endpoint returns `503` while the engine is unavailable.
- No additional environment variables are introduced by this overlay.

## Job types

| Type | Payload |
| --- | --- |
| `search.documents.upsert` | `{"documents":[{"id":"doc-1","fields":{...}}]}` |
| `search.documents.delete` | `{"ids":["doc-1"]}` |

The worker registers both handlers at startup. The API does not expose an
indexing HTTP route.
