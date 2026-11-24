# AGENTS.md — generated workspace

Read [CONTEXT.md](CONTEXT.md), [CONTEXT-MAP.md](CONTEXT-MAP.md), and the
selected app's `AGENTS.md` before changing code.

This is a composable monorepo. Root guidance covers cross-client boundaries and
workspace orchestration. Platform guidance lives with each app.

## Selected apps

<!-- smokeforge:anchor workspace-agents -->

## Root skills

- `.agents/skills/workspace-orchestration/SKILL.md` — lane routing, commands,
  boundaries, and when to load app-local skills.
- `.agents/skills/api-contract/SKILL.md` — shared auth, error, DTO, and
  environment contract.
- `.agents/skills/cross-client-testing/SKILL.md` — workspace verification and
  failure triage.

Use `.claude/skills/` for the same workspace-level skills when using Claude.

## Rules

- Read the closest app `AGENTS.md` and `CONTEXT.md` before changing that app.
- Keep platform skills under the app that owns them.
- Never put tokens, passwords, or server secrets in root docs, logs, or fixtures.
- Run the narrowest app quality gate first, then workspace `make test`/`make ci`
  when a change crosses client boundaries.
- Keep root docs client-neutral unless a selected-client section explicitly says
  otherwise.
- Do not add a client target outside the generated `apps/` layout.
