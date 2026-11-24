# Workspace context

This is a composable monorepo. The API, when present, owns persistence and
authentication. Each client owns its native UI, session lifecycle, and platform
storage.

## Boundaries

- The API owns persistence, authorization, and background work.
- Clients never connect directly to the database or hold server secrets.
- Web browser code uses same-origin BFF routes; it does not receive raw refresh
  tokens.
- Desktop tokens stay in the trusted main process.
- Android tokens use encrypted storage; iOS tokens use Keychain.
- Cross-client changes require checking every affected app guide and updating
  `CONTEXT-MAP.md` when responsibilities change.
- Root skills cover workspace orchestration and shared API contracts. Platform
  skills stay in each app.

## Commands

```bash
make install
make test
make ci
```

## Client rules

<!-- smokeforge:anchor workspace-context -->
