# Environments

## Local

- Web and desktop packages use the workspace `package_manager` value.
- Mobile apps use their platform simulator defaults documented in each app
  `README.md`.
- Read each selected app's `README.md` and `CONTEXT.md` for local commands and
  environment values.

## Staging and release

Each selected app owns its staging and release configuration and secrets.

Do not put production secrets in the workspace root or commit them to git.

## Quality gates

Use app-local commands for focused work and root commands for cross-client
changes:

```bash
make test
make ci
```
