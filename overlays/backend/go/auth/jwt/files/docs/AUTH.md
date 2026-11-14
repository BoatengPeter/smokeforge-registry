# Authentication

The JWT overlay provides email/password accounts, HS256 access tokens, rotating
refresh sessions, email verification, password reset, and optional TOTP.

## Package layout

- `internal/auth` contains application behavior, domain types, and ports.
- `internal/auth/http` contains Gin handlers, middleware, and OpenAPI transport.
- `internal/auth/postgres` contains the SQLC/pgx adapter.
- `internal/auth/email` sends SMTP mail or logs messages in development.
- `internal/auth/google` adapts Gothic to the auth application core.

The core package must not import Gin, Gothic, pgx, SMTP, or `net/http`. Keep new
transport and infrastructure behavior in an adapter package.

## Transport

- Access tokens are returned in `TokenResponse.access_token` and sent as
  `Authorization: Bearer <token>`.
- Browser clients receive the refresh token in a Secure, HttpOnly, SameSite=Lax
  cookie scoped to `/api/v1/auth`.
- Cookie-based refresh and logout require `X-Requested-With: XMLHttpRequest` to
  prevent cross-site request forgery.
- Native clients set `X-Refresh-Token-Transport: body` on login/refresh to
  receive the refresh token in JSON and avoid cookie storage.

Refresh tokens are opaque random values. Only SHA-256 hashes are stored. Every
refresh rotates the token; reusing a consumed token revokes the user's active
refresh sessions.

## Routes

| Route | Purpose |
| --- | --- |
| `POST /api/v1/auth/register` | Create an account |
| `POST /api/v1/auth/login` | Password login |
| `POST /api/v1/auth/login/totp` | Complete an MFA login |
| `POST /api/v1/auth/refresh` | Rotate access/refresh tokens |
| `POST /api/v1/auth/logout` | Revoke the refresh token |
| `GET /api/v1/auth/me` | Current account |
| `POST /api/v1/auth/verify-email` | Consume an email token |
| `POST /api/v1/auth/forgot-password` | Request a reset link |
| `POST /api/v1/auth/reset-password` | Consume a reset token |
| `GET /api/v1/auth/oauth/google` | Start Google OAuth |
| `GET /api/v1/auth/oauth/google/callback` | Complete Google OAuth |

## Configuration

`JWT_SECRET` must be at least 32 characters. The generator creates a local
secret. Production must set `COOKIE_SECURE=true`, configure SMTP, and either
require verification explicitly or leave `REQUIRE_EMAIL_VERIFICATION=auto`,
which requires it in production.

Local development logs verification and reset emails to the Zap logger instead
of sending them.

## Optional demo user

`make db-seed` creates a verified demo user when `SEED_USER_PASSWORD` is set.
Change `SEED_USER_EMAIL` from its default before using it anywhere non-local.

## Password policy

Passwords are 12-256 characters and hashed with Argon2id. Never log passwords,
tokens, OTPs, or recovery codes.

## TOTP

Select the `totp` feature to add enrollment, recovery codes, and second-factor
login. TOTP always requires this JWT overlay.

## Google OAuth

Google sign-in uses Gothic with the server-side Google provider.

Set:

```dotenv
GOOGLE_CLIENT_ID=...
GOOGLE_CLIENT_SECRET=...
# Optional. Defaults to PUBLIC_BASE_URL + /api/v1/auth/oauth/google/callback.
GOOGLE_REDIRECT_URL=
# Optional. Defaults to PUBLIC_BASE_URL.
OAUTH_SUCCESS_REDIRECT=
```

Routes:

- `GET /api/v1/auth/oauth/google` starts the flow.
- `GET /api/v1/auth/oauth/google/callback` completes it.

The callback stores the rotating refresh session in the HttpOnly cookie and
redirects to `OAUTH_SUCCESS_REDIRECT`. The frontend then calls
`POST /api/v1/auth/refresh` with `X-Requested-With: XMLHttpRequest` to obtain an
access token. Google OAuth is disabled in local development when credentials are
empty; production validation requires them.

The Google account ID is stored in `oauth_accounts`, linked to the user by
verified email on first sign-in.
