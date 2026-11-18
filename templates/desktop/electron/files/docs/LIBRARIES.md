# Libraries and platform choices

This app shell deliberately uses a small set of composed libraries. Add a
library only when a screen or a main-process capability needs it.

## Preferred defaults

- **Routing:** React Router 8 in Data Mode with `createHashRouter`. Hash routing
  works with `app://bundle/index.html` and needs no server. The project-scoped
  `react-router` skill is the source of truth for routing changes.
- **UI primitives:** shadcn/ui components, generated from the registry. Never
  hand-write a shadcn primitive. Add new ones with the package-manager command:

  ```bash
  pnpm dlx shadcn@latest add <component> --yes
  # npm: npx shadcn@latest add <component> --yes
  # bun: bunx --bun shadcn@latest add <component> --yes
  ```

- **Component architecture:** use `frontend-ui-engineering`. Keep one clear
  responsibility per file, colocate feature code, separate data/container logic
  from presentation, and avoid one-file screens that mix routing, data, forms,
  and rendering.

## UI

| Concern | Choice | Notes |
| --- | --- | --- |
| Styling | Tailwind CSS 4 | CSS-first semantic tokens in `src/renderer/src/styles/globals.css`; native chrome colors in `src/shared/design-tokens.ts`. |
| Components | shadcn/ui + Radix | Generated code lives in `src/renderer/src/components/ui`; the CLI owns primitives. |
| Icons | lucide-react | One icon family; do not hand-roll SVG paths. |
| Toasts | sonner | Use the generated shadcn `Toaster`. |
| Command palette | cmdk | Wrapped by the generated shadcn Command component. |
| Resizable panes | react-resizable-panels | List/detail inspector layout. |
| Client UI state | Zustand | Local UI only, never server state or secrets. |
| Main-process data | TanStack Query | Cache, loading, errors, mutation invalidation. |
| Forms | React Hook Form + Zod | Shared validation at IPC and form boundaries. |
| Tables | TanStack Table | Column definitions, sorting, and header behavior. |
| Virtualization | TanStack Virtual | Long Library lists render only visible rows. |
| Charts | Recharts | Dashboard chart via the generated shadcn Chart wrapper. |
| i18n | i18next + react-i18next | English locale ships; UI text uses translation keys. |
| Dates | date-fns | Relative timestamps and local formatting. |
| Drag and drop | dnd-kit | Dashboard cards and file drop surfaces. |
| Motion | motion | Small route transitions; respect reduced motion. |

## Main process and data

| Concern | Choice | Notes |
| --- | --- | --- |
| Local database | better-sqlite3 + Drizzle ORM | Typed relational data with explicit migrations. Fullstack desktop adds a sync outbox, cursor, and conflict table. |
| Migrations | drizzle-kit | Read the project-scoped `drizzle` skill. Generate SQL with `pnpm db:generate`; review and commit it. Never use `push` against user data. |
| Migrations at runtime | drizzle migrator | Bundled under `drizzle/` and applied from the main process at startup. |
| Settings | typed atomic JSON store | Deliberate replacement for `electron-store`: no ESM-only runtime dependency, Zod validation, atomic writes, and corrupt-file quarantine. Use it for non-sensitive preferences only. |
| Secrets | Electron `safeStorage` | Refresh tokens only. Never use `electron-store` encryption or SQLite as a secret store. Do not use keytar. |
| Logging | electron-log | Rotating main log with a redaction wrapper in `infra/logger.ts`. |
| HTTP | Electron `net.fetch` | Inherits system proxy settings; auth network calls stay in main. |
| Validation | Zod | Every IPC payload, settings patch, form, import row, and deep-link callback. |
| Files | fs-extra | Import/export and diagnostics. |
| File watching | chokidar | Intentionally not enabled in the default shell; import/export covers the safe path. Folder watching is highly app-specific. |
| Queues | p-queue | Serializes Activity jobs in main. |
| Schedules | node-cron | Opt-in daily SQLite WAL checkpoint at 03:00 through `scheduledMaintenance`; off by default. |
| Window state | electron-window-state | Restores bounds and rejects off-screen positions. |
| Tray | Electron `Tray` | Opt-in `shell:tray` feature; desktop-specific icon and menu. |
| Detached window | Electron `BrowserWindow` | Library detail window shares typed IPC and query invalidation. |
| Updates | electron-updater | Service is packaged-build only; configure signing and publish metadata before enabling. |
| OAuth | oauth4webapi | Opt-in `auth:oauth-pkce`; system browser + deep link + safeStorage. |

## Explicit non-choices

- No `keytar`: archived and unnecessary when `safeStorage` exists.
- No renderer filesystem, shell, Node, or token access.
- No raw `ipcRenderer` in renderer code.
- No second server database inside SQLite.
- No ORM or query builder calls outside main-process repositories.
