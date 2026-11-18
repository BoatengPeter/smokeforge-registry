# Registry verification

`verify-templates.sh` builds generated Go projects from this registry and
validates them.

`verify-rust-templates.sh` builds `rust/axum` with the base and `api` presets,
runs rustfmt, clippy, tests, and the release build, and checks database, auth,
Docker, CI, and Rust skill files.

Both scripts require `SMOKEFORGE_BIN` pointing to a built smokeforge binary.
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
