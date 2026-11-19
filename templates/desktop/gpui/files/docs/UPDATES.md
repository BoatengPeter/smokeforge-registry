# Updates

Auto-update is explicitly out of scope for the initial GPUI template. Packaging
artifacts are built locally or in CI; upgrading is an explicit user install.

When auto-update is added, it must:

- Verify signed release metadata.
- Keep the update client off the UI thread.
- Request a repaint when download or verification state changes.
- Never overwrite the local database without a migration and backup path.
