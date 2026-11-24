---
name: api-contract
description: Keep API, web, desktop, Android, and iOS clients aligned on auth, error, DTO, and environment contracts. Use when an API payload, auth flow, or shared error shape changes.
---

# API contract

## When to use

Use this skill when a change affects request or response shapes, authentication,
refresh behavior, error mapping, or environment variables shared by clients.

## Contract rules

- Treat the API schema and auth behavior as the source of truth.
- Keep access tokens short-lived and refresh tokens rotating.
- Never log tokens, passwords, full auth headers, or server secrets.
- Map transport, unauthorized, validation, and server errors explicitly in each
  client.
- Keep platform token storage platform-native: HTTP-only BFF cookies, Keychain,
  encrypted Android storage, or the desktop trusted process.
- Update every affected client and its app-local docs.

## Verification

Run the API contract tests and the affected client tests. Run `make test` when
more than one client consumes the changed contract.
