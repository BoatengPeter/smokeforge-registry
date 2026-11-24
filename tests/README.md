# Registry verification

`verify-templates.sh` builds generated Go projects from this registry and
validates them.

`verify-rust-templates.sh` builds `rust/axum` with the base and `api` presets,
runs rustfmt, clippy, tests, and the release build, and checks database, auth,
Docker, CI, and Rust skill files.

`verify-android.sh` generates standalone and fullstack Android projects and runs
their tests, lint, and debug APK builds. It requires `SMOKEFORGE_BIN`, an Android
SDK, JDK 17, and `GRADLE_BIN` or the generated Gradle wrapper.

`verify-ios.sh` generates standalone and fullstack iOS projects, generates the
Xcode project with XcodeGen, runs Swift package tests, and builds for the iOS
Simulator. It requires macOS, Xcode, XcodeGen, and `SMOKEFORGE_BIN`.

`verify-workspace.sh` generates composable workspace projects, including the
all-client shape, API + desktop without web, client-only, and API-only. It checks
root agent files, skills, docs, Make targets, CI jobs, and selected-only app
references.

All scripts require `SMOKEFORGE_BIN` pointing to a built smokeforge binary.
The Rust script also requires `cargo` and a Rust toolchain.

```bash
SMOKEFORGE_BIN=../smokeforge/bin/smokeforge ./tests/verify-rust-templates.sh
```

## Live desktop sync E2E

The generated desktop suite includes an opt-in sync test. Set
`SYNC_E2E_API_URL`, `SYNC_E2E_EMAIL`, and `SYNC_E2E_PASSWORD` after starting a
migrated and seeded Go/Rust API. Without those variables the test is skipped.

The test signs into the desktop, creates a Library item, and polls the API
snapshot until the item is present.
