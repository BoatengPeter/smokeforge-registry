-- name: GetTOTPState :one
SELECT totp_secret, totp_enabled
FROM users
WHERE id = $1;

-- name: SetTOTPSecret :exec
UPDATE users
SET totp_secret = $2, totp_enabled = false, updated_at = now()
WHERE id = $1;

-- name: EnableTOTP :exec
UPDATE users
SET totp_enabled = true, updated_at = now()
WHERE id = $1;

-- name: DisableTOTP :exec
UPDATE users
SET totp_secret = NULL, totp_enabled = false, updated_at = now()
WHERE id = $1;

-- name: DeleteRecoveryCodes :exec
DELETE FROM totp_recovery_codes
WHERE user_id = $1;

-- name: CreateRecoveryCode :exec
INSERT INTO totp_recovery_codes (id, user_id, code_hash)
VALUES ($1, $2, $3);

-- name: GetRecoveryCode :one
SELECT id
FROM totp_recovery_codes
WHERE user_id = $1 AND code_hash = $2 AND used_at IS NULL;

-- name: ConsumeRecoveryCode :execrows
UPDATE totp_recovery_codes
SET used_at = now()
WHERE id = $1 AND used_at IS NULL;
