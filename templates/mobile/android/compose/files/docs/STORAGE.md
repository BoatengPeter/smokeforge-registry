# Storage

## Preferences

DataStore stores non-sensitive settings such as the selected theme. The app uses
a single preferences file.

## Tokens

- `SessionManager` owns the in-memory access and refresh tokens.
- `DataStoreTokenStore` persists ciphertext only.
- `KeystoreTokenCipher` uses AES/GCM/NoPadding with a key created in Android
  Keystore.
- The IV is stored with the ciphertext; the key is never exported.
- A failed decrypt is treated as no session and is cleared on the next auth
  attempt.

## Boundaries

- Feature modules never read DataStore directly.
- UI never receives raw tokens.
- Do not add a Room database until an offline overlay is selected. Repositories
  are the seam for adding local-first storage later.
