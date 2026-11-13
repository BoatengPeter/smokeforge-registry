# Architecture

This project uses a hexagonal (ports and adapters) architecture. The domain is
independent of Gin, the database, and outbound clients.

## Request flow

```text
HTTP request
  → internal/httpapi middleware
  → generated OpenAPI handler
  → internal/app facade
  → internal/port inbound contract
  → internal/service implementation
  → internal/domain types
  → response
```

## Packages

| Package | Responsibility |
| --- | --- |
| `cmd/api` | Reads config, builds the logger, wires adapters, starts and stops HTTP. |
| `internal/httpapi` | Primary Gin adapter, middleware, generated server interface, docs. |
| `internal/response` | Success response helpers and RFC 9457 error responses. |
| `internal/app` | Adapts inbound ports to the HTTP transport. |
| `internal/port` | Inbound application contracts. |
| `internal/service` | Application implementation of inbound ports. |
| `internal/domain` | Pure domain types and invariants. |
| `internal/config` | Typed environment configuration and validation. |
| `internal/logging` | Zap configuration. |
| `internal/server` | HTTP and pprof listeners, graceful shutdown. |
| `internal/version` | Build metadata. |

## Boundaries

- `domain` never imports Gin, SQL, or infrastructure packages.
- Ports belong where consumed; adapters implement them.
- No business logic in HTTP handlers.
- No global mutable state or `init()`.
- Context flows from the request through every call.
- Errors are returned with context and mapped to RFC 9457 at the boundary.

## Swagger and contract

`internal/httpapi/openapi.yaml` is the base contract. Each documented feature
registrar contributes another OpenAPI document. At startup the server merges
them into `/openapi.json` and serves Swagger UI at `/docs`.
