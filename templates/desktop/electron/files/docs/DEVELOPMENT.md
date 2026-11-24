# Development and hot reload

```bash
make install
make dev           # hot reload everything
make dev-inspect   # same, plus the V8 inspector on the main process
```

`make dev` runs `electron-vite dev --watch`. What happens on save depends on
which process owns the file:

| You edit | What reloads | State kept |
| --- | --- | --- |
| `src/renderer/**` (React, CSS) | Vite HMR / React Fast Refresh swaps the module in place | Component state, window, main process |
| `src/preload/**` | Preload is rebuilt and the renderer reloads | Window, main process; renderer state resets |
| `src/main/**`, `src/main/workers/**` | Main process is rebuilt and Electron restarts | Settings, SQLite data, window size/position |
| `src/shared/**` | Every side that imports the file follows the rules above | — |
| `index.html`, `electron.vite.config.ts`, `package.json` | Nothing automatically; stop and rerun `make dev` | — |

Fast Refresh keeps state only for modules that export React components. A
module that also exports non-component values (constants, hooks, helpers)
causes a full page reload. Keep those in separate files.

## Content Security Policy in development

React Fast Refresh needs an inline preamble script and the Vite HMR
websocket. The renderer CSP is defined once in `src/shared/csp.ts`:

- `make dev` uses `script-src 'self' 'unsafe-inline'` and
  `connect-src 'self' http://localhost:* ws://localhost:*`.
- `make build` / `make package` use `script-src 'self'` and
  `connect-src 'self'`.

`electron.vite.config.ts` writes the right policy into
`src/renderer/index.html`, and `src/main/security/index.ts` sends the same
policy as a header. Edit `src/shared/csp.ts`, never the meta tag by hand. If
the browser console shows `@vitejs/plugin-react can't detect preamble`, the
development policy is not being applied.

## Debugging the main process

`make dev-inspect` starts the main process with `--inspect` on port 5858.
Attach Chrome (`chrome://inspect`) or VS Code ("Attach to Node process",
port 5858). The inspector reattaches after each main-process restart.
Renderer DevTools open with the usual shortcut; they are disabled in packaged
builds.

## Troubleshooting

- **App starts twice or the second window closes immediately.** The single
  instance lock keeps only one copy running. Quit any packaged or
  `make start` copy of the app before `make dev`.
- **Port 5173 is in use.** Vite picks the next free port and Electron follows
  it automatically.
- **Changes to `.env` are ignored.** Environment is read at main-process start;
  rerun `make dev`.

## Dock and taskbar icon

On Linux (Wayland, GNOME/KDE) the dock does not use the window icon. It
matches the window's `app_id` to a desktop entry of the same name and shows
that entry's icon; with no match it shows a generic one.

- `package.json` `"desktopName": "<bundle id>.desktop"` makes Electron send the
  bundle id as its `app_id` in dev and in packaged builds. (The `--class`
  switch does not change it.)
- `make dev` runs `scripts/install-dev-desktop.sh`, which installs
  `~/.local/share/applications/<bundle id>.desktop` pointing at
  `resources/icon.png` by absolute path. Run `make desktop-entry` alone to
  refresh it after changing the icon.
- Packaged Linux builds get the same match from `electron-builder.yml`
  (`linux.desktop.entry.StartupWMClass`).
- macOS dev builds set the dock icon in `bootstrap.ts`; Windows uses the
  `BrowserWindow` icon.

To check what a running window reports, start it with `WAYLAND_DEBUG=1` and
look for `xdg_toplevel.set_app_id`.
