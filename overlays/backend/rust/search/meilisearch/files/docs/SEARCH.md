# Search

Meilisearch provides a public `GET /api/v1/search?q=...&limit=20&offset=0`
endpoint. Applications index documents through `search::Meilisearch::upsert`
and `delete`; there is no document CRUD endpoint.

Only public documents belong in the configured index. The API returns `503`
when Meilisearch is unavailable and an empty result when the index does not exist.
