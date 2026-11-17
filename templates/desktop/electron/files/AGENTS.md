# AGENTS.md — generated Electron desktop app

This project was scaffolded by smokeforge. Read [CONTEXT.md](CONTEXT.md) before
changing code.

## Stack

- Electron 45, React 19, TypeScript 5.9
- electron-vite, Vite, electron-builder
- Tailwind CSS 4, Zod
- TanStack Query and Zustand when selected
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
- Keep main-process services testable and free of unnecessary Electron imports.
- Use the shared result type for renderer/main calls.
- Read and invoke the project-scoped skills before changing relevant code:

- `electron-best-practices`
- `zod`
- `vercel-react-best-practices`
- `vercel-composition-patterns`
- `frontend-design`
- `tailwind-design-system`
- `web-design-guidelines`
- `animation-vocabulary`
- `zustand` when state is selected

`skills-lock.json` records the sources and content hashes.
