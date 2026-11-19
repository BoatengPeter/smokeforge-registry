# CLAUDE.md — GPUI desktop app

Read [CONTEXT.md](CONTEXT.md), [AGENTS.md](AGENTS.md), [DESIGN.md](DESIGN.md), and [README.md](README.md) first.

Read the installed `.agents/skills/gpui-kit/SKILL.md` and `.agents/skills/gpui-kit-design-guides/SKILL.md` before UI work. Use the vendored Rust skills for async, testing, patterns, and review.

Never guess a GPUI or GPUI Kit API; inspect the pinned source/docs. Keep models separate from views, store every task/subscription, and never block the GPUI executor.

Quality gate:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Component sourcing and creation

Use this order before writing UI:

1. **Check GPUI Kit first.** Search the installed `gpui-kit` skill, `https://gpui-kit.com/llms.txt`, and the specific component page at `https://gpui-kit.com/component/<name>.md`. Prefer the existing `Sidebar`, `TitleBar`, `StatusBar`, `Button`, `Input`, `Select`, `Dialog`, `Command`, `DataTable`, `VirtualList`, `Resizable`, and other shipped components.
2. **Check the pinned source.** If the docs are ambiguous, inspect the `gpui-kit` source in `~/.cargo/registry/src` for the exact pinned version. Do not invent an API from memory.
3. **Wrap in `crates/ui`.** If the product needs a repeated visual or behavior, add a thin Smokeforge wrapper/component there using `cx.theme()` tokens and the GPUI Kit behavior. Feature views must not fork component behavior.
4. **Create only when missing.** If GPUI Kit has no suitable component, create it in `crates/ui` following the Design Guides. It must have a small builder-style public API, semantic theme tokens, stable `ElementId`s, keyboard/focus behavior, and tests.
5. **Never paste arbitrary third-party code into a feature view.** Check the license, pin it, and keep shared primitives in `crates/ui`.

Component state belongs in an `Entity<T>` owned by the view or model. Do not keep state in `render`.

