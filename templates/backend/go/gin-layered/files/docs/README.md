# Documentation

Start here:

- [Architecture](ARCHITECTURE.md) — package boundaries, process lifecycle,
  dependency injection, and middleware order.
- [API contract and versioning](API.md) — OpenAPI, Swagger, and breaking-change
  rules.

Feature overlays add their own documents when selected:

- `AUTH.md` — users, JWT sessions, email flows, and Google OAuth.
- `TOTP.md` — two-factor enrollment, recovery codes, and login flow.
- `DATABASE.md` — PostgreSQL setup, migrations, pool settings, and readiness.
- `CACHE.md` — Redis cache API, TTLs, fail-open behavior, and local/full Docker usage.
- `DOCKER.md` — full-stack Compose and the local-API/Docker-database workflow.
- `CI.md` — GitHub Actions checks and integration-test notes.

The project root [README](../README.md) covers quick start, environment
variables, and commands. [CONTEXT](../CONTEXT.md) defines engineering rules and
the quality gate.
