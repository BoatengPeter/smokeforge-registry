# Scheduler leadership

The `scheduler:leader` overlay makes the single scheduler process safe to run
with multiple replicas by electing one leader with a Redis lease.

## Requirements

- `queue:rabbitmq`
- `cache:redis`

## Behavior

- Each scheduler uses `SET NX PX` with a unique token.
- Only the lease owner starts cron.
- The leader renews the lease in the background using a Lua compare-and-expire
  check.
- If renewal fails or the lease is lost, the scheduler stops its cron runner and
  tries to acquire leadership again.
- Shutdown releases the lease with a Lua compare-and-delete check.
- The lease TTL is a failover window; renew must be shorter than the TTL.

## Configuration

```text
SCHEDULER_LEASE_TTL=30s
SCHEDULER_LEASE_RENEW=10s
SCHEDULER_LEASE_RETRY=5s
```

The lock key is `<REDIS_KEY_PREFIX or SERVICE_NAME>:scheduler:leader`.

## Operations

- Run any number of scheduler replicas; only one is active.
- Keep the Redis instance used for leases highly available if scheduler failover
  matters.
- Do not use the fail-open cache semantics for scheduler leadership. This
  overlay uses a dedicated Redis client and treats lease errors as loss of
  leadership.
- Alert on repeated leadership loss or Redis errors.
