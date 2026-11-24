# AGENTS.md — egui desktop app

Read `CONTEXT.md` before changing code. Before any visual work read
`DESIGN.md` (tokens and rules), `docs/UI.md` (shell, components, recipes), and
`docs/STATES.md` (error, empty, offline, and confirmation states).

## Architecture

- `crates/core` — domain and validation; no egui or I/O frameworks.
- `crates/api-client` — generated OpenAPI client and HTTP adapters.
- `crates/storage` — SQLite, migrations, settings, keyring-facing persistence.
- `crates/app` — egui state, actions, views, theme, i18n.
- `crates/desktop` — eframe entry point and platform bootstrap.

## Design system (do not deviate)

- Colours, type, spacing, radii, and shadows come only from
  `crates/app/src/theme.rs`. No `Color32::from_rgb` or magic sizes elsewhere.
- Icons come only from `crates/app/src/icons.rs`. Add a variant there; never
  embed icon images for UI chrome.
- Widgets used in more than one place live in `crates/app/src/components/`.
  Pages in `crates/app/src/views/` compose them; they do not custom-paint.
- Every failure, empty, degraded, or destructive situation uses a pattern from
  `docs/STATES.md`. Never show an empty state when a read failed.
- One primary button per area. Badges are neutral unless the state has meaning.
- When you add a token, icon, component, page, or state, update `DESIGN.md`,
  `docs/UI.md`, or `docs/STATES.md` in the same change, and add a test.
- Check every visual change in light, dark, and high contrast.

## Rules

- Do not put business logic inside a widget closure.
- Do not block the UI thread.
- Pin exact egui/eframe versions and read the changelog before upgrades.
- Use the theme tokens; no raw component colors.
- Keep the renderer/API boundary in `main`/`api-client`; no tokens in UI state.
- Use `egui_kittest` accessibility checks for interactive widgets.
- Read and follow `.agents/skills/egui-component-engineering/SKILL.md` before component work.
- Read the matching Rust skill before changing that area:
  - `rust-async-patterns` — Tokio, channels, workers, cancellation, and background tasks.
  - `rust-patterns` — ownership, errors, module boundaries, and API design.
  - `rust-testing` — test layout, fixtures, async tests, and failure cases.
  - `rust-best-practices-apollo` — practical Rust review and correctness.
  - `rust-best-practices-beautifulrust` — idiomatic, maintainable Rust.
- `skills-lock.json` records the vendored Rust skill sources and hashes.
