# Redis cache

`internal::cache::redis::RedisCache` provides async `get`, `set`, `delete`, and
`ping` methods with a key prefix and TTLs.

```bash
make cache-up
make cache-down
```

The API logs a warning when Redis is unavailable at startup; cache usage is
opt-in per feature.
