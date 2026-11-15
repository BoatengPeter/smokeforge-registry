# TOTP

TOTP enrollment and verification use the authenticated auth service.

- `POST /api/v1/auth/totp/setup` creates a secret and returns it for an
  authenticator app.
- `POST /api/v1/auth/totp/enable` verifies the first code and enables TOTP.
- `POST /api/v1/auth/totp/verify` verifies a code for an enabled factor.

Codes use SHA-1, six digits, and a 30-second period with one period of drift.
Login challenge integration is the next auth feature.
