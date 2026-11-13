# AGENTS.md — generated Next.js app

This project was scaffolded by smokeforge. It is a Next.js App Router project.

Read [CONTEXT.md](CONTEXT.md) before changing code. It defines the required
skills, shadcn-first UI rules, file patterns, and the quality gate. Read
[DESIGN.md](DESIGN.md) before visual work and [PRODUCT.md](PRODUCT.md) for
product truth.

## Stack

- Next.js `16.4.6`, React `19.3.0`, TypeScript `5.9.3`
- Node.js 24 LTS
- Package manager: read `packageManager` in `package.json` (pnpm default; npm/bun supported)
- Tailwind CSS 4 with `@tailwindcss/turbopack`, shadcn/ui, Lucide
- Jest + Testing Library; Playwright when the E2E overlay is selected
- Zod; React Hook Form with auth; Zustand with the state overlay
- Pino server logging
- Optional overlays: Postgres + Drizzle, Better Auth, Zustand, Playwright E2E,
  PostHog, Docker, CI

## Commands

Replace `pnpm` below with the package manager declared in `package.json`.

```bash
pnpm install
pnpm dev
pnpm lint
pnpm typecheck
pnpm test
pnpm test:e2e     # E2E overlay only
pnpm build
pnpm start
```

Database overlays add:

```bash
pnpm db:generate
pnpm db:migrate
pnpm db:studio
```

A lockfile is created by the first install. Commit it and use frozen/CI installs
afterward.

## Architecture

- App Router under `src/app`; Server Components by default.
- The backend is implemented with standard Route Handlers under `src/app/api`:
  - `GET /api/health`
  - `/api/auth/[...all]` when auth is selected
  - versioned business routes under `/api/v1/**`
- Validate request bodies, query strings, and params with Zod.
- Success envelope: `{ "data": ... }`; list responses may add `pagination`.
- Error envelope: `{ "error": { "code", "message", "fields" } }`.
- Never trust client input; authorize inside every protected route handler.
- No server actions as the API layer; use Route Handlers.
- Metadata, OpenGraph image, sitemap, and robots are configured under `src/app`.
- When the auth overlay is present, forms use React Hook Form + Zod.
- When the state overlay is present, read shared client state through the
  per-request Zustand provider; never use module-level mutable request state.

## Security

- Never put secrets in `NEXT_PUBLIC_*` variables.
- Validate environment variables with Zod; fail fast on missing required values.
- Do not fetch user-controlled URLs from the server (SSRF).
- Use secure, HTTP-only cookies for sessions.
- Keep `/_next/`, CSS, JS, and images crawlable in robots rules; block `/api/`
  only when appropriate. Pages that need noindex must remain crawlable.
- PostHog is opt-in; do not capture analytics before consent.
- Server errors use `AppError` + `withApi` and Pino; never return ad-hoc error
  shapes or log secrets.
- Docker runs as a non-root user.

## Project-scoped skills

These skills are installed in `.agents/skills` and `.claude/skills`. Read the
relevant `SKILL.md` before changing that area:

- `nextjs-app-router-patterns` — App Router structure, route handlers, data flow
- `better-auth-best-practices` — Better Auth sessions, adapters, and TOTP
- `better-auth-security-best-practices` — auth hardening, cookies, CSRF, secrets, and attack prevention
- `github-actions-templates` — CI workflow patterns, caching, permissions, and Docker publishing
- `vercel-react-best-practices` — React/Next.js performance rules
- `vercel-composition-patterns` — component composition and state providers
- `animation-vocabulary` — precise names for motion effects (scroll reveal, stagger, spring, easing)
- `impeccable` — design direction, responsive adaptation, critique, audit, and polish
- `frontend-design` — intentional visual direction
- `web-design-guidelines` — interface/accessibility review
- `tailwind-design-system` — Tailwind v4 tokens and components
- `zustand` — state-only stores, actions, providers, selectors
- `zod` — schema and validation patterns
- `robots-txt` — crawl directives, sitemap, AI crawler rules

`skills-lock.json` records sources and hashes. Update it with the skills CLI when
changing skills; do not edit skill copies by hand.

## Conventions

- Keep client components small; pass only serializable fields across the RSC boundary.
- Avoid barrel imports and client-side waterfalls.
- Use accessible semantic HTML and visible focus states.
- Prefer composition and compound components over boolean prop proliferation.
- Run `lint`, `typecheck`, `test`, and `build` before finishing changes. See
  `CONTEXT.md`; never claim completion without fresh verification output.
