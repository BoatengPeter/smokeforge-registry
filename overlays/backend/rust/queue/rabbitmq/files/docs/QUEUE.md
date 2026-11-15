# RabbitMQ queue

The Rust queue uses durable RabbitMQ queues, a JSON job envelope, a worker
registry, and a heartbeat scheduler.

```bash
make queue-up
make worker
make scheduler
```

Jobs are acknowledged after successful handling and dead-lettered on failure.
Compression and dedupe overlays can be added after the queue core.
