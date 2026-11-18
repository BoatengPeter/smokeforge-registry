# Offline editing

Offline support is implemented by the desktop outbox and the Go/Rust Library
sync API. See [`SYNC.md`](SYNC.md), [`CONFLICTS.md`](CONFLICTS.md), and
[`IDEMPOTENCY.md`](IDEMPOTENCY.md).

The API remains the source of truth. SQLite is the offline working copy and the
outbox is the durable delivery queue. The renderer never writes to the outbox or
calls the API directly.
