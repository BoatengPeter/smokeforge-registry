---
name: cross-client-testing
description: Plan and run verification for a composable smokeforge workspace. Use when a change spans clients, after changing root wiring, or when diagnosing a workspace CI failure.
---

# Cross-client testing

## Matrix

Only test selected clients. Do not add checks for apps that were not generated.

- API: API unit, integration, and build checks.
- Web: install, lint, typecheck, test, and build.
- Desktop: platform-specific check, test, and build.
- Android: unit tests, lint, and debug build.
- iOS: package tests, project generation, and simulator build.

## Order

1. Run the narrowest failing app check.
2. Run affected app checks.
3. Run `make test` for cross-client behavior.
4. Run `make ci` before claiming workspace completion.

## Failure triage

- Identify the owning app first.
- Check whether the root wiring selected an app or command that does not exist.
- Check generated agent files and context maps for stale unselected references.
- Keep root fixes cross-cutting; move platform fixes into the app.
