# Library sync API

The Go backend exposes one versioned, user-scoped Library resource for desktop
offline sync. All paths require `Authorization: Bearer <access-token>`.

## Contract

- `GET /api/v1/library/snapshot` returns active items and the change cursor.
- `GET /api/v1/library/changes?cursor=&limit=` returns ordered changes.
- `POST /api/v1/library` requires `Idempotency-Key`.
- `PATCH /api/v1/library/{id}` requires `If-Match` and `Idempotency-Key`.
- `DELETE /api/v1/library/{id}` requires `If-Match` and `Idempotency-Key`.

Versions increment on every mutation. A stale `If-Match` returns `409`; clients
must apply their configured conflict policy. The `library_changes` table is an
append-only feed with tombstones. Deletes are soft.

Idempotency records live for 24 hours and are unique per user and key. Reusing a
key with a different request hash returns `409 idempotency_key_reused`.

The `library` PostgreSQL migration is `00006_library_sync.sql`; it depends on the
users table from the JWT auth migration.
