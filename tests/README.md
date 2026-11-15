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
