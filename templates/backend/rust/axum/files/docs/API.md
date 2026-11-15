# API

- `GET /healthz` — liveness.
- `GET /readyz` — dependency readiness.
- `GET /api/v1/ping` — lightweight API probe.
- `GET /openapi.json` — OpenAPI document.
- `GET /docs` — Swagger UI.

Errors use RFC 9457 problem details with `application/problem+json`:

```json
{
  "type": "about:blank",
  "title": "Validation failed",
  "status": 422,
  "detail": "One or more fields are invalid.",
  "instance": "/api/v1/items",
  "code": "validation_failed",
  "request_id": "019...",
  "errors": [{ "field": "name", "message": "Required." }]
}
```

`instance` and `request_id` are added by `response::finalize_problem`; the
`X-Request-ID` response header is also set. Success responses are normal JSON
bodies and use `crate::response` helpers where helpful.
