# smokeforge-registry

Versioned template registry for [smokeforge](https://github.com/BoatengPeter/smokeforge).

**Status:** Next.js, the four Go/Gin architectures, the Rust/Axum backend, the
full Electron app shell, the native egui desktop base, the Android Compose
production base, and optional fullstack mobile/desktop composition are implemented. The desktop shell includes React Router, shadcn/ui, Drizzle SQLite,
Library CRUD, utility-process Activity, Settings/Diagnostics, auto-update
wiring, OAuth PKCE, offline Library sync with idempotency/retries/manual
conflicts, and an optional tray. Go includes PostgreSQL,
users/JWT auth, Google OAuth, TOTP, Redis
cache, RabbitMQ queues, scheduler leadership, dedupe, gzip, image media,
Meilisearch search, RabbitMQ index sync, signed pagination, Docker, CI, Swagger
UI, embedded migrations, and project-scoped Go skills. Rust includes Axum,
SQLx/Postgres, JWT auth with email verification/reset and Google OAuth, Redis
cache, signed pagination, TOTP enrollment/login/recovery, Meilisearch search
with RabbitMQ sync, private S3/MinIO/R2 media with imgproxy transforms, RabbitMQ
worker retries/scheduler, Redis dedupe and leader election, gzip queue
compression, a Go-parity response/error layer, Docker, CI, and five Rust skills.
Contract/build-matrix automation is the remaining M3 work.

## Layout

```text
manifest.yaml
templates/
  backend/go/
    gin-flat/ gin-layered/ gin-hexagonal/ gin-ddd/
  backend/rust/
    axum/
  frontend/next/
  mobile/android/compose/             # Android Compose production base + Kotlin skills
  desktop/electron/                   # secure TypeScript Electron base
  desktop/egui/                       # native Rust egui base + component/Rust skills
  workspace/fullstack/                # root CONTEXT-MAP, Makefile, env docs
overlays/
  backend/go/                          # database, auth, cache, queue, media, search, pagination
  backend/rust/                        # database, auth, cache, queue, scheduler, dedupe, media, search, sync, pagination
  next/                                # database, auth, bff, state, data, testing, posthog, docker, ci
  workspace/fullstack/mobile/           # Android workspace wiring and root CI
  desktop/electron/                    # data, state, storage:sqlite, testing, CI
  desktop/egui/                         # egui base CI overlay
  workspace/fullstack/                 # root compose and CI aggregation
tests/                                 # contract/build matrix
```

## Current Go decisions

- Four explicit Gin stacks: flat, layered, hexagonal, and DDD.
- Shared scaffold contract: `cmd/api`, `internal/httpapi`, configuration and
  main anchors, plus self-contained feature packages.
- Auth uses a transport-neutral application core with HTTP, PostgreSQL, email,
  and Google OAuth adapters under `internal/auth/*`.
- Spec-first OpenAPI with `oapi-codegen`; Swagger UI at `/docs`.
- Gin, Zap, `caarlos0/env`, validator, RFC 9457, rate limiting, graceful
  shutdown.
- Presets: `api` (default backend: Postgres + JWT) and `api-custom` (choose features).
- Frontend presets: `fullstack` (default) and `frontend`.
- Android preset: `mobile` (Compose, Material 3, Hilt/KSP, Retrofit, encrypted
  DataStore, email/password auth, debug/staging/release variants, CI).
- Desktop presets: `desktop` (Electron + React + TypeScript, TanStack Query,
  Zustand, SQLite) and `desktop-egui` (native Rust, rusqlite/refinery, keyring,
  egui_tiles, full CI). Electron package manager is selectable (pnpm, npm, or
  bun).
- Fullstack mobile: opt in with `--mobile android/compose`; adds `apps/android`.
- Fullstack desktop: opt in with `--desktop electron`; adds the same desktop
  base with main-process auth, SQLite, and the offline Library sync outbox. The
  egui stack is available as a standalone base and will gain the same API/sync
  composition in its milestone issues.
- Fullstack preset: `fullstack-api` (Next.js + Go/Rust API, BFF auth, TanStack Query, Docker, CI).
- Rust: `rust/axum` single-crate layered base with SQLx/Postgres, JWT auth,
  Redis cache, signed pagination, TOTP login/recovery, RabbitMQ retries/search
  sync, Redis dedupe/leader election, MinIO/imgproxy media, Meilisearch, Docker,
  CI, and five project-scoped Rust skills.
- Overlays: Postgres/pgx/sqlc/goose, optional Redis cache, RabbitMQ worker and
  scheduler, Redis scheduler leadership, dedupe, gzip, users/JWT auth, Google
  OAuth, email verification/reset, TOTP login/recovery, MinIO/imgproxy/nginx or
  Cloudflare R2 media, Meilisearch search, RabbitMQ search index sync, signed
  cursor/offset pagination, Docker, CI, embedded migrations, and optional
  demo-user seeding.
- All 17 Go skills plus `sql-optimization-patterns` and `postgresql-optimization` ship in the base; `cache:redis` adds `redis-patterns` and `queue:rabbitmq` adds `rabbitmq-development`.

## Current Next decisions

- One composable `next` base; `fullstack` is the default frontend preset.
- Presets: `fullstack` (Next-owned auth) and `frontend`. The `fullstack-api`
  kind preset combines Next.js with a Go/Rust API.
- Add Docker, CI, E2E, and PostHog with `--feature`. `auth:bff` is auto-added
  for `fullstack-api` when the API uses `auth:jwt`.
- Next 16.4.6, React 19.3.0, TypeScript 5.9.3, Node 24.
- pnpm default; npm and bun selectable.
- App Router; `route.ts` Route Handlers are the backend.
- Tailwind 4, Jest, Zod, Pino.
- Metadata API, OpenGraph image, sitemap, robots.
- Overlays: Postgres/Drizzle, Better Auth/RHF, Zustand, TanStack Query,
  API JWT BFF, Playwright E2E, PostHog, Docker, CI. CI runs E2E only when the E2E overlay is
  present.
- Required overlays are auto-added; removal is guarded unless `--cascade`.

Full stack and version details: [STACKS.md](https://github.com/BoatengPeter/smokeforge/blob/main/STACKS.md).

## Skills policy

Generated apps receive their own agent skills under `.agents/skills` and
`.claude/skills`, plus `AGENTS.md`, `CLAUDE.md`, and `skills-lock.json`.
Next, Go, Rust, Android/Kotlin, and the egui component skills are scoped to the
generated app, not to the smokeforge CLI repo.

## Release format

Registry releases publish:

```text
registry.tar.gz      # archive with a top-level registry/ directory
registry.sha256      # sha256sum output
```

smokeforge fetches `<source>/download/<tag>/registry.tar.gz` and verifies the
checksum before extraction.

## Development

The registry is declarative: no hooks or executable template content. Every
overlay operation must be validated and covered by contract tests once the
manifest is introduced. Shell commands are only allowed in maintainer scripts
outside template content.

## Documentation

- [Agent guide](AGENTS.md)
- [Claude guide](CLAUDE.md)
- [smokeforge architecture](https://github.com/BoatengPeter/smokeforge/blob/main/docs/architecture.md)
- [Overlay model](https://github.com/BoatengPeter/smokeforge/blob/main/docs/registry/overlays.md)
