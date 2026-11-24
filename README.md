# smokeforge-registry

Versioned template registry for [smokeforge](https://github.com/BoatengPeter/smokeforge).

**Status:** Next.js, the four Go/Gin architectures, the Rust/Axum backend, the
full Electron app shell, the native egui desktop base, the Android Compose and
iOS SwiftUI production bases, optional fullstack mobile/desktop composition, and the composable `workspace` root are implemented. The desktop shell includes React Router, shadcn/ui, Drizzle SQLite,
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
  mobile/ios/swiftui/                 # iOS SwiftUI base + Swift/XcodeGen skills
  workspace/mobile/                   # API + mobile workspace root
  workspace/monorepo/                 # composable API/web/desktop/mobile root
  desktop/electron/                   # secure TypeScript Electron base
  desktop/egui/                       # native Rust egui base + component/Rust skills
  workspace/fullstack/                # root CONTEXT-MAP, Makefile, env docs
overlays/
  backend/go/                          # database, auth, cache, queue, media, search, pagination
  backend/rust/                        # database, auth, cache, queue, scheduler, dedupe, media, search, sync, pagination
  next/                                # database, auth, bff, state, data, testing, posthog, docker, ci
  workspace/fullstack/mobile/           # web + mobile workspace wiring
  workspace/mobile/                    # API + mobile workspace wiring and root CI
  desktop/electron/                    # data, state, storage:sqlite, testing, CI
  desktop/egui/                         # egui base CI overlay
  workspace/fullstack/                 # root compose and CI aggregation
  workspace/monorepo/                  # composable root skills, docs, Make, CI
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
- Mobile presets: `mobile` (client-only) and `mobile-api` (API + Postgres/JWT).
- Workspace preset: `workspace-api` (optional API foundation with CI).
- Android base: Compose, Material 3, Hilt/KSP, Retrofit, encrypted DataStore,
  email/password auth, debug/staging/release variants, CI.
- iOS base: SwiftUI, local Swift packages, XcodeGen, Keychain, Swift Testing,
  debug/staging/release xcconfigs, macOS CI.
- Desktop presets: `desktop` (Electron + React + TypeScript, TanStack Query,
  Zustand, SQLite) and `desktop-egui` (native Rust, rusqlite/refinery, keyring,
  egui_tiles, full CI). Electron package manager is selectable (pnpm, npm, or
  bun).
- Desktop dev loops: Electron `make dev` is `electron-vite dev --watch`
  (React Fast Refresh with a dev-only CSP from `src/shared/csp.ts`, preload
  reload, main restart; `make dev-inspect` adds the V8 inspector). egui and
  GPUI `make dev` use pinned cargo-watch to rebuild and restart on save
  (`make dev-tools`, `make watch-check`, `make watch-test`). Each base ships
  `docs/DEVELOPMENT.md`.
- Fullstack mobile: opt in with repeatable `--mobile android/compose` and/or
  `--mobile ios/swiftui`; adds `apps/android` and/or `apps/ios`. Web is generated
  only with explicit `--frontend next`. API + mobile without web uses
  `workspace/mobile`.
- Fullstack desktop: opt in with `--desktop electron` and explicit `--frontend next`; adds the same desktop
  base with main-process auth, SQLite, and the offline Library sync outbox. The
  egui stack is available as a standalone base and will gain the same API/sync
  composition in its milestone issues.
- Fullstack presets: `fullstack-api` (web + API) and `mobile-api` (API + mobile, no web).
- Composable workspace: opt in with `--kind workspace`, an optional `--backend`,
  optional `--frontend next`, one `--desktop electron|egui|gpui`, and repeatable
  `--mobile` clients. It uses `workspace/monorepo`; root skills and agent docs
  route to app-local skills and contexts.
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
Composable workspace roots receive only cross-cutting workspace skills, root
agent routers, and a root lockfile; platform skills stay in each app. Next, Go,
Rust, Android/Kotlin, Swift/SwiftUI, and the egui component skills are scoped to
the generated app, not to the smokeforge CLI repo.

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
