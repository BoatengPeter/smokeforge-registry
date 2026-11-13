# Architecture

This project uses a deliberately flat architecture. Start with one application
package and split only when a second consumer, test seam, or domain boundary
appears.

## Request flow

```text
HTTP request
  → internal/httpapi middleware
  → generated OpenAPI handler
  → internal/app behavior
  → response
```

## Packages

| Package | Responsibility |
| --- | --- |
| `cmd/api` | Reads config, builds the logger, wires services, starts and stops HTTP. |
| `internal/httpapi` | Gin engine, middleware, generated server interface, and docs. |
| `internal/response` | Success response helpers and RFC 9457 error responses. |
| `internal/app` | Application behavior. |
| `internal/config` | Typed environment configuration and validation. |
| `internal/logging` | Zap configuration. |
| `internal/server` | HTTP and pprof listeners, graceful shutdown. |
| `internal/version` | Build metadata. |

Feature overlays add self-contained packages. They must not depend on
`cmd/api`, and they register their routes through `httpapi.Registrar`.

## Boundaries

- No business logic in handlers.
- No SQL in handlers.
- No global mutable state or `init()`.
- Context flows from the request through every call.
- Errors are returned with context and mapped to RFC 9457 at the boundary.

## Swagger and contract

`internal/httpapi/openapi.yaml` is the base contract. Each documented feature
registrar contributes another OpenAPI document. At startup the server merges
them into `/openapi.json` and serves Swagger UI at `/docs`.
