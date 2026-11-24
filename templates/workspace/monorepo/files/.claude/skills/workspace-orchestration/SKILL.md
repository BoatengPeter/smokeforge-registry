---
name: workspace-orchestration
description: Route work across a generated smokeforge workspace. Use when deciding which app owns a change, which app-local skill to load, or which root quality gate to run.
---

# Workspace orchestration

## When to use

Use this skill before changing code that crosses app boundaries or before
running workspace-wide verification.

## Routing

1. Read the root `CONTEXT-MAP.md`.
2. Open the closest app's `AGENTS.md` and `CONTEXT.md`.
3. Load the app's platform skills from `.agents/skills` or `.claude/skills`.
4. Keep implementation changes inside the owning app unless the root wiring or
   workspace documentation must change.

## Commands

- App-local command: use the command in the app `README.md` or `Makefile`.
- Cross-client check: run `make test`.
- Full selected-lane check: run `make ci`.
- If only one client is selected, do not invent commands for absent clients.

## Boundaries

- API owns persistence and authorization.
- Web owns presentation and its BFF.
- Desktop owns trusted-process API access and local data.
- Android and iOS own native UI and platform secure storage.
- App skills are authoritative for platform code.
