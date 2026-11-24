# Authentication

## Endpoints

- `POST /api/v1/auth/register`
- `POST /api/v1/auth/login`
- `POST /api/v1/auth/refresh`
- `POST /api/v1/auth/logout`
- `GET /api/v1/auth/me`
- `POST /api/v1/auth/forgot-password`

Login and refresh send `X-Refresh-Token-Transport: body` so native clients
receive and rotate refresh tokens without browser cookies.

## Session lifecycle

1. `RootView.task` calls `AuthRepository.restore()`.
2. The Keychain is read; no tokens means signed out.
3. `auth/me` validates the access token.
4. A `401` triggers one refresh attempt, then `auth/me` retries.
5. A failed refresh clears the Keychain and signs out.
6. Login saves both tokens before loading the user.
7. Logout revokes the refresh session and clears local state even if the API is
   unavailable.

## Security

- Tokens live in Keychain with a device-only accessibility class.
- Passwords are never persisted.
- Authorization headers and tokens are never logged.
- App Transport Security remains enabled; `Info.plist` allows local simulator
  networking while release endpoints use HTTPS.
- Google OAuth and TOTP are future overlays.
