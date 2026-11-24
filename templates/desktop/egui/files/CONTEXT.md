# Engineering rules — egui desktop app

- Keep UI thin. `crates/core` owns domain types, `crates/storage` owns persistence, `crates/api-client` owns HTTP, `crates/app` owns UI/actions, `crates/desktop` is the entry point.
- All views emit `Action`; apply business mutations in `DesktopApp::dispatch`, not inside widget closures.
- Never block the UI thread. Tokio/API/storage work runs off the frame loop and must call `Context::request_repaint`.
- Pin egui/eframe exactly. Read the upstream changelog before upgrading.
- Use one theme token module (`theme.rs`). Do not hardcode colors or sizes in feature files.
- UI layers: `theme.rs` tokens → `icons.rs` → `components/` → `shell.rs` and `views/`. See `DESIGN.md` and `docs/UI.md`.
- Keep `lib.rs` to app state, `Action`, and `dispatch`. A feature's own state types get their own module (`welcome.rs`), app-wide shortcuts live in `shortcuts.rs`, and reusable chrome goes in `components/` (`WizardWindow`), so views only compose.
- Error, empty, offline, not-found, and confirmation screens follow `docs/STATES.md`.
- Use stable widget IDs and `push_id` in loops.
- Store refresh tokens only in the OS keyring. Never in SQLite or JSON.
- Use manual conflict resolution by default for user-authored records.
- Read `.agents/skills/egui-component-engineering/SKILL.md` before building or changing any component. It defines the component API, immediate-mode/ID rules, accessibility bar, theme-token policy, and verification gate.
- Use the Rust skills that ship with this template:
  - `rust-async-patterns` for Tokio, channels, background work, and cancellation.
  - `rust-patterns` for ownership, error types, modules, and API design.
  - `rust-testing` for unit/integration tests and fixtures.
  - `rust-best-practices-apollo` and `rust-best-practices-beautifulrust` for Rust review and idiom guidance.
- Run `cargo fmt --all -- --check`, `cargo clippy --workspace --all-targets -- -D warnings`, and `cargo nextest run --workspace`.
