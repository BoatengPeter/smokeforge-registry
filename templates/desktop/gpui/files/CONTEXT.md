# Engineering rules — GPUI desktop app

- `crates/core`, `crates/api-client`, and `crates/storage` must not depend on GPUI.
- `crates/ui` owns GPUI Kit theme tokens and reusable components.
- `crates/app` owns entities, models, views, actions, keybindings, and async state.
- `crates/desktop` owns application bootstrap, assets, logging, and panic handling.
- Read `docs/DEVELOPMENT.md` for the `make dev` watch loop.
- Read `docs/BUILD.md`, `docs/ARCHITECTURE.md`, `docs/AUTH.md`, and `docs/SYNC.md` when changing build, architecture, auth, or sync behavior.

Read `.agents/skills/gpui-kit/SKILL.md`, `.agents/skills/gpui-kit-design-guides/SKILL.md`, and the matching Rust skills before work.
- Never guess a GPUI API; check the pinned source or GPUI Kit docs.
- Keep `render` cheap; no I/O, locks, or network calls.
- Store `Subscription` and `Task` handles.
- Use stable domain-derived `ElementId`s. Never use list indexes.
- API refresh tokens live in the OS keyring; access tokens are memory-only.
- Use the shared sync contract: idempotency, version checks, cursors, retries, manual conflicts.

## Component sourcing and creation

Use this order before writing UI:

1. **Check GPUI Kit first.** Search the installed `gpui-kit` skill, `https://gpui-kit.com/llms.txt`, and the specific component page at `https://gpui-kit.com/component/<name>.md`. Prefer the existing `Sidebar`, `TitleBar`, `StatusBar`, `Button`, `Input`, `Select`, `Dialog`, `Command`, `DataTable`, `VirtualList`, `Resizable`, and other shipped components.
2. **Check the pinned source.** If the docs are ambiguous, inspect the `gpui-kit` source in `~/.cargo/registry/src` for the exact pinned version. Do not invent an API from memory.
3. **Wrap in `crates/ui`.** If the product needs a repeated visual or behavior, add a thin project wrapper/component there using `cx.theme()` tokens and GPUI Kit behavior. Feature views must not fork component behavior.
4. **Create only when missing.** If GPUI Kit has no suitable component, create it in `crates/ui` following the Design Guides. It must have a small builder-style public API, semantic theme tokens, stable `ElementId`s, keyboard/focus behavior, and tests.
5. **Never paste arbitrary third-party code into a feature view.** Check the license, pin it, and keep shared primitives in `crates/ui`.

Component state belongs in an `Entity<T>` owned by the view or model. Do not keep state in `render`.


## UI composition

- Theme: `crates/ui/themes/smokeforge.json` + `crates/ui/src/theme.rs`. See `DESIGN.md`.
- States (errors, empties, degraded, confirmations): `docs/STATES.md`.

- Shared presentational components live in `crates/ui/src/components`; shell composition lives in `crates/app/src/shell.rs`.
- Feature pages live in `crates/app/src/pages`, one file per page.
- `crates/app/src/lib.rs` is wiring only: application state, actions, and shell composition.
- Prefer `RenderOnce` for stateless components. Use `Entity<T>` only for retained state.
- A page or component should stay under roughly 200 lines; split sections into child components before adding behavior.
- Use GPUI Kit components directly for standard controls. Add a wrapper in `crates/ui` only for a repeated project-specific composition or visual variant; do not rebuild kit behavior.

## Non-negotiable skills

Before writing or changing any GPUI code or UI, you **MUST** invoke and read both project skills:

1. `.agents/skills/gpui-kit/SKILL.md`
2. `.agents/skills/gpui-kit-design-guides/SKILL.md`

Do not start implementation until both are loaded and their Coding Guides and Design Guides have been read. If context is lost, reload both. Also invoke the matching Rust skills when changing async, patterns, tests, or review-sensitive code.

