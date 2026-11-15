# RabbitMQ queue

The Rust queue uses durable RabbitMQ queues, a JSON job envelope, a worker
registry, and a heartbeat scheduler.

```bash
make queue-up
make worker
make scheduler
```

Jobs are acknowledged after successful handling. Failed jobs are republished to
1s, 5s, and 30s retry queues with an incremented attempt count. After
`RABBITMQ_MAX_RETRIES` attempts they are dead-lettered. Compression and dedupe
overlays can be added after the queue core.
