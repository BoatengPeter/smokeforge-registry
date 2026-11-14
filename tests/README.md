# Registry verification

`verify-templates.sh` builds generated projects from this registry and validates
them. It requires:

- `SMOKEFORGE_BIN` pointing to a built smokeforge binary
- Go 1.26+
- Python 3 for documentation link checks
- optional Docker CLI for `docker compose config`

It covers all four Go/Gin architecture bases with `api-custom` (base) and
`api` plus TOTP, cache, queue, scheduler leadership, dedupe, compression,
Docker, and CI, the database-only, cache-only, and queue-only overlays, and
generated code drift.

```bash
SMOKEFORGE_BIN=../smokeforge/bin/smokeforge ./tests/verify-templates.sh
```
