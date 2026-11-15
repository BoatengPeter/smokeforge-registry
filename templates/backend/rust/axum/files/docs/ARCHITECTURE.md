# Architecture

The API is one crate with domain-oriented modules. `main` wires configuration,
telemetry, state, and the Axum application. `lib` exposes the router for
network-free tests. Health, database, and auth concerns live in their own
modules instead of a shared technical-layer directory.

## Request flow

```mermaid
flowchart LR
    Client --> Axum
    Axum --> Middleware[Request ID, trace, CORS, limits, timeout, rate limit]
    Middleware --> Routes[Health / Auth / Media / Search]
    Routes --> Services[Domain services]
    Services --> DB[(PostgreSQL)]
    Services --> Redis[(Redis)]
    Services --> S3[(S3 / MinIO / R2)]
    Services --> Meili[(Meilisearch)]
```
