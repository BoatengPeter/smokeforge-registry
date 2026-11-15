# Scheduler leadership

The `scheduler:leader` overlay makes the scheduler safe to run with multiple
replicas by electing one leader with a Redis lease.

## Requirements

- `queue:rabbitmq`
- `cache:redis`

## Behavior

- Each scheduler uses `SET key token NX PX ttl` with a unique UUID token.
- Only the lease owner runs the scheduler ticker.
- The leader renews the lease in the background with a Lua compare-and-expire
  check.
- If renewal fails or the lease is lost, the scheduler cancels the ticker and
  retries acquisition.
- Shutdown releases the lease with a Lua compare-and-delete check.
- The lease TTL is the failover window; renew must be shorter than the TTL.

## Configuration

```text
SCHEDULER_LEASE_TTL_SECONDS=30
SCHEDULER_LEASE_RENEW_SECONDS=10
SCHEDULER_LEASE_RETRY_SECONDS=5
```

The lock key is `<REDIS_KEY_PREFIX or SERVICE_NAME>:scheduler:leader`.

## Operations

- Run any number of scheduler replicas; only one is active.
- Keep the Redis instance used for leases highly available if scheduler failover
  matters.
- Do not reuse the fail-open cache semantics for leadership. Lease errors are
  treated as loss of leadership.
- Alert on repeated leadership loss or Redis errors.
