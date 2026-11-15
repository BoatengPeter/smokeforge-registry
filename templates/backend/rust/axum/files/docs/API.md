# API

- `GET /healthz` — liveness.
- `GET /readyz` — dependency readiness.
- `GET /api/v1/ping` — lightweight API probe.
- `GET /openapi.json` — OpenAPI document.
- `GET /docs` — Swagger UI.

Errors use RFC 9457 problem details with `application/problem+json`.
