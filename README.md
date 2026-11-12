# smokeforge-registry

Versioned template registry for [smokeforge](https://github.com/BoatengPeter/smokeforge).

**Status:** the Next.js base, shadcn/ui setup, project-scoped agent skills, and
Docker/CI/Postgres/auth/PostHog overlays are implemented. Contract tests and
registry release automation are the remaining M3 work.

## Layout

```text
manifest.yaml                          # added when the Next stack is complete
templates/
  frontend/next/
    template.yaml
    files/                             # rendered into the generated project
      .agents/skills/                  # project-scoped skills for agents
      .claude/skills/
      AGENTS.md
      CLAUDE.md
      skills-lock.json
overlays/
  next/                                # database, auth, totp, posthog, docker, ci
tests/                                 # contract/build matrix
```

## Current Next decisions

- One composable `next` base; standalone = no backend overlays.
- Next 16.4.6, React 19.3.0, TypeScript 5.9.3, Node 24.
- pnpm default; npm and bun selectable.
- App Router; `route.ts` Route Handlers are the backend.
- Tailwind 4, Jest, Playwright, RHF + Zod, Zustand.
- Metadata API, OpenGraph image, sitemap, robots.
- Optional overlays: Postgres/Drizzle, Better Auth/TOTP, PostHog, Docker, CI.

Full stack and version details: [STACKS.md](https://github.com/BoatengPeter/smokeforge/blob/main/STACKS.md).

## Skills policy

Generated apps receive their own agent skills under `.agents/skills` and
`.claude/skills`, plus `AGENTS.md`, `CLAUDE.md`, and `skills-lock.json`.
Frontend skills are scoped to the generated app, not to the smokeforge CLI repo.

## Release format

Registry releases publish:

```text
registry.tar.gz      # archive with a top-level registry/ directory
registry.sha256      # sha256sum output
```

smokeforge fetches `<source>/download/<tag>/registry.tar.gz` and verifies the
checksum before extraction.

## Development

The registry is declarative: no hooks or executable template content. Every
overlay operation must be validated and covered by contract tests once the
manifest is introduced. Shell commands are only allowed in maintainer scripts
outside template content.

## Documentation

- [Agent guide](AGENTS.md)
- [Claude guide](CLAUDE.md)
- [smokeforge architecture](https://github.com/BoatengPeter/smokeforge/blob/main/docs/architecture.md)
- [Overlay model](https://github.com/BoatengPeter/smokeforge/blob/main/docs/registry/overlays.md)
