# CLAUDE.md — egui desktop app

Read [CONTEXT.md](CONTEXT.md), [AGENTS.md](AGENTS.md), [DESIGN.md](DESIGN.md), and
[README.md](README.md) before changing code.

## Required skills

Before UI or component work, read and follow
`.agents/skills/egui-component-engineering/SKILL.md` and its `REFERENCE.md`.
For Rust work, read the matching vendored skill:

- `rust-async-patterns` for Tokio, workers, channels, and cancellation.
- `rust-patterns` for ownership, errors, modules, and API design.
- `rust-testing` for tests, fixtures, and async test patterns.
- `rust-best-practices-apollo` and `rust-best-practices-beautifulrust` for review.

## Non-negotiables

- Keep domain logic in `crates/core`, persistence in `crates/storage`, API access
  in `crates/api-client`, and UI/actions in `crates/app`.
- Never block the egui frame loop with database, file, network, lock, or async
  work. Request a repaint when background work finishes.
- Use theme tokens; no raw colors in components.
- Use stable widget IDs and `push_id`/`id_salt` in loops.
- Tokens belong in the OS keyring, never JSON or SQLite.
- Read `DESIGN.md` before visual changes, `docs/UI.md` before shell, page, or
  component changes, and `docs/STATES.md` before adding any error, empty,
  loading, offline, or confirmation state.
- Reuse `crate::components` and `crate::icons`; add new primitives there, never
  inline in a page. Document every new token, icon, component, page, or state in
  the matching doc in the same change.

## Local development

use `make dev` (cargo-watch rebuild + restart on save) or `make watch-check`; see `docs/DEVELOPMENT.md`. Watch targets are for humans, not for verification: run the quality gate below before reporting completion.

## Quality gate

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

A failing command blocks completion. Report actual output; do not claim success
without fresh evidence.
