# Development and watch mode

Rust has no in-process hot module replacement, so the egui app uses a fast
edit → rebuild → restart loop driven by
[cargo-watch](https://github.com/watchexec/cargo-watch).

```bash
make dev-tools     # once: installs the pinned cargo-watch
make dev           # run the app; rebuild and restart it on every save
make watch-check   # re-run `cargo check --workspace --all-targets` on save
make watch-test    # re-run `cargo test --workspace` on save
make run           # run once, no watching
```

## What triggers a rebuild

| Path | Why |
| --- | --- |
| `crates/**` | all Rust sources, `locales/*.yml` (compiled in by rust-i18n), SQL migrations (embedded by refinery) |
| `Cargo.toml` | workspace dependencies and lints |
| `assets/**` | icons and images embedded with `include_bytes!` |

`target/`, `Cargo.lock`, docs, and the app's own data/config/log directories
are not watched, so the running app never retriggers itself. Change the
`WATCH_PATHS` variable in the `Makefile` to adjust this.

## What survives a restart

- Settings, the SQLite Library, the outbox, and keyring tokens live in the OS
  data/config directories, not in the project, so they persist.
- Window size/position and the egui_tiles layout are saved by eframe (on exit and every 30 seconds), so the most recent layout is restored after a restart.
- In-memory UI state (open dialogs, text being typed, scroll position) is
  lost. Keep anything you need across restarts in settings or SQLite.

## Keeping the loop fast

- Prefer `make watch-check` while restructuring code; switch to `make dev`
  when you want to see the UI.
- Most edits only touch `crates/app` (UI) or `crates/core`; the heavy
  dependencies stay cached in `target/`, so incremental rebuilds take seconds.
- On Linux, installing `mold` and adding
  `rustflags = ["-C", "link-arg=-fuse-ld=mold"]` under
  `[target.x86_64-unknown-linux-gnu]` in `.cargo/config.toml` cuts link time
  substantially. This is optional and local to your machine.

## Troubleshooting

- **`cargo-watch is not installed`.** Run `make dev-tools`.
- **The app window appears and immediately closes, or every save only focuses
  an old window.** The app is single-instance. A copy started from the desktop
  launcher or another terminal is still running, and the new process forwards
  to it and exits. Quit the other copy.
- **A panic message flashes and the app does not come back.** cargo-watch
  waits for the next save after a crash. The panic is printed in the terminal
  and also written to `<temp dir>/<crate_name>-panic.log`
  (see [CRASH.md](CRASH.md)).
- **A compile error stops the loop.** Fix it and save again; cargo-watch
  retries automatically.
