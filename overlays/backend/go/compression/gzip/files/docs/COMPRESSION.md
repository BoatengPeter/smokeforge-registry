# Queue compression

The `compression:gzip` overlay compresses job envelopes above a configurable
threshold using gzip. Small jobs remain plain JSON.

## Behavior

- Jobs below `QUEUE_COMPRESSION_MIN_BYTES` are published as JSON.
- Larger jobs are gzip-compressed before publishing.
- The consumer detects gzip by its magic header and decompresses safely.
- Decompressed output is bounded by
  `QUEUE_COMPRESSION_MAX_DECOMPRESSED_BYTES`.
- The RabbitMQ max message size applies to the compressed body.
- Compression is a transport codec; handler payloads remain JSON.

## Configuration

```text
QUEUE_COMPRESSION_MIN_BYTES=1024
QUEUE_COMPRESSION_MAX_DECOMPRESSED_BYTES=1048576
```

Keep `QUEUE_COMPRESSION_MAX_DECOMPRESSED_BYTES` aligned with your
`RABBITMQ_MAX_MESSAGE_BYTES` policy. A compressed message may be small while its
uncompressed representation is large.

## Operations

- Compression adds CPU cost; use it for measured large-payload workloads.
- Prefer claim-check/object storage for very large payloads.
- Monitor decompression errors and message-size rejections.
- Do not treat compression as encryption.
