# Testing

## Quality gates

Run app-local tests first, then workspace gates for cross-client changes:

```bash
make test
make ci
```

Only selected clients participate. API-less workspaces skip API checks.

## Expectations

- Test observable behavior in each app.
- Keep unit tests deterministic and free of live services.
- Use integration or end-to-end tests for cross-client contracts.
- Keep platform test commands in the app docs.

## Root checks

Root verification should check selected targets, generated agent files, skills,
lockfile, Make targets, and CI jobs. It should fail when root guidance references
an app that was not generated.
