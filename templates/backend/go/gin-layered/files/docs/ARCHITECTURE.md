# Architecture

This project uses a layered architecture. The goal is a direct dependency flow
that is obvious to a new reader and easy to test.

## Request flow

```text
HTTP request
  → internal/httpapi middleware
  → generated OpenAPI handler
  → internal/app facade
  → internal/service use case
  → feature package or database adapter
  → response
```

## Packages

| Package | Responsibility |
| --- | --- |
| `cmd/api` | Reads config, builds the logger, wires services, starts and stops HTTP. |
| `internal/httpapi` | Gin engine, middleware, and generated server handlers. |
| `internal/swagger` | Embedded OpenAPI contract, document merging, and Swagger UI. |
| `internal/response` | Success response helpers and RFC 9457 error responses. |
| `internal/app` | Thin adapters between transport contracts and use cases. |
| `internal/service` | Transport-neutral application behavior. |
| `internal/server` | HTTP and pprof listeners, graceful shutdown. |
| `internal/config` | Typed environment configuration and validation. |
| `internal/logging` | Zap configuration. |
| `internal/version` | Build metadata. |

Feature overlays add self-contained packages. They must not depend on
`cmd/api`, and they register their routes through `httpapi.Registrar`.

## Boundaries

- No business logic in handlers.
- No Gin or HTTP types in services.
- No SQL in handlers or services; database adapters own it.
- No global mutable state.
- No hidden initialization in `init()`.
- Context flows from the request through every call.
- Errors are returned with context and mapped to RFC 9457 only at the boundary.

## Swagger and contract

`internal/swagger/openapi.yaml` is the base contract. Each documented feature
registrar contributes another OpenAPI document. At startup the server merges
them into `/openapi.json` and serves Swagger UI at `/docs`. The merge fails
closed if two documents declare the same path or component.

Regenerate code after changing a contract:

```bash
make generate
```
