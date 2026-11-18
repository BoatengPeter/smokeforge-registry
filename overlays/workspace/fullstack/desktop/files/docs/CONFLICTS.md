# Sync conflicts

Manual resolution is the default for user-authored Library records. A conflict
is recorded only when the local base version no longer matches the server
version.

## State

The local row becomes `sync_status = 'conflict'`. The outbox row is retained but
is not retried. A `sync_conflicts` row stores the local payload and the latest
server snapshot. The Library inspector shows a conflict panel with:

- **Keep local**: reapply the local edit against the current server version with
  a new idempotency key.
- **Use server**: discard the pending local mutation and replace the local row
  with the server snapshot.

## Policies

| Policy | Behavior | Use |
| --- | --- | --- |
| Manual | Wait for the user | Default for editable records |
| Server-wins | Replace local, discard outbox | Server-owned/system fields |
| Client-wins | Reapply local against newest version | Explicit single-user preference |

## Rules

- Never resolve a conflict by silently retrying the original idempotency key.
- Never apply a remote change over a pending local mutation without recording a
  conflict.
- Preserve the server version with every conflict.
- Deletions are conflicts too: a local delete can lose to a remote update.
