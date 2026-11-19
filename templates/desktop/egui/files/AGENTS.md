# AGENTS.md — egui desktop app

Read `CONTEXT.md` before changing code and `DESIGN.md` before visual work.

## Architecture

- `crates/core` — domain and validation; no egui or I/O frameworks.
- `crates/api-client` — generated OpenAPI client and HTTP adapters.
- `crates/storage` — SQLite, migrations, settings, keyring-facing persistence.
- `crates/app` — egui state, actions, views, theme, i18n.
- `crates/desktop` — eframe entry point and platform bootstrap.

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
