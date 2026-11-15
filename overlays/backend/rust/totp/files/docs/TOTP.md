# TOTP

TOTP enrollment, recovery codes, and second-factor login use the authenticated
auth service.

- `GET /api/v1/auth/totp/status` reports whether TOTP is enabled.
- `POST /api/v1/auth/totp/setup` creates a secret and `otpauth://` URL.
- `POST /api/v1/auth/totp/enable` verifies the first code and returns ten
  one-time recovery codes.
- Password login returns `202` with a short-lived `challenge` when TOTP is
  enabled. `POST /api/v1/auth/login/totp` completes the login.
- `POST /api/v1/auth/totp/disable` verifies a TOTP or recovery code and turns
  the factor off.
- `POST /api/v1/auth/totp/verify` verifies a code for an enabled factor.

TOTP uses SHA-1, six digits, and a 30-second period with one period of drift.
Recovery codes are stored as SHA-256 hashes and can be used once. Challenge
tokens are short-lived JWTs and are not access tokens. Never log a secret, code,
recovery code, or challenge.
