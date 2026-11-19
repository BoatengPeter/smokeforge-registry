# Sync status

The local-first Library foundation is implemented: SQLite WAL, migrations,
soft delete/restore, import/export, backup, and explicit clear-local-data.

API-backed outbox synchronization is not yet implemented in this template. The
next slice adds:

1. `outbox` and `conflict` tables in a refinery migration.
2. Transactional local mutation plus outbox insertion.
3. `Idempotency-Key` and `If-Match` requests against
   `/api/v1/library/*`.
4. Cursor pulls with tombstones, jittered retries, and manual conflict choices.
5. Sign-out cancellation and per-user DB/sync scoping.

Do not describe the app as synced until those steps and their tests land.
