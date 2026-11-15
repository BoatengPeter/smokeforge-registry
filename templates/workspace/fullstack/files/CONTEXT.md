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
