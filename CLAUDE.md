# CLAUDE.md — smokeforge registry

Read [AGENTS.md](AGENTS.md) and [README.md](README.md) first.

## Purpose

Versioned template registry for smokeforge. M3 is in progress; the Next.js base
and its project-scoped agent skills are the first slice.

## Rules

- Templates are declarative data; never add executable content.
- Keep paths relative and confined to their roots.
- Anchors must match exactly once per supported base.
- Add contract/build tests for every base and overlay×base pair.
- Frontend skills belong in `templates/frontend/next/files/.agents/skills` and
  `.claude/skills`, not at the repository root.
- Generated apps must get `AGENTS.md`, `CLAUDE.md`, and `skills-lock.json`.
- Never commit secrets.

## References

- CLI: https://github.com/BoatengPeter/smokeforge
- Stack catalog: https://github.com/BoatengPeter/smokeforge/blob/main/STACKS.md
- Overlay contract: https://github.com/BoatengPeter/smokeforge/blob/main/docs/registry/overlays.md
