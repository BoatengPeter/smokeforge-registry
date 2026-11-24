# Release

## Configurations

- Debug: local simulator endpoint, `.debug` bundle suffix.
- Staging: HTTPS staging endpoint, `.staging` suffix.
- Release: HTTPS production endpoint, production bundle ID.

## Signing

Signing uses Xcode automatic signing locally or CI-managed certificates. Do not
commit certificates, profiles, or secrets. No signing configuration is required
for simulator CI.

## Checks

1. `make ci` passes.
2. Replace the staging and release `API_BASE_URL` values in `project.yml`.
3. Configure a team and signing in Xcode.
4. Archive and upload to TestFlight.
5. Complete App Privacy, demo-account, and review notes before submission.

App Store delivery, TestFlight automation, dSYM upload, and phased rollout are
future release overlays.
