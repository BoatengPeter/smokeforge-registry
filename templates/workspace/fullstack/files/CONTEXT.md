# Workspace context

## Non-negotiables

- The API is the authority for authentication, authorization, and persistence.
  Never duplicate those rules in the web app.
- The Next.js BFF is the only place that handles raw JWT access or refresh
  tokens. Browser code uses HttpOnly cookies and the typed query hooks.
- Environment variables are declared in each app's typed config and validated
  at startup. Never read arbitrary environment variables at call sites.
- Validate every external boundary server-side. Client-side validation is for
  UX only.
- TanStack Query owns server state; do not copy fetched data into Zustand.
- Keep API and web contracts aligned. If an API response changes, update the web
  hook, BFF normalization, and tests in the same change.

## Commands

```bash
make install
make api-test
make web-test
make ci
make docker-up
```

## Frontend component engineering

For renderer work in `apps/web` or `apps/desktop`, read and invoke
`frontend-ui-engineering` in that app. It is installed in both frontend
templates and is the required standard for modular component architecture,
clear file boundaries, and avoiding one-file screens that mix routing, data,
forms, and presentation. Refresh it with:

```bash
npx skills add https://github.com/addyosmani/agent-skills --skill frontend-ui-engineering
```

## Desktop sync

Fullstack desktop projects sync Library records through the API. Local mutations
write the SQLite item and outbox row in one transaction. The main process pushes
idempotent operations with `If-Match`, pulls changes by cursor, retries transient
failures, and keeps conflicts for manual resolution by default.

Read `docs/SYNC.md`, `docs/CONFLICTS.md`, and `docs/IDEMPOTENCY.md` before
changing the API contract, local schema, or sync state machine. The renderer
must never call the API or write the outbox directly.
