# Workspace context

This is an API plus mobile-client workspace.

## Boundaries

- The API owns persistence, authentication, authorization, and background work.
- Each mobile app owns its native UI, session lifecycle, and secure token storage.
- Mobile clients never connect to the database or hold server secrets.
- Shared API behavior is documented under `apps/api/docs/`.
- Each mobile client has its own `CONTEXT.md` and quality gate.
- Cross-app changes require reading the closest context file and updating
  `CONTEXT-MAP.md` when responsibilities change.

## Commands

```bash
make install
make api-dev
make mobile-build
make test
make ci
```

Mobile build/test targets skip apps that were not generated.
