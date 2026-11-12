# AGENTS.md — smokeforge registry

This repository is the versioned template registry used by
[smokeforge](https://github.com/BoatengPeter/smokeforge). Read
[README.md](README.md) first.

## Status

M3 is in progress. The first slice is the Next.js frontend template and its
project-scoped agent skills. The manifest, base files, overlays, and tests are
being added incrementally.

## Non-negotiables

1. **Templates are declarative data.** No executable hooks, scripts, plugins, or
   shell commands inside template or overlay content.
2. **Every path is relative and local.** Overlay destinations, template paths,
   and rendered outputs must not escape their roots.
3. **Anchors are contracts.** Every anchor an overlay references must exist
   exactly once in each supported base.
4. **Overlays compose.** Do not duplicate full base trees for feature combinations.
5. **Every base and every overlay×base pair must build.** Add contract tests and
   a native build check for new content.
6. **Generated apps own their skills.** Frontend skills live under
   `templates/frontend/next/files/.agents/skills` and `.claude/skills`, not at
   the registry root. They are copied into generated projects with
   `AGENTS.md`, `CLAUDE.md`, and `skills-lock.json`.
7. **Secrets never enter the registry.** Generated secrets are created locally by
   the CLI from `.env.example` declarations.

## Layout

```text
manifest.yaml
templates/<kind>/<stack>/{template.yaml,files/}
overlays/<group>/<feature>/<variant>/{overlay.yaml,files/}
tests/
```

## Authoring checklist

- [ ] Stack template has `template.yaml` and a `files/` tree.
- [ ] Variables have prompts, sensible defaults, and allowed choices where needed.
- [ ] Overlay operations validate against every supported stack.
- [ ] Anchors resolve exactly once.
- [ ] No-op install/build works with pnpm, npm, and bun where supported.
- [ ] Generated `AGENTS.md`, `CLAUDE.md`, and `skills-lock.json` are current.
- [ ] Security defaults documented in the generated app remain intact.

## Commands

Contract tests and the build matrix will be added with the manifest. Until then,
changes are validated against the CLI's local fixture workflow and Next's
`pnpm install && pnpm build` path.

## Related repositories

- CLI: https://github.com/BoatengPeter/smokeforge
- Stack catalog: https://github.com/BoatengPeter/smokeforge/blob/main/STACKS.md
- Security: https://github.com/BoatengPeter/smokeforge/blob/main/SECURITY.md
