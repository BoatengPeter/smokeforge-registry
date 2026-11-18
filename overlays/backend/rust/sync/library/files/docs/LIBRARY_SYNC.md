# Library sync API

The Rust backend exposes one versioned, user-scoped Library resource for desktop
offline sync.

- `GET /api/v1/library/snapshot`
- `GET /api/v1/library/changes?cursor=&limit=`
- `GET /api/v1/library/{id}`
- `POST /api/v1/library` with `Idempotency-Key`
- `PATCH /api/v1/library/{id}` with `If-Match` and `Idempotency-Key`
- `DELETE /api/v1/library/{id}` with `If-Match` and `Idempotency-Key`

The SQLx migration is `0005_library_sync.sql`. Versions increment on mutations;
stale versions return `409`. The change feed is append-only and includes
tombstones. Idempotency records are scoped per user and expire after 24 hours.
