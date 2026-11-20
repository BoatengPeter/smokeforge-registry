# Sync

The desktop app is local-first. Every Library mutation is persisted locally and
added to the durable outbox. Sync runs in a background worker and never blocks
the GPUI render loop.

```mermaid
sequenceDiagram
    participant View
    participant SQLite
    participant Sync as Sync worker
    participant API

    View->>SQLite: Local mutation + outbox insert
    Sync->>SQLite: Read due outbox
    Sync->>API: Idempotency-Key + If-Match
    alt accepted
        API-->>Sync: Stored record
        Sync->>SQLite: Acknowledge outbox
    else conflict
        API-->>Sync: 409
        Sync->>SQLite: Record manual conflict
    else transient error
        API-->>Sync: 408/429/5xx or network
        Sync->>SQLite: Retry with backoff
    end
    Sync->>API: Pull changes after cursor
    Sync->>SQLite: Apply remote upserts/tombstones
    Sync->>SQLite: Advance cursor
```

Rules:

- Local mutation and outbox insertion should remain one transaction.
- Pending local outbox rows are never overwritten by a pull.
- Conflicts are never silently resolved; the user chooses Keep local or Use
  server.
- Signing out stops sync and clears the in-memory access token.
- The GPUI view must not call the API or write the outbox directly.

The standalone preset is local-only. Fullstack API mode enables the sync path.
