# Idempotency and versioning

## Client rules

- Generate the item UUID before the first local write.
- Generate one idempotency key per outbox operation.
- Reuse that key for transport retries.
- Use a new key when client-wins resolution creates a replacement operation.
- Send the last acknowledged server version in `If-Match`.

## Server rules

- `idempotency_keys` is unique by `(user_id, key)`.
- Store the request hash, response status, and response body.
- Replay the original response for a matching request hash.
- Reject a reused key with a different hash as `409 idempotency_key_reused`.
- Store the mutation, change-feed row, and idempotency response in one
  transaction.
- Expire keys after 24 hours.
- Return `409` for stale `If-Match`; do not apply the mutation.
- Increment `version` exactly once per accepted mutation.
- Write a tombstone for every delete.
