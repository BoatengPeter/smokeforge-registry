# AGENTS.md — generated Go API

This project was scaffolded by smokeforge. It is a layered Go HTTP API.

Read [CONTEXT.md](CONTEXT.md) before changing code. CONTEXT.md owns the coding
rules, skill routing, and required quality gate.

## Stack

- Go 1.26, Gin, `net/http`
- Spec-first OpenAPI with `oapi-codegen`
- Swagger UI at `/docs`, merged contract at `/openapi.json`
- RFC 9457 error responses
- `caarlos0/env` configuration; `godotenv` for local loading
- Zap structured logging
- `go-playground/validator`
- UUIDv7 identifiers
- `golang.org/x/time/rate` per-IP rate limiting
- Optional PostgreSQL, pgx, sqlc, and goose
- Optional JWT auth with rotating refresh sessions and TOTP
- Optional Docker and GitHub Actions overlays

## Commands

```bash
make install
make run
make dev
make test
make test-race
make test-integration
make lint
make fmt
make fmt-check
make vet
make vulncheck
make generate
make ci
make build
```

Run `make ci` before reporting completion. Read `CONTEXT.md` for the full gate.

## Architecture

```text
cmd/api/              process startup and graceful shutdown
internal/httpapi/     Gin transport, middleware, and generated handlers
internal/swagger/     OpenAPI contract and Swagger UI
internal/response/    Success responses and RFC 9457 error responses
internal/app/         application facade exposed to HTTP
internal/service/     application use cases
internal/server/      HTTP and pprof lifecycle, graceful shutdown
internal/config/      typed environment configuration
internal/logging/     Zap logger construction
internal/version/     build metadata
```

Feature overlays use self-contained packages and register through
`httpapi.Registrar`. The base merges each documented registrar's OpenAPI
document into the Swagger UI contract.

## Dependency rules

- `cmd` wires dependencies; it does not contain business logic.
- `httpapi` owns HTTP concerns and may call `app`; `app` may call `service`.
- `service` must not import Gin or `httpapi`.
- Infrastructure packages may depend on service contracts, never the reverse.
- Pass `context.Context` as the first parameter on every request-scoped call.
- Constructors return concrete types; interfaces are defined by consumers.

## API conventions

- OpenAPI is the source of truth. Change the contract before the handler.
- Validate request bodies with `validator` at the transport boundary.
- Success responses match the OpenAPI schema.
- Errors use `application/problem+json` with RFC 9457 fields.
- Every request gets a server-generated UUIDv7 request ID.
- Never expose internal errors to clients; log them with Zap.

## Security

- Never commit `.env`, secrets, tokens, or password hashes.
- Use `crypto/rand` for tokens; use Argon2id for passwords when auth is present.
- Use parameterized SQL only; never build queries by concatenation.
- Keep CORS origins explicit; wildcard origins are rejected.
- Keep pprof on the separate opt-in listener.
- Keep refresh cookies Secure, HttpOnly, and SameSite=Lax in production.
- Require explicit CSRF protection for cookie-authenticated refresh/logout.

## Project-scoped skills

Read the matching `SKILL.md` under `.agents/skills` and `.claude/skills` before
changing an area. The full routing table is in `CONTEXT.md`.

Base skills installed:
`golang-code-style`, `golang-testing`, `golang-error-handling`,
`golang-security`, `golang-safety`, `golang-context`, `golang-naming`,
`golang-documentation`, `golang-design-patterns`,
`golang-structs-interfaces`, `golang-performance`, `golang-concurrency`,
`golang-lint`, `golang-benchmark`, `golang-troubleshooting`,
`golang-spf13-viper`, `golang-swagger`, `sql-optimization-patterns`, and
`postgresql-optimization`.

<!-- smokeforge:anchor feature-skills -->

`skills-lock.json` records sources and hashes. Update it with the skills CLI when
changing installed skills; do not edit skill copies by hand.
