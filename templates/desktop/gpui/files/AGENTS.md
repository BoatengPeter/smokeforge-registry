# AGENTS.md — GPUI desktop app

Read `CONTEXT.md`, `DESIGN.md`, and `CLAUDE.md` before changing code.

## Required skills

Read and follow the installed project skills:

- `.agents/skills/gpui-kit/SKILL.md` — GPUI Kit setup, components, entities, async, focus, testing, coding guides.
- `.agents/skills/gpui-kit-design-guides/SKILL.md` — design and accessibility guides.
- `rust-async-patterns`, `rust-patterns`, `rust-testing`, `rust-best-practices-apollo`, `rust-best-practices-beautifulrust`.

Do not invent GPUI APIs. Search the pinned source or GPUI Kit documentation.

## Rules

- Keep GPUI out of `core`, `api-client`, and `storage`.
- Put reusable theme/components in `crates/ui`.
- Keep models and views separate; views render and emit actions.
- No blocking work in render or foreground spawn.
- Store subscriptions and tasks.
- Use semantic theme tokens, never raw colors.
- Commit `Cargo.lock`; never upgrade GPUI casually.

## Component sourcing and creation

Use this order before writing UI:

1. **Check GPUI Kit first.** Search the installed `gpui-kit` skill, `https://gpui-kit.com/llms.txt`, and the specific component page at `https://gpui-kit.com/component/<name>.md`. Prefer the existing `Sidebar`, `TitleBar`, `StatusBar`, `Button`, `Input`, `Select`, `Dialog`, `Command`, `DataTable`, `VirtualList`, `Resizable`, and other shipped components.
2. **Check the pinned source.** If the docs are ambiguous, inspect the `gpui-kit` source in `~/.cargo/registry/src` for the exact pinned version. Do not invent an API from memory.
3. **Wrap in `crates/ui`.** If the product needs a repeated visual or behavior, add a thin project wrapper/component there using `cx.theme()` tokens and GPUI Kit behavior. Feature views must not fork component behavior.
4. **Create only when missing.** If GPUI Kit has no suitable component, create it in `crates/ui` following the Design Guides. It must have a small builder-style public API, semantic theme tokens, stable `ElementId`s, keyboard/focus behavior, and tests.
5. **Never paste arbitrary third-party code into a feature view.** Check the license, pin it, and keep shared primitives in `crates/ui`.

Component state belongs in an `Entity<T>` owned by the view or model. Do not keep state in `render`.

## Non-negotiable skills

Before writing or changing any GPUI code or UI, you **MUST** invoke and read both project skills:

1. `.agents/skills/gpui-kit/SKILL.md`
2. `.agents/skills/gpui-kit-design-guides/SKILL.md`

Do not start implementation until both are loaded and their Coding Guides and Design Guides have been read. If context is lost, reload both. Also invoke the matching Rust skills when changing async, patterns, tests, or review-sensitive code.

