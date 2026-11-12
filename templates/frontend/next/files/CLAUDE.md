# CLAUDE.md — generated Next.js app

Read [AGENTS.md](AGENTS.md) and [CONTEXT.md](CONTEXT.md) first. CONTEXT.md is
the source of truth for style, patterns, and the quality gate.

This is a Next.js App Router application scaffolded by smokeforge.

## Essentials

- **Stack:** Next.js 16.4.6, React 19.3.0, TypeScript 5.9.3, Node 24, Tailwind 4.
- **Package manager:** the `packageManager` field in `package.json` (pnpm default);
  replace `pnpm` in commands with npm or bun if declared.
- **Backend:** standard `src/app/api/**/route.ts` Route Handlers.
- **Validation:** Zod everywhere at trust boundaries.
- **Forms/state:** React Hook Form + Zod; Zustand provider with per-request stores.
- **Tests:** Jest + Testing Library; Playwright for e2e.
- **Agents:** use the project-scoped skills under `.agents/skills` and
  `.claude/skills`; see `skills-lock.json`.

## Non-negotiables

- No secrets in `NEXT_PUBLIC_*`; validate env with Zod.
- Authenticate and authorize inside every protected route handler.
- Do not fetch user-controlled URLs server-side.
- No mutable module-level request state.
- PostHog capture stays opt-in until consent.
- Keep robots rules crawl-safe: do not block `/_next/`, CSS, JS, or images.
- Run `lint`, `typecheck`, `test`, and `build` before claiming completion.
