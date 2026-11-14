# Redis deduplication

The `dedupe:redis` overlay records successfully processed job IDs so that
redelivered messages can be acknowledged without running handlers again.

## Requirements

- `queue:rabbitmq`
- `cache:redis`

## Behavior

- The dedupe marker key is `<REDIS_KEY_PREFIX or SERVICE_NAME>:dedupe:<job id>`.
- A job is marked only after its handler succeeds.
- A redelivery with an existing marker is acknowledged without running the
  handler.
- `DEDUPE_TTL` defaults to 24 hours. Size it above the maximum retry and replay
  window.
- Redis errors while checking a marker nack-requeue the job. Dedupe is
  fail-closed because correctness depends on the marker.
- If writing a marker fails after a successful handler, the job is acknowledged
  and the failure is logged. The handler may run again on a later duplicate;
  handlers must remain idempotent.
- A crash after a side effect but before writing the marker can still cause
  reprocessing. This overlay provides practical deduplication, not exactly-once
  execution.

## Configuration

```text
DEDUPE_TTL=24h
```

## Operations

- Monitor mark/check errors and Redis availability.
- Use a Redis instance with appropriate durability for your dedupe window.
- Do not rely on dedupe as a replacement for idempotent handlers or database
  constraints.
