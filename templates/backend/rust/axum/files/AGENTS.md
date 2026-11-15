# AGENTS.md — generated Rust API

This project was scaffolded by smokeforge. It is a single-crate Axum API with
domain-oriented modules. Read [CONTEXT.md](CONTEXT.md) before changing code.

## Commands

```bash
make install
make run
make test
make fmt-check
make clippy
make check
make build
```

## Layout

```text
src/main.rs        process startup and graceful shutdown
src/lib.rs         crate composition, router, OpenAPI
src/config.rs      figment + dotenv configuration
src/error.rs       RFC 9457 problem details
src/state.rs       cheaply cloneable AppState and readiness
src/health/        liveness and readiness routes
src/db/            sqlx pool and database adapters (overlay)
src/auth/          JWT auth service and routes (overlay)
tests/             integration tests and shared helpers
```

Do not create `handlers/`, `services/`, or `models/` kind directories. Organize
new modules by use case. Keep handlers thin; put application logic in services
and infrastructure behind traits where a consumer needs substitution.

## Project-scoped skills

Read the matching `SKILL.md` under `.agents/skills` and `.claude/skills` before
changing code:

- `rust-best-practices-apollo` — Apollo's idiomatic Rust guidance.
- `rust-best-practices-beautifulrust` — comprehensive Rust style, API, and
  design guidance.
- `rust-async-patterns` — Tokio, async traits, cancellation, and concurrency.
- `rust-testing` — unit, integration, async, property, and coverage patterns.
- `rust-patterns` — ownership, errors, traits, and domain module structure.

`skills-lock.json` records sources and content hashes.
