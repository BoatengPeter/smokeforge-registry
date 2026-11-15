# CONTEXT.md — engineering rules for this API

This file is the source of truth for coding style, architecture, security, and
the required quality gate. Read it before changing code.

## Agent non-negotiables

1. **Invoke the relevant Go skills before acting.** Read the matching `SKILL.md`
   under `.agents/skills` and `.claude/skills` for every area you touch. If a
   task crosses areas, invoke every relevant skill.
2. **Stay inside the selected stack.** Do not add another router, logging
   library, config library, ORM, migration tool, auth system, or validation
   library unless the user explicitly asks for it.
3. **OpenAPI is the contract.** Change the YAML first, regenerate code, then
   implement the handler. Do not invent response shapes in handlers.
4. **TDD is mandatory.** Write a failing test, watch it fail for the expected
   reason, implement the minimum, and keep it green.
5. **Do not weaken security defaults** to make a local check pass. Never commit
   secrets, disable auth checks, bypass validation, or add wildcard CORS with
   credentials.
6. **Keep changes scoped.** Do not modify generated files, dependencies, or
   configuration outside the task without explaining why.
7. **Never run deployment builds or start long-running production services**
   unless the user asks. Unit tests, lint, vet, and local runs are allowed.
8. **Use `make ci` before claiming completion.** Report the exact command and
   fresh output. Never claim success without evidence.

## Before any action

Read the SKILL.md for every relevant area.

| Work area | Required skill(s) |
| --- | --- |
| Any Go code, formatting, clarity | `golang-code-style` |
| Tests, fuzz tests, integration tests | `golang-testing` |
| Error creation, wrapping, logging | `golang-error-handling` |
| Input, crypto, auth, file/network I/O | `golang-security` |
| Nil safety, slices, numeric conversions | `golang-safety` |
| Context propagation and cancellation | `golang-context` |
| Packages, types, functions, tests naming | `golang-naming` |
| Godoc, README, examples, changelog | `golang-documentation` |
| Architecture, constructors, shutdown | `golang-design-patterns` |
| Interfaces, structs, embedding, DI | `golang-structs-interfaces` |
| Profiling identified a bottleneck | `golang-performance` |
| Goroutines, channels, mutexes, workers | `golang-concurrency` |
| golangci-lint or nolint directives | `golang-lint` |
| Benchmarks, pprof, benchstat | `golang-benchmark` |
| Bugs, crashes, races, deadlocks | `golang-troubleshooting` |
| Viper config files or hot reload | `golang-spf13-viper` |
| Swagger/OpenAPI docs | `golang-swagger` |
| SQL schema, indexes, query plans | `sql-optimization-patterns`, `postgresql-optimization` |

If a change crosses areas, invoke every relevant skill. Read the current
`SKILL.md`; do not rely on memory.

## DDD architecture

```text
cmd/api/                      composition root; startup and shutdown
internal/httpapi/             Gin transport, middleware, OpenAPI
internal/response/    Success responses and RFC 9457 error responses
internal/app/                 adapter from the bounded context to HTTP
internal/ping/domain/         pure domain value
internal/ping/application/    use case handler
internal/config/              typed environment configuration
internal/logging/             Zap logger construction
internal/server/              HTTP and pprof lifecycle
internal/version/             build metadata
```

Rules:

- Keep each bounded context vertical: domain, application, then adapters.
- The domain has no framework or infrastructure dependencies.
- Application handlers orchestrate domain behavior; HTTP handlers translate.
- Feature packages are self-contained and register through
  `httpapi.Registrar`.
- `internal/auth` is the application core. Keep HTTP in `internal/auth/http`,
  persistence in `internal/auth/postgres`, email in `internal/auth/email`, and
  Google OAuth in `internal/auth/google`; the core must not import them.
- Pass `context.Context` first and propagate it through every call.

## Go standards

- Constructors are explicit; avoid `init()` and mutable package globals.
- Return concrete types; accept small interfaces.
- Errors use `%w`, lowercase messages, and `errors.Is`/`errors.As`.
- Never log and return the same error. Log once at the boundary.
- Use `crypto/rand` for tokens and secrets; never `math/rand`.
- All external calls have timeouts and respect context cancellation.
- Bound every resource: body size, rate limiter map, DB pool, goroutines.
- Use UUIDv7 for generated identifiers.
- Use `slices`/`maps` from the standard library; avoid reflection and `any` when
  a concrete type works.
- Exported declarations have doc comments; package comments exist.
- No comments that restate code. Comment non-obvious decisions and invariants.
- No unchecked errors. If ignoring one is genuinely correct, use
  `//nolint:errcheck // reason` with a specific linter and justification.

## HTTP and API rules

- `/healthz` is liveness; `/readyz` checks dependencies.
- `/api/v1/**` is the public API. Version before breaking changes.
- Swagger UI lives at `/docs`; the merged contract at `/openapi.json`.
- `API_DOCS_ENABLED=auto` is on in development and off in production.
- Request IDs are generated server-side as UUIDv7 and returned in
  `X-Request-ID`.
- Errors are RFC 9457 `application/problem+json` with stable `code` and
  `request_id` extension members.
- Validation errors return `422` with field-level `errors`.
- Validate every request body at the transport boundary with `validator`.
- Panic recovery returns a generic 500 and logs the panic with Zap.
- CORS origins are explicit. Wildcards are rejected.
- Rate limiting is per client IP with bounded, periodically cleaned state.

## Configuration and logging

- Configuration comes from the environment through `internal/config`.
- `godotenv` only loads local `.env`; it must not be required in production.
- Validate configuration at startup and fail closed.
- Use Zap everywhere. Never log secrets, passwords, tokens, OTPs, or full
  request bodies.
- Keep log messages stable and low-cardinality; attach IDs as fields.

## Database (when the Postgres overlay is present)

- Use pgx through `pgxpool`; use sqlc for generated queries.
- Use goose migrations; never auto-migrate on API startup.
- Run migrations with `make db-migrate` (or the generated `cmd/migrate` tool).
- Use parameterized SQL only.
- Wrap multi-step writes in transactions when invariants span statements.
- Set statement contexts from request contexts.
- Integration tests use the `integration` build tag and a test database from
  `TEST_DATABASE_URL`.

## Auth (when the JWT overlay is present)

- Passwords use Argon2id with a random salt and a versioned encoded hash.
- Access tokens are short-lived HS256 JWTs with issuer, audience, expiry, and
  token type.
- Refresh tokens are opaque random values; only their SHA-256 hashes are stored.
- Rotate refresh tokens on every use and detect reuse by revoking the user's
  active sessions.
- Browser refresh uses a Secure, HttpOnly, SameSite=Lax cookie and requires the
  `X-Requested-With: XMLHttpRequest` header.
- Native clients request body tokens with
  `X-Refresh-Token-Transport: body`.
- TOTP is a second factor. Recovery codes are stored as hashes and are
  single-use.
- In production, email verification is required and auth fails closed when SMTP
  is not configured.

## Quality gate

Run this before reporting completion:

```bash
make fmt-check
make vet
make lint
make test
make test-race
make build
```

For the full gate:

```bash
make ci
```

If `make generate` is relevant, run it before `fmt-check` and commit generated
output. A failing command blocks completion. Report the actual output; never
claim success without fresh evidence.

## Documentation

- `README.md` — setup, commands, endpoints, configuration, security.
- `CONTEXT.md` — this file: rules and quality gate.
- `AGENTS.md` — full agent guide and skill routing.
- `CLAUDE.md` — quick agent guide.
- `docs/README.md` — documentation index.
- `docs/ARCHITECTURE.md` — package boundaries, lifecycle, middleware, and DI.
- `docs/API.md` — OpenAPI contract and versioning.

## Environment variables

- `internal/config` is the only package that reads `.env` or process environment variables.
- Add typed fields with defaults, and validate required and secret values at startup.
- Never call `os.Getenv` from handlers, services, repositories, or feature packages; inject `config.Config` or typed values.
- Secrets are server-only, never logged, never serialized into responses, and never committed to `.env.example`.
- New feature overlays declare variables through the registry `env` operation and config anchors, not ad hoc reads.

## Input validation

- Validate every untrusted boundary: request bodies, query parameters, path parameters, headers, cookies, uploads, environment values, and external responses.
- Reject invalid input before business logic, SQL, filesystem, network, or rendering work; return the app's typed validation error.
- Use allow-lists for enums, content types, sizes, sort fields, and redirect/origin values. Never concatenate untrusted input into SQL, shell commands, paths, or URLs.
- Validate at the closest boundary and keep domain invariants enforced by types. Client-side checks are UX only, never the security boundary.
- Validate content type and size for uploads, and validate the final stored object before marking it ready.
