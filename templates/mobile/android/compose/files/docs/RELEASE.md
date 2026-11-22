# Release

## Build types

- `debug`: `.debug` application ID suffix, local cleartext allowed.
- `staging`: `.staging` suffix, HTTPS endpoint, separate environment label.
- `release`: R8, resource shrinking, strict HTTPS, no debug tooling.

## Signing

Signing is optional for local builds. CI supplies:

```text
SMOKE_KEYSTORE_PATH
SMOKE_KEYSTORE_PASSWORD
SMOKE_KEY_ALIAS
SMOKE_KEY_PASSWORD
```

Never commit a keystore or credentials. The generated `.gitignore` excludes
keystores and `keystore.properties`.

## Release checks

1. Run `./gradlew test lint assembleDebug`.
2. Build the release bundle or APK with the signing environment configured.
3. Upload the mapping file to the crash-reporting provider when that overlay is
   installed.
4. Test against staging before promoting.
5. Use a staged rollout and watch crash/ANR metrics.

## Future overlays

Crash reporting, analytics, Play publishing, baseline profiles, and screenshot
tests are intentionally separate. Add them as overlays so the base stays
buildable without provider credentials.
