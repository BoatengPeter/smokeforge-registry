# TOTP

The TOTP overlay adds a second factor to the JWT auth overlay.

## Flow

1. `GET /api/v1/auth/totp/status` checks whether TOTP is enabled.
2. `POST /api/v1/auth/totp/setup` returns a secret and `otpauth://` URL.
3. Add the secret to an authenticator app.
4. `POST /api/v1/auth/totp/enable` confirms a current code and returns ten
   one-time recovery codes.
5. Password login returns `202` with a short-lived challenge when TOTP is
   enabled. `POST /api/v1/auth/login/totp` completes the login.
6. `POST /api/v1/auth/totp/disable` requires a current TOTP or recovery code.

All TOTP routes require a Bearer access token. Recovery codes are stored as
SHA-256 hashes and can be used once. Never log the secret, a code, or a recovery
code.
