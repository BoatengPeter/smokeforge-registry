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
