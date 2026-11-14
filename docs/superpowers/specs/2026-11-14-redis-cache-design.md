# Redis Cache Overlay Design

**Status:** Approved design

**Goal:** Add an optional `cache:redis` backend overlay that gives generated Go APIs
a small, production-safe Redis cache boundary without coupling application code to
`go-redis`.

**Non-goals:** Distributed rate limiting, auth/session storage, distributed locks,
Redis Streams, Pub/Sub, HTTP response caching middleware, Redis Cluster/Sentinel
operations, and automatic cache stampede protection.

## Decisions

- Cache is fail-open. Redis downtime must not prevent API startup or make
  `/readyz` fail.
- Startup performs one best-effort `Ping` and logs a warning on failure.
- Cache operations return errors; callers decide whether to ignore them and fall
  back to the source of truth.
- TTL is mandatory for every write.
- Keys are namespaced with `REDIS_KEY_PREFIX`, defaulting to `SERVICE_NAME`.
- The public cache API is byte-oriented with generic JSON helpers.
- Redis is available locally through `compose.cache.yaml` and through the full
  Docker stack through `compose.override.cache.yaml`.
- No demo endpoint is added. The scaffold proves wiring through initialization,
  the startup ping, shutdown cleanup, docs, and tests.

## Package layout

```text
internal/cache/
  cache.go            # Store port and JSON helpers
  cache_test.go
  redis/
    store.go          # go-redis adapter
    store_test.go     # miniredis tests
```

### Cache port

```go
type Store interface {
    Get(ctx context.Context, key string) ([]byte, bool, error)
    Set(ctx context.Context, key string, value []byte, ttl time.Duration) error
    Delete(ctx context.Context, key string) error
    Ping(ctx context.Context) error
}
```

JSON helpers:

```go
func GetJSON[T any](ctx context.Context, store Store, key string) (T, bool, error)
func SetJSON[T any](ctx context.Context, store Store, key string, value T, ttl time.Duration) error
```

- Missing keys return `found=false` and a nil error.
- Invalid keys, non-positive TTLs, and JSON failures return wrapped errors.
- `GetJSON` returns `found=true` with the decode error when the value is malformed.
- Key validation happens before writes or reads.
- The adapter never uses `KEYS`, `FLUSHALL`, or unbounded scans.

### Redis adapter

```go
func New(cfg config.Config) (*Store, error)
func (s *Store) Close() error
```

Configuration:

```text
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

- `REDIS_URL` supports password, database selection, and `rediss://` TLS URLs.
- `REDIS_KEY_PREFIX` defaults to `SERVICE_NAME`.
- Pool and timeout values must be positive; URL scheme must be `redis` or `rediss`.
- The key format is `<prefix>:<caller-key>`.
- The cache is intentionally not a readiness dependency.

## Dependencies

- `github.com/redis/go-redis/v9 v9.23.0`
- `github.com/alicebob/miniredis/v2 v2.40.0` for adapter tests

## Application lifecycle

`cmd/api/main.go` gets two new anchors: `import-cache` and `main-cache`.

```go
cacheStore, err := rediscache.New(cfg)
if err != nil {
    return fmt.Errorf("create cache: %w", err)
}

defer func() {
    _ = cacheStore.Close()
}()

if err := cacheStore.Ping(ctx); err != nil {
    logger.Warn("cache unavailable; continuing without cache", zap.Error(err))
}
```

- The cache is created after the database when both are selected.
- Startup does not fail when Redis is unreachable.
- Feature code can receive `cacheStore` through its constructor when a feature
  starts using it.
- `/readyz` remains unchanged.

## Docker and local development

### Cache-only

`compose.cache.yaml`:

- `redis:8-alpine`
- no persistence: RDB snapshots and AOF disabled
- `--maxmemory-policy allkeys-lru`
- host port `${REDIS_PORT:-6379}:6379`
- health check using `redis-cli ping`

Make targets:

```text
make cache-up
make cache-down
make cache-shell
```

### Full Docker stack

`compose.override.cache.yaml` adds:

- the same Redis service
- `REDIS_URL=redis://redis:6379/0` for the API
- API dependency on Redis health

The Docker overlay Dockerfile/Makefile continues to be usable without cache. The
Docker Makefile merges:

```text
compose.yaml
compose.override.yaml
compose.override.*.yaml
```

This lets database and cache overrides coexist and keeps `make docker-down`
aware of every selected service.

## Manifest and preset behavior

Add feature:

```yaml
- id: cache
  name: Cache
  kinds: [backend]
  variants:
    - id: redis
      name: Redis
      default: true
      stacks: [go/gin-flat, go/gin-layered, go/gin-hexagonal, go/gin-ddd]
```

- `cache:redis` has no required companion.
- The `api-custom` preset description includes cache.
- `api` remains Postgres + JWT only.
- Future `ratelimit:redis`, `session:redis`, and lock overlays require
  `cache:redis` when they are added.

## Documentation

Add `docs/CACHE.md` covering:

- cache-aside usage with `GetJSON`/`SetJSON`
- cache-aside with byte values
- TTL guidance
- key naming and prefix behavior
- fail-open semantics and why Redis is not in `/readyz`
- local cache-only commands
- full Docker behavior
- invalidation and stampede guidance
- security notes for Redis URLs and credentials

Update `docs/README.md` and `docs/ARCHITECTURE.md` so the cache is discoverable.

## Testing and verification

Unit tests:

- JSON helper round-trip
- missing key
- malformed JSON
- invalid key
- non-positive TTL
- Redis adapter get/set/delete
- key prefixing
- TTL expiry
- ping
- miniredis-backed behavior

Verification script:

- generate cache-only project and run install/test/vet/build
- generate full project with `cache:redis` and Docker, and verify Compose includes
  `redis`
- verify `compose.cache.yaml` parses
- verify generated docs links
- run the full four-stack matrix

No external Redis is required in CI; miniredis covers the adapter.

## Acceptance criteria

- Selecting `cache:redis` adds a compilable cache package and docs.
- Startup never fails when Redis is down; it logs a warning.
- `/readyz` is unchanged.
- Cache writes require positive TTLs.
- Keys are namespaced.
- Redis is not persisted in the local cache Compose service.
- Cache and database Compose overrides work together.
- `make cache-up`, `make cache-down`, and `make cache-shell` work.
- The full template verification matrix passes with cache-only and
  cache-plus-database-plus-Docker projects.
