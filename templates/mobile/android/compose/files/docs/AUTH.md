# Authentication

## Endpoints

The base uses the smokeforge API auth contract:

- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/auth/logout`
- `GET /api/v1/auth/me`
- `POST /api/v1/auth/forgot-password`

Login, refresh, and logout send `X-Refresh-Token-Transport: body` so native
clients receive and rotate refresh tokens without browser cookies.

## Session lifecycle

1. `AppViewModel` starts `restoreSession`.
2. `SessionManager` decrypts stored tokens.
3. `AuthRepositoryImpl` calls `auth/me` through the authenticated client.
4. The session becomes `SignedIn`; failures clear tokens and become `SignedOut`.
5. A `401` on a non-auth request triggers one refresh attempt.
6. A failed refresh clears the session and returns to sign-in.

## Security rules

- Access and refresh tokens are stored only in encrypted DataStore values.
- The AES-GCM key lives in Android Keystore.
- Passwords are never persisted.
- Authorization headers are never logged.
- Cleartext HTTP is allowed only in debug builds for local development.
- Release and staging builds should use HTTPS endpoints.

## Future overlays

`auth:android-oauth` will add Custom Tabs/AppAuth PKCE. `auth:android-totp` will
add the challenge, verification, and recovery flows. The base state machine keeps
the login result boundary ready for those additions.
