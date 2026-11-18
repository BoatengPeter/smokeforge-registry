# Updates

Auto-update uses `electron-updater` with the existing electron-builder setup. It
is inactive in development and in unpacked builds.

## Release requirements

- Build and publish from a tagged, versioned commit.
- Configure a publish provider in `electron-builder.yml` (GitHub Releases or a
  generic HTTPS endpoint).
- Sign macOS builds with a Developer ID, harden the runtime, notarize the app,
  and ship the ZIP target used by the updater.
- Sign Windows NSIS builds. Unsigned updates trigger warnings and should not be
  shipped.
- Linux AppImage supports auto-update; `.deb` installs are updated by the system
  package manager.
- Publish `latest.yml`/`latest-mac.yml`, blockmaps, installers, and ZIP assets
  together.

## Runtime behavior

- Update checks run only inside a packaged app.
- The main process owns `autoUpdater`; the renderer sees typed status events.
- The renderer never sees update credentials or download URLs.
- Download is user-initiated, and install requires an explicit restart.
- `settings.autoCheckUpdates` and `settings.updateChannel` are stored locally.
- Update errors are logged through electron-log and shown in Settings.

## Testing

Use a local update server and `dev-app-update.yml` with
`autoUpdater.forceDevUpdateConfig`, or publish a private pre-release channel.
Install an older signed build, check for the new version, download it, and
verify restart, release notes, and the stored "what's new" version.

## Security

Never use an unsigned update, plain HTTP, or a GitHub token embedded in the
application. Public GitHub Releases or a signed private endpoint are acceptable.
