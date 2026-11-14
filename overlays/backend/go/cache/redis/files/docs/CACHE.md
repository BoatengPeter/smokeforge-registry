# Redis cache

The `cache:redis` overlay adds a small, fail-open cache boundary backed by
Redis.

## Behavior

- Cache is an optimization. Redis downtime must not stop the API.
- Startup performs one best-effort `Ping` and logs a warning on failure.
- `/readyz` does not check Redis.
- Every write requires a positive TTL.
- Keys are namespaced as `<prefix>:<key>`. `REDIS_KEY_PREFIX` defaults to
  `SERVICE_NAME`.
- Cache operations return errors; callers decide whether to log and fall back to
  the source of truth.
- The adapter never uses `KEYS`, `FLUSHALL`, or unbounded scans.

## Cache API

```go
type Store interface {
    Get(ctx context.Context, key string) ([]byte, bool, error)
    Set(ctx context.Context, key string, value []byte, ttl time.Duration) error
    Delete(ctx context.Context, key string) error
    Ping(ctx context.Context) error
}
```

JSON helpers are available for the common case:

```go
value, found, err := cache.GetJSON[Account](ctx, cacheStore, "account:123")
if err != nil {
    // Cache failed or the stored JSON is invalid. Log and load from the source.
}

if found {
    return value, nil
}
```

```go
if err := cache.SetJSON(ctx, cacheStore, "account:123", account, 5*time.Minute); err != nil {
    // Log and continue; a failed cache write must not fail the request.
}
```

Pass the store into the service or adapter that owns the cached data. Do not put
Redis types in domain code.

## TTL and keys

Use TTLs based on tolerating stale data:

| Data | Suggested TTL |
| --- | --- |
| API/account response cache | 5-15 minutes |
| Reference data | 1 hour to 1 week |
| Short-lived derived data | Seconds to minutes |

Use stable, specific keys such as `account:123` or `catalog:category:books`.
Avoid high-cardinality keys that are never read again. Do not use `KEYS`.

## Local development

Run the API on the host with Redis in Docker:

```bash
make install
make cache-up
make run
```

Useful commands:

```bash
make cache-down
make cache-shell
```

Default local configuration:

```dotenv
REDIS_URL=redis://localhost:6379/0
REDIS_KEY_PREFIX=
REDIS_DIAL_TIMEOUT=5s
REDIS_READ_TIMEOUT=3s
REDIS_WRITE_TIMEOUT=3s
REDIS_POOL_SIZE=10
REDIS_MIN_IDLE_CONNS=1
REDIS_MAX_RETRIES=3
REDIS_PORT=6379
```

`REDIS_PORT` controls the Compose host port. Update `REDIS_URL` when changing it.
`REDIS_URL` supports passwords, database selection, and `rediss://` TLS URLs.

## Full Docker stack

When the Docker overlay is also selected, `make docker-up` merges
`compose.override.yaml`, `compose.override.cache.yaml`, and any other
`compose.override.*.yaml` files. The API connects to Redis at
`redis://redis:6379/0`.

Redis runs without persistence and uses `allkeys-lru` eviction. It is a cache,
not a durable store. Do not use this Compose setup for sessions or job queues.

## Invalidation

Delete the exact key that changed:

```go
if err := cacheStore.Delete(ctx, "account:123"); err != nil {
    // Log; the TTL remains a backstop.
}
```

Tag-based invalidation and cache stampede protection are intentionally not
included. For hot-key stampedes, add `singleflight` or a Redis lock in the
feature that owns the data.

## Security

- Never commit `.env` or Redis credentials.
- Use `rediss://` or a private network in production.
- Keep Redis off the public internet.
- Scope credentials per environment.
- Do not store secrets or large blobs in the cache.

## Future overlays

Distributed rate limiting, Redis-backed sessions, and distributed locks are
separate overlays. They can depend on `cache:redis` but must define their own
failure and consistency rules.
