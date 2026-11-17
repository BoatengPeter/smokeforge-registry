# TanStack Query

TanStack Query owns renderer server state. It calls `window.desktopApi`, not
`fetch` directly, so tokens and API access stay in the main process.

- `useSystemVersion()` reads app/system info through the preload bridge.
- `useSystemPing()` exercises a mutation.
- Query keys live in `renderer/src/lib/query/keys.ts`.
- Do not copy query data into Zustand or component state.

The main process remains the API/auth boundary; TanStack Query is a cache, not a
security boundary.
