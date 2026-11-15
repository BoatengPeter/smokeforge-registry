# Redis cache

`cache::Store` is the cache boundary. `cache::redis::RedisCache` implements it
with async `get`, `set`, `delete`, and `ping` methods.

- `cache::get_json` and `cache::set_json` encode typed JSON values.
- Keys must not be empty.
- Writes require an explicit positive TTL.
- The API logs a warning when Redis is unavailable at startup; cache usage stays
  opt-in per feature.
- Do not cache secrets or authorization decisions without a deliberate policy.

```bash
make cache-up
make cache-down
```
