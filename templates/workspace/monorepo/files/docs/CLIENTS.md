# Clients

The workspace contains only the clients listed in the root `README.md` and
`CONTEXT-MAP.md`.

## Selected client map

<!-- smokeforge:anchor workspace-doc-clients -->

## Client ownership

- Each app owns its `AGENTS.md`, `CONTEXT.md`, docs, skill tree, and quality gate.
- Root files route agents to those app-local guides and list cross-client
  commands.
- Root skills never replace platform skills.

## Boundaries

- Web browser state is not an authoritative auth store.
- Desktop tokens stay in the trusted process.
- Mobile tokens stay in platform secure storage.
- No client reads database credentials or server secrets.
