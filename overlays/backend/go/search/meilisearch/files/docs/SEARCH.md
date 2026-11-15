# Search

The search overlay adds a public full-text query endpoint backed by
Meilisearch. Indexing is a Go API, not a public HTTP route.

## Endpoint

```http
GET /api/v1/search?q=design&limit=20&offset=0
```

`q` is required and limited to 200 characters. `limit` defaults to 20 and is
capped by `SEARCH_MAX_LIMIT`; `offset` is capped at 1000.

```json
{
  "query": "design",
  "hits": [{ "id": "doc-1", "title": "Design systems" }],
  "estimated_total_hits": 1,
  "limit": 20,
  "offset": 0,
  "processing_time_ms": 3
}
```

The endpoint is public and uses the existing rate limit, timeout, CORS, request
ID, and RFC 9457 error handling. `503 search_unavailable` means Meilisearch could
not serve the request.

## Indexing

Inject `search.Indexer` where your domain data changes:

```go
err := indexer.Upsert(ctx, []search.Document{{
	ID:     "doc-1",
	Fields: map[string]any{"title": "Design systems"},
}})
```

Delete documents by ID:

```go
err := indexer.Delete(ctx, []string{"doc-1"})
```

The first upsert creates the index with `id` as the primary key. Meilisearch
accepts indexing tasks asynchronously; the call returns after Meilisearch accepts
the task, not after the task completes.

Only public documents belong in the configured index because the query endpoint
is unauthenticated. There is no document CRUD API.

## Configuration

| Variable | Default | Purpose |
| --- | --- | --- |
| `SEARCH_PORT` | `7700` | Docker host port |
| `SEARCH_URL` | `http://localhost:7700` | API-to-Meilisearch URL |
| `SEARCH_INDEX` | `documents` | Index UID |
| `SEARCH_API_KEY` | generated secret | API key |
| `SEARCH_TIMEOUT` | `5s` | Upstream request timeout |
| `SEARCH_MAX_LIMIT` | `50` | Maximum page size |

## Commands

```bash
make search-up
make search-logs
make search-down
```

The API starts even when Meilisearch is unavailable. Search requests return 503
until the engine is reachable. A first search before any documents exist returns
an empty result page.
