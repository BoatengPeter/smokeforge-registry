# API contract and versioning

The API contract is spec-first.

- Public business endpoints live under `/api/v1`.
- `/healthz` and `/readyz` remain unversioned operational probes.
- `internal/swagger/openapi.yaml` is the base contract.
- Generated server interfaces live in `internal/httpapi/api.gen.go`.
- Feature packages own their OpenAPI documents and register routes through
  `httpapi.Registrar`; `internal/swagger` merges them at startup.
- Swagger UI is served at `/docs`; the merged document at `/openapi.json`.

## Changing the API

1. Edit `internal/swagger/openapi.yaml` or the owning feature contract.
2. Run `make generate`.
3. Implement the generated handler interface.
4. Run `make ci`.

Do not hand-write request/response types that duplicate the contract, and do not
add Swagger annotations to handlers. The OpenAPI YAML is the source of truth.

## Breaking changes

Additive fields and endpoints stay in `v1`. Breaking request/response or
semantic changes require a new `/api/v2` prefix and a parallel contract.
