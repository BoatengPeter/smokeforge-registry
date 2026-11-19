# Packaging

The template ships `Packager.toml` for `cargo-packager`. Linux builds AppImage
and `.deb`; macOS and Windows must run the same config on their native runners
and add their platform formats before release.

```bash
cargo install cargo-packager --version 0.11.0
cargo packager --release
```

Before release:

- Set `version` in the workspace package metadata.
- Replace the placeholder bundle identifier and add platform icons under
  `assets/`.
- Build on each target OS; do not cross-package DMG/MSI from Linux.
- Sign and notarize macOS builds, and sign Windows installers, in CI secrets.
- Verify the app launches from the packaged artifact, not only `cargo run`.

Auto-update is intentionally out of scope for the initial template.

## GPUI native dependencies

Linux builds require the GPUI X11/Wayland and fontconfig development packages.
CI installs them before building; see `docs/BUILD.md`.
