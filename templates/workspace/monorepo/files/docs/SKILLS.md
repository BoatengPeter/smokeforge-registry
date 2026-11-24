# Skills

Root skills cover workspace-wide work only:

- `workspace-orchestration`: selected lanes, commands, boundaries, and when to
  load app-local skills.
- `api-contract`: auth, error, DTO, and environment consistency across clients.
- `cross-client-testing`: verification matrix and failure triage.

Platform skills live under each app:

<!-- smokeforge:anchor workspace-skills -->

Do not copy app-specific skill payloads into the root. The root
`skills-lock.json` records only workspace-level skills.
