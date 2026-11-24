# UI

Read `.agents/skills/gpui-kit-design-guides/SKILL.md` before any screen change.

The app uses GPUI Kit components and semantic theme tokens. `crates/ui` holds shared theme helpers and reusable components; feature views stay in `crates/app`. Store stateful GPUI Kit component state as `Entity<T>` in the owning model/view. Use domain-derived `ElementId`s. Keep command palette, dialogs, forms, and tables accessible and keyboard reachable.

## Shell

```text
┌ title bar ── logo · app name ···························· [⌕ Search or run a command  Ctrl K] [☀] ┐
│ sidebar (w_60) │ content column (max_w_5xl, px_8 py_6), scrolls                                     │
│  icon · label  │   PageHeader (title, subtitle, one primary action)                                 │
│                │   SectionCard / StatCard / Table / EmptyState / StatusPage …                       │
└ status bar ── ● Local ································································ version ┘
```

- `shell.rs` owns the frame, theme switching (`sync_theme`, `toggle_theme`),
  and the command palette. Pages own their `PageHeader`; the shell draws none.
- Navigation: sidebar items and `cmd-1`…`cmd-6`; `cmd-k` command palette;
  `cmd-shift-l` theme toggle.

## Recipes

### Add a page

1. Add a `Pane` variant (title key, `IconName` in `Pane::icon`, position in
   `Pane::available`) and an action in the `actions!` list with a keybinding in
   `crates/desktop/src/main.rs`.
2. Create `crates/app/src/pages/<page>.rs.tmpl` with
   `pub(crate) fn <page>_body(&mut self, cx) -> AnyElement`, register it in
   `pages/mod.rs.tmpl`, and route it in `pane_body`.
3. Start with `PageHeader`, then `SectionCard`s. Handle errors and empties per
   `docs/STATES.md`.
4. Add strings to both locale files.

### Add a shared component

Follow the sourcing order below. A wrapper in `crates/ui/src/components/`
gets a builder API (`new`, chainable setters, `#[must_use]`), reads
`cx.theme()`, uses rem helpers, and is re-exported from `crates/ui/src/lib.rs`.
List it in `DESIGN.md`.

### Change or add a colour

Edit `crates/ui/themes/smokeforge.json` for **both** themes, keep the hex
values identical to the egui template's palette when the role exists there,
add the pair to the contrast tests in `crates/ui/src/theme.rs`, and update the
table in `DESIGN.md`.

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

