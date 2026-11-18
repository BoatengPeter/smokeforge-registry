# Tray icon

The optional `shell:tray` feature adds a native tray icon. Menu actions use the
same command IDs as the renderer and native application menu.

## Behavior

- Clicking the tray toggles the main window.
- Closing the last window leaves the app running while the tray exists.
- New library item and Settings open through the command channel.
- Quit exits the process and removes the tray.

Tray support depends on the Linux desktop environment. Verify AppIndicator
support on the target distribution; Windows and macOS use native tray APIs.
