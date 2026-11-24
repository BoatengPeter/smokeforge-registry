# Storage

## Tokens

- `KeychainTokenStore` stores access and refresh tokens as one generic-password
  item under a service and account pair.
- Accessibility is `kSecAttrAccessibleAfterFirstUnlockThisDeviceOnly`.
- Saving replaces the existing item; clearing removes it.
- Missing tokens are a normal signed-out state; Keychain failures are not
  silently converted to successful reads.

## Preferences

- `ThemeStore` uses UserDefaults only for the non-sensitive appearance choice.
- Sensitive values must never use UserDefaults or `@AppStorage`.

## Future

A local database is intentionally absent. Offline SwiftData or GRDB
storage is an overlay and must remain behind repository interfaces.
