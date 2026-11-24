# Context map

This workspace contains the selected clients below. Read the closest
`CONTEXT.md` before changing code in that app.

| Area | Context | Responsibility |
| --- | --- | --- |
| Workspace | [`CONTEXT.md`](CONTEXT.md) | Cross-client wiring, environments, CI |
<!-- smokeforge:anchor workspace-context-map -->

## Boundaries

- The API is the source of truth when selected.
- Client storage is local state, not a second authoritative server database.
- Cross-client changes require checking every selected app guide.
- Root agent files route to app-local `AGENTS.md`, `CONTEXT.md`, and skills.

<!-- smokeforge:anchor workspace-context-map-details -->
