# CONTEXT.md — engineering rules for this app

This file is the source of truth for coding style, patterns, and the required
quality gate. Read it before changing code. `AGENTS.md` and `CLAUDE.md` both
reference it.

## Agent non-negotiables

These are hard boundaries, not suggestions:

1. **Invoke the relevant frontend skills before acting.** Read the matching
   `SKILL.md` under `.agents/skills` and `.claude/skills` for every area you
   touch. If a task crosses areas, invoke every relevant skill.
2. **Stay inside the selected stack.** Do not introduce another framework,
   router, state manager, ORM, auth system, UI library, or styling system unless
   the user explicitly asks for it.
3. **Use the chosen component library.** This project uses shadcn/ui under
   `src/components/ui`. Prefer its primitives and add missing ones with the
   shadcn CLI. Do not replace them with native controls or another library
   without explicit permission.
4. **Never run a production build until explicitly asked.** Do not run
   `pnpm build`, `npm run build`, `bun run build`, `make build`, or any
   deployment build unless the user requests it. Typechecking, linting, and
   tests are allowed unless the user says otherwise.
5. **Do not deploy, publish, or start long-running services** unless explicitly
   asked.
6. **Do not weaken security defaults** to make a local check pass. Never commit
   secrets, disable auth checks, bypass validation, or make destructive
   database changes without explicit approval.
7. **Keep changes scoped.** Do not modify generated files, dependencies, or
   configuration outside the requested task without explaining why.

## Before any action

Invoke the relevant project skill before starting the work:

| Work area | Required skill(s) |
| --- | --- |
| App Router pages, layouts, route handlers | `nextjs-app-router-patterns` |
| React components, data fetching, performance | `vercel-react-best-practices` |
| Component APIs, composition, providers | `vercel-composition-patterns` |
| Visual direction and layout | `frontend-design` |
| Accessibility and interface review | `web-design-guidelines` |
| Tailwind tokens and components | `tailwind-design-system` |
| State management | `zustand` |
| Forms and schemas | `zod` |
| Auth | `better-auth-best-practices`, `better-auth-security-best-practices` |
| GitHub Actions | `github-actions-templates` |
| robots/sitemap/crawlers | `robots-txt` |

If a change crosses areas, invoke every relevant skill. Read the skill's
`SKILL.md`; do not guess its rules.

## UI components: shadcn first

- **Never hand-write shadcn primitives.** Add them with the shadcn CLI:

  ```bash
  pnpm dlx shadcn@latest add <component> --yes
  # npm: npx shadcn@latest add <component> --yes
  # bun: bunx shadcn@latest add <component> --yes
  ```

- Do not replace shadcn components with native interactive elements. Use
  `Button`, `Input`, `Textarea`, `Label`, `Card`, `Form`, and other
  `@/components/ui/*` primitives instead of raw `<button>`, `<input>`,
  `<textarea>`, `<label>`, `<select>`, dialogs, or checkboxes.
- Semantic layout elements (`main`, `section`, `nav`, `header`, `footer`,
  `form`, headings, lists) remain normal HTML.
- Icons come from `lucide-react` only.
- Style with Tailwind tokens and `class-variance-authority` variants. No inline
  style attributes for layout/color and no hard-coded hex colors in components.
- Prefer composition and explicit variants over boolean prop proliferation.
- When the auth/forms overlay is present, use shadcn `Form` with React Hook
  Form; do not build parallel form wrappers.

## File and import patterns

- Shared types: `src/types/*.ts`.
- Zod schemas: `src/validations/*.ts`; export inferred types with the schema.
- Client stores: `src/lib/stores/`.
- Server-only code: `src/server/`.
- **No barrel files.** Do not create or import `index.ts` re-export files.
  Import the concrete module:

  ```ts
  import { signupSchema } from "@/validations/auth";
  import type { AuthOTPType } from "@/types/auth";
  import { Button } from "@/components/ui/button";
  ```

- Import UI primitives directly (`@/components/ui/button`), never from a barrel.
- Keep client components small; server components are the default.
- Do not pass unnecessary fields across the RSC boundary.
- No mutable module-level request state.

## State and data

- When the Zustand overlay is present, state is state-only: state lives in the
  store, actions live in `AppActions`, and components call actions through
  `useAppActions()`.
- Use the per-request `AppStoreProvider`; never share a store at module scope.
- Read state through selectors from `src/lib/stores/app-selectors.ts`; use
  `useShallow` for multi-field selections.
- Persist only non-sensitive preferences. Never persist auth/session/secrets.
- API rules:
  - Standard `src/app/api/**/route.ts` handlers only as the backend.
  - Validate every boundary with Zod via `parseJson`.
  - Success `{ "data": ... }`; the only error shape is
    `{ "error": { "code", "message", "fields"? } }`.
  - Throw `AppError`; wrap handlers with `withApi`. Never hand-roll error
    responses.
  - Authenticate and authorize inside each protected handler.
  - Never fetch user-controlled URLs server-side (SSRF).
- Logging: use `logger` from `@/server/logger` (Pino). Log unexpected server
  failures; never log secrets, passwords, tokens, or production OTPs.

## Security

- Secrets never use `NEXT_PUBLIC_*`; validate server env with Zod.
- `.env` is local only; `.env.example` documents keys.
- Better Auth owns sessions and OTP flows. Keep CSRF, rate limiting, and secure
  cookies enabled.
- PostHog captures only after explicit opt-in; autocapture stays off.

## Quality gate — required after every change

Run these before reporting completion, in this order:

```bash
npx tsc --noEmit     # or: pnpm typecheck
npx eslint .         # or: pnpm lint
pnpm test            # or: npm test / bun run test
pnpm build           # or: npm run build / bun run build
```

For a full gate, run:

```bash
make ci              # install, lint, typecheck, test, build
```

Rules:

- A failing command blocks completion. Report the actual output; never claim
  success without fresh evidence.
- If the E2E overlay is present and UI behavior changed, run `pnpm test:e2e`.
- If Docker or CI files changed, validate the Docker build and the workflow.
- If the change adds commands, update the Makefile and `README.md`.
- Commit the lockfile once it exists; do not commit `.env` or `.smokeforge/`.

## Documentation

- `README.md` — setup, commands, configuration, deployment.
- `DEPLOYMENT.md` — deployment checklist and environment matrix.
- `AGENTS.md` — agent mindmap and skill routing.
- `CLAUDE.md` — Claude quick guide.
- `CONTEXT.md` — this file: style, patterns, and the quality gate.
