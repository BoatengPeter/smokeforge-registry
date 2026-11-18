# AGENTS.md — generated Electron desktop app

This project was scaffolded by smokeforge. Read [CONTEXT.md](CONTEXT.md) before
changing code and [DESIGN.md](DESIGN.md) before visual work.

## Stack

- Electron 45, React 19, TypeScript 5.9
- electron-vite, Vite, electron-builder
- React Router 8, Tailwind CSS 4, shadcn/ui, Zod, React Hook Form
- Drizzle ORM + drizzle-kit on better-sqlite3; electron-log/updater/window-state
- TanStack Query, Table, Virtual, Zustand; Recharts; i18next
- Vitest; Playwright Electron when selected

## Layout

```text
src/main/       trusted process: lifecycle, windows, security, IPC, features, storage
src/preload/    contextBridge API only
src/renderer/   sandboxed React application
src/shared/     IPC schemas, types, and constants shared by both sides
tests/          unit, IPC, and Playwright Electron tests
docs/           architecture, security, packaging
```

## Rules

- Never expose raw `ipcRenderer`, Node primitives, filesystem, or shell access to
  the renderer.
- Keep `contextIsolation`, `sandbox`, and `webSecurity` enabled; never enable
  `nodeIntegration`.
- Validate every IPC payload in main with Zod.
- Add UI primitives with the shadcn CLI, never by hand. Read
  `docs/LIBRARIES.md` before adding a dependency.
- Keep main-process services testable and free of unnecessary Electron imports.
- Use the shared result type for renderer/main calls.
- Use the palette in `src/renderer/src/styles/globals.css`; never add raw hex or
  named palette colors in components.
- Read and invoke the project-scoped skills before changing relevant code:

- `electron-best-practices`
- `zod`
- `react-router`
- `drizzle`
- `frontend-ui-engineering`
- `vercel-react-best-practices`
- `vercel-composition-patterns`
- `frontend-design`
- `tailwind-design-system`
- `web-design-guidelines`
- `animation-vocabulary`
- `zustand` when state is selected

`skills-lock.json` records the sources and content hashes. Use
`npx skills add https://github.com/addyosmani/agent-skills --skill frontend-ui-engineering`
to refresh the component-engineering skill. Read it before writing renderer
components; keep feature files modular and avoid one-file screens that mix
routing, data, forms, and presentation.
