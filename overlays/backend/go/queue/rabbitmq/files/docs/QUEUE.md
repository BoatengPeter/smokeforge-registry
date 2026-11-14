# RabbitMQ worker and scheduler

The `queue:rabbitmq` overlay adds a durable RabbitMQ publisher, a bounded worker
pool, jittered retries, a dead-letter queue, and a single-instance cron
scheduler.

## Topology

```text
exchange:  <RABBITMQ_EXCHANGE>          direct, durable
queue:     <RABBITMQ_QUEUE>             quorum, durable
dlq:       <RABBITMQ_DLQ>               quorum, durable
retry:     <RABBITMQ_QUEUE>.retry.1s    classic, durable
retry:     <RABBITMQ_QUEUE>.retry.5s    classic, durable
retry:     <RABBITMQ_QUEUE>.retry.30s   classic, durable
```

The main queue dead-letters to `<RABBITMQ_EXCHANGE>.dlx`. Retry queues
dead-letter back to the main exchange. Every message is persistent and every
publish waits for a publisher confirmation.

## Delivery and retries

- Workers use manual acknowledgments.
- A successful handler → ack.
- A failed handler → publish to the retry queue, then ack the original.
- Retry delays are 1s, 5s, and 30s with ±20% jitter.
- After `RABBITMQ_MAX_RETRIES`, the job goes to the DLQ.
- Malformed jobs or unknown job types go to the DLQ.
- If retry/DLQ publication fails, the original message is nacked and requeued.
- Handlers must be idempotent. RabbitMQ and this worker provide at-least-once
  delivery, not exactly-once processing.

## Worker pool

`WORKER_CONCURRENCY` workers consume the same queue. Each worker has its own
AMQP channel because channels must not be used concurrently. Each channel uses
`RABBITMQ_PREFETCH` as its QoS prefetch value.

The worker process reconnects with bounded backoff and recreates channels after
connection loss. Shutdown stops new deliveries and waits for in-flight handlers.

## Built-in heartbeat

The scheduler publishes a `system.heartbeat` job at
`SCHEDULER_HEARTBEAT` (default `@every 1m`). The worker logs each heartbeat.
Use it to verify the scheduler → queue → worker-pool path before adding product
jobs.

Add product handlers in `cmd/worker/main.go`:

```go
if err := registry.Register("my.job", myHandler); err != nil {
    return fmt.Errorf("register my job: %w", err)
}
```

## Single scheduler instance

The first queue overlay runs the scheduler as a **single instance**. Do not
scale the scheduler service to more than one replica. A later scheduler
leadership overlay adds a distributed Redis lease and automatic failover.

## Configuration

```text
RABBITMQ_URL=amqp://smokeforge:smokeforge@localhost:5672/
RABBITMQ_EXCHANGE=smokeforge.jobs
RABBITMQ_QUEUE=smokeforge.jobs
RABBITMQ_DLQ=smokeforge.jobs.dlq
RABBITMQ_PREFETCH=10
RABBITMQ_MAX_RETRIES=3
RABBITMQ_MAX_MESSAGE_BYTES=262144
RABBITMQ_CONNECT_TIMEOUT=5s
RABBITMQ_CONNECT_RETRIES=5
RABBITMQ_CONNECT_BACKOFF=1s
RABBITMQ_HEARTBEAT=30s
RABBITMQ_PUBLISH_TIMEOUT=5s
WORKER_CONCURRENCY=4
SCHEDULER_HEARTBEAT=@every 1m
RABBITMQ_PORT=5672
RABBITMQ_MANAGEMENT_PORT=15672
```

Large payloads should use a claim-check pattern: store the payload in object
storage and publish only a reference. Compression is intentionally not enabled
by default.

## Local development

```bash
make install
make queue-up
make worker
make scheduler
```

RabbitMQ management UI: <http://localhost:15672>.

Useful commands:

```bash
make queue-down
make queue-shell
```

## Full Docker stack

When Docker is also selected, `make docker-up` includes RabbitMQ, worker, and
scheduler. The scheduler remains single-instance.

```bash
make docker-up
docker compose -f compose.yaml -f compose.override.yaml -f compose.override.queue.yaml logs -f worker scheduler
```

## Operational checks

- Watch queue depth, unacked messages, publish/deliver/ack rates, and DLQ depth.
- Alert when the DLQ receives messages or queue depth grows without draining.
- Use the management UI/API or export RabbitMQ Prometheus metrics.
- Rotate RabbitMQ credentials and use TLS (`amqps://`) outside local development.
- Keep RabbitMQ off the public internet.

## Deferred overlays

- `scheduler:leader` — Redis lease and leader election.
- `queue:dedupe` — explicit idempotency/deduplication policy.
- `queue:compression` — opt-in compression with strict size limits.
