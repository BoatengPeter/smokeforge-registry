# Offline editing design

Offline write support is a separate `offline:outbox` feature. It is not enabled
by the base desktop storage overlay.

## Flow

```mermaid
sequenceDiagram
    participant U as User
    participant R as Renderer
    participant M as Main
    participant Q as SQLite outbox
    participant A as API
    U->>R: Save document
    R->>M: typed IPC mutation
    M->>Q: insert idempotent outbox row
    M-->>R: optimistic result
    M->>A: POST /api/v1/...
    alt success
        A-->>M: server record/version
        M->>Q: mark delivered
    else offline / retryable
        M->>Q: keep pending with backoff
    end
```

## Required rules

- The API remains the source of truth.
- Generate stable client IDs before the first write.
- Send an `Idempotency-Key` per mutation.
- Persist ordered outbox rows with attempt count, next retry, and last error.
- Define a conflict policy before enabling writes: server-wins, client-wins, or
  field-level merge.
- Namespace local data and outbox rows by user; clear them on logout.
- Never store access or refresh tokens in SQLite.
- Keep retries bounded and surface permanent failures to the UI.
- Do not run a background sync from the renderer.

## Mermaid conflict decision

```mermaid
flowchart TD
    Write[Remote write] --> Conflict{Server version changed?}
    Conflict -->|No| Apply[Apply and mark delivered]
    Conflict -->|Yes| Policy{Conflict policy}
    Policy -->|Server wins| Reject[Drop local mutation and show diff]
    Policy -->|Client wins| Force[Retry with expected version]
    Policy -->|Manual| Review[Show conflict UI]
```
