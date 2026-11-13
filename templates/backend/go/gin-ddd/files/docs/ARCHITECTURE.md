# Architecture

This project uses a small DDD layout. The ping behavior is a bounded context
with a pure domain and an application handler.

## Request flow

```text
HTTP request
  → internal/httpapi middleware
  → generated OpenAPI handler
  → internal/app facade
  → internal/ping/application handler
  → internal/ping/domain value
  → response
```

## Packages

| Package | Responsibility |
| --- | --- |
| `cmd/api` | Reads config, builds the logger, wires the context, starts and stops HTTP. |
| `internal/httpapi` | Gin transport, middleware, generated server interface, docs. |
| `internal/response` | Success response helpers and RFC 9457 error responses. |
| `internal/app` | Adapts the bounded context to the HTTP transport. |
| `internal/ping/domain` | Pure ping domain value. |
| `internal/ping/application` | Ping use case handler. |
| `internal/config` | Typed environment configuration and validation. |
| `internal/logging` | Zap configuration. |
| `internal/server` | HTTP and pprof listeners, graceful shutdown. |
| `internal/version` | Build metadata. |

## Boundaries

- Domain code has no framework or infrastructure dependencies.
- Application handlers orchestrate domain behavior; HTTP only translates.
- No business logic in HTTP handlers.
- No global mutable state or `init()`.
- Context flows from the request through every call.
- Errors are returned with context and mapped to RFC 9457 at the boundary.

## Swagger and contract

`internal/httpapi/openapi.yaml` is the base contract. Each documented feature
registrar contributes another OpenAPI document. At startup the server merges
them into `/openapi.json` and serves Swagger UI at `/docs`.
