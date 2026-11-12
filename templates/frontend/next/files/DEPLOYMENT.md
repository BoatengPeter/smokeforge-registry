# Deployment

Deployment checklist for this Next.js app. Environment values live in `.env`
locally and in your platform's secret manager in production.

## Environment matrix

| Variable | Required in production | Notes |
| --- | --- | --- |
| `NEXT_PUBLIC_APP_URL` | yes | Public HTTPS origin; metadata, auth callbacks, sitemap. |
| `NEXT_PUBLIC_API_URL` | no | Standalone frontend only. Leave unset for same-origin `/api`. |
| `DATABASE_URL` | database overlay | Postgres connection string. |
| `BETTER_AUTH_SECRET` | auth overlay | 32+ chars. Generate with `openssl rand -base64 32`. |
| `BETTER_AUTH_URL` | auth overlay | Public HTTPS origin of the app. |
| `BETTER_AUTH_TRUSTED_ORIGINS` | optional | Comma-separated extra origins. |
| `RESEND_API_KEY` | auth email | Required for real OTP email delivery. |
| `EMAIL_FROM` | auth email | Verified sender address for Resend. |
| `NEXT_PUBLIC_POSTHOG_KEY` | analytics overlay | Public key; analytics stays opt-in. |
| `NEXT_PUBLIC_POSTHOG_HOST` | optional | Defaults to US cloud. |
| `PORT` | optional | Docker sets `3000`. |

Never commit `.env`. Confirm `.env` is gitignored.

## Pre-deploy checklist

1. Use Node 24 LTS with the package manager declared in `package.json`.
2. Install with the lockfile (`pnpm install --frozen-lockfile`, `npm ci`, or
   `bun install --frozen-lockfile`) after committing it.
3. Run the quality gate:

   ```bash
   npx tsc --noEmit
   npx eslint .
   pnpm test
   pnpm build
   ```

   Or `make ci` for install + lint + typecheck + tests + build.

4. Run database migrations explicitly before starting the new version:

   ```bash
   pnpm db:migrate
   ```

   Do not run migrations automatically on every application boot.

5. Verify `/api/health` returns `{"data":{"status":"ok","service":"web"}}`.
6. Exercise signup, email OTP verification, sign-in, and password reset on a
   staging deployment before promoting.

## Authentication email

Local development logs OTP codes to the server console when
`RESEND_API_KEY`/`EMAIL_FROM` are unset. This is development-only.

For production, set `RESEND_API_KEY` and `EMAIL_FROM`. Without them, auth email
delivery fails closed. Never log OTPs in production.

## Docker

If the `docker` overlay was selected:

```bash
make docker-build
make docker-up
```

With the database overlay, start Postgres separately:

```bash
docker compose -f compose.db.yaml up -d db
```

Then run migrations from your release job or a one-off container:

```bash
docker compose run --rm web pnpm db:migrate
```

The image runs as a non-root user and includes a health check. Backing services
are not started automatically by the app container.

## Node standalone

`next.config.ts` sets `output: "standalone"`. After `pnpm build`, run the
generated server:

```bash
node .next/standalone/server.js
```

Copy `.next/static` and `public/` into the standalone tree if deploying outside
Docker.

## Vercel

Vercel is supported for the app server, but serverless constraints apply:

- use a serverless-friendly Postgres provider/pooler;
- avoid long-lived queue workers;
- confirm Better Auth uses the public HTTPS URL;
- set all environment variables in the project settings.

The default deployment target for these templates is Node/Docker.

## CI

If the `ci:github` overlay was selected, `.github/workflows/ci.yml` runs install,
lint, typecheck, Jest, build, and Playwright. Commit the lockfile for frozen
installs; the workflow falls back to a normal install before one exists.

## Rollback

1. Redeploy the previous build.
2. Only roll back database migrations when the previous release is compatible
   with the restored schema; otherwise roll forward with a fix.
3. Rotate secrets if auth or database credentials were exposed.
4. Monitor `/api/health`, auth error rates, and PostHog opt-in metrics.

## Post-deploy verification

- [ ] Public URL and metadata render correctly.
- [ ] `/api/health` is healthy.
- [ ] Signup → email OTP → verify → login works.
- [ ] Forgot password → reset OTP → new password works.
- [ ] Session cookies are `Secure`, `HttpOnly`, and scoped correctly.
- [ ] PostHog does not capture before consent.
- [ ] Database backups and migration history are current.
- [ ] CI is green on `main`.
