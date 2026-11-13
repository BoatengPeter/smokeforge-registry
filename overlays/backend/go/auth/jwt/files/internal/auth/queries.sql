-- name: CreateUser :one
INSERT INTO users (id, email, password_hash, role)
VALUES ($1, lower($2), $3, $4)
RETURNING id, email, password_hash, role, email_verified_at, created_at, updated_at;

-- name: GetUserByEmail :one
SELECT id, email, password_hash, role, email_verified_at, created_at, updated_at
FROM users
WHERE lower(email) = lower($1);

-- name: GetUserByID :one
SELECT id, email, password_hash, role, email_verified_at, created_at, updated_at
FROM users
WHERE id = $1;

-- name: MarkEmailVerified :exec
UPDATE users
SET email_verified_at = now(), updated_at = now()
WHERE id = $1;

-- name: UpdatePassword :exec
UPDATE users
SET password_hash = $2, updated_at = now()
WHERE id = $1;

-- name: CreateAuthToken :exec
INSERT INTO auth_tokens (id, user_id, kind, token_hash, expires_at)
VALUES ($1, $2, $3, $4, $5);

-- name: GetAuthTokenByHash :one
SELECT id, user_id, kind, token_hash, expires_at, used_at, created_at
FROM auth_tokens
WHERE token_hash = $1;

-- name: ConsumeAuthToken :execrows
UPDATE auth_tokens
SET used_at = now()
WHERE id = $1 AND used_at IS NULL;

-- name: RevokeAuthToken :exec
UPDATE auth_tokens
SET used_at = now()
WHERE id = $1 AND used_at IS NULL;

-- name: RevokeUserTokens :exec
UPDATE auth_tokens
SET used_at = now()
WHERE user_id = $1 AND kind = $2 AND used_at IS NULL;

-- name: DeleteExpiredTokens :exec
DELETE FROM auth_tokens
WHERE expires_at < now();

-- name: GetOAuthAccount :one
SELECT user_id
FROM oauth_accounts
WHERE provider = $1 AND provider_account_id = $2;

-- name: CreateOAuthAccount :exec
INSERT INTO oauth_accounts (id, user_id, provider, provider_account_id, email)
VALUES ($1, $2, $3, $4, $5);
