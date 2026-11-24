# CLAUDE.md — generated workspace

Read [AGENTS.md](AGENTS.md), [CONTEXT.md](CONTEXT.md), and
[CONTEXT-MAP.md](CONTEXT-MAP.md) before changing code.

## Selected apps

<!-- smokeforge:anchor workspace-claude -->

## Workspace skills

- `.claude/skills/workspace-orchestration/SKILL.md`
- `.claude/skills/api-contract/SKILL.md`
- `.claude/skills/cross-client-testing/SKILL.md`

## Rules

- Read the selected app's own `AGENTS.md`, `CONTEXT.md`, and skills first.
- Keep app-specific code, docs, and skills inside the app.
- Keep root guidance limited to cross-client behavior and workspace commands.
- Run `make test` or `make ci` when changing more than one client.
- Never persist or log tokens, passwords, or server secrets.
