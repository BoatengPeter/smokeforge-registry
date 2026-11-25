# UI

Read `.agents/skills/gpui-kit-design-guides/SKILL.md` before any screen change.

The app uses GPUI Kit components and semantic theme tokens. `crates/ui` holds shared theme helpers and reusable components; feature views stay in `crates/app`. Store stateful GPUI Kit component state as `Entity<T>` in the owning model/view. Use domain-derived `ElementId`s. Keep command palette, dialogs, forms, and tables accessible and keyboard reachable.

## Shell

```text
┌ [logo] App    [⌕][◧] ┬ title bar ··································· [☀] window controls ┐
│ Workspace         [+] │ content column (max_w 72rem, px_8 py_6), scrolls                      │
│  ▣ Dashboard          │   PageHeader (title, subtitle, one primary action)                    │
│  ▤ Library            │   SectionCard / StatCard / Table / EmptyState / StatusPage …          │
│  ◔ Activity           │                                                                        │
│ ───────────────────── │                                                                        │
│  ⚙ Settings           │                                                                        │
│  ⓘ About              ├ status bar ── ● Local ····································· version ┘
└ Sidebar 15rem / rail ─┘
```

- `shell.rs` owns the frame, theme switching (`sync_theme`, `toggle_theme`),
  interface size (`sync_zoom`, `set_zoom`), the "Go to…" window, and the
  welcome tour layer.
- Sidebar (GPUI Kit `Sidebar`, full window height; the title and status bars
  span the content column): the header has the logo and name (they drag the
  window), a search button, and a collapse button; a `SidebarGroup` "Workspace"
  with a "+" suffix (`NewRecord`); Settings and About in the `SidebarFooter`
  (`Pane::in_sidebar_footer`). `settings.sidebar_open` persists the state;
  collapsed, GPUI Kit draws an icon rail. `secondary-b` toggles it.
- "Go to…" (sidebar search, `secondary-k`): the ui crate's `PickerFrame`
  (title, Esc, 40rem wide, 2.5rem from the top) around a GPUI Kit `Command`
  with a Pages and an Actions group. Every row carries the action it
  dispatches (`go_to::page_action`, `go_to::ACTIONS`), so a row and its
  shortcut do the same thing. Add an action as a row in `go_to::ACTIONS`. Pages own their `PageHeader`; the shell draws none.
- Keyboard (`secondary` = Ctrl on Windows/Linux, Cmd on macOS):
  `secondary-1`…`6` pages, `secondary-k` command palette, `secondary-shift-l`
  theme, `secondary-=`/`-`/`0` interface size. All bindings live in
  `crates/app/src/keymap.rs` and load with `KeyBinding::load` (`KeyBinding::new`
  panics on `secondary`). Show a shortcut with `Kbd::global_binding_for_action`,
  never a typed string; `keymap::tests` loads every binding.
- Interface size is `settings.zoom` (tokens in `ui::theme::zoom`). It sets the
  theme's font size, which is the rem, so every rem-based size scales.
  `Theme::change` resets it, so `sync_theme` forces a re-apply.

## Welcome tour

```text
┌ [logo] Welcome to App  v0.1.0 ················· Drag to move  [×] ┐  header: drags the window
│ Title / paragraph                                                    │
│ [icon] Feature title / body   × 3                                    │  body: scrolls, fixed height
│ Appearance  [ System | Light | Dark ]                                 │
│ Size        [ −  130%  + ]                                            │
├──────────────────────────────────────────────────────────────────────┤
└ ▬ • •                                                Back  [Next →] ┘
```

- Opens on first launch (`settings.onboarded == false`) and from Settings ›
  Help (`show_welcome`, or the `ShowWelcome` action). Any close sets
  `onboarded` and logs the `WelcomeOutcome`.
- Split by layer: `crates/app/src/welcome.rs` is state (`WelcomePage`,
  `WelcomeOutcome`); `crates/ui` `WizardWindow` + `WindowDrag` +
  `wizard_size` are the reusable chrome and drag logic; `pages/welcome.rs`
  owns the transitions and draws pages from `FeatureRow`, `Inset`,
  `ValueStepper`, and the shared `theme_choice` / `size_stepper` builders in
  `pages/mod.rs`.
- It floats over the live app with no backdrop. The header starts the drag;
  the shell root forwards mouse moves and the release to `welcome_drag`, which
  keeps the window on screen.
- Keys go to `welcome_focus`: Escape skips; Enter / → / ← move only while the
  tour itself (not one of its buttons) has focus.
- **Add a page:** add a `WelcomePage` variant and list it in `ALL`; add
  `welcome.*` strings to both locale files; add a match arm in
  `pages/welcome.rs` built from `crates/ui` components.
- **Another multi-step flow:** reuse `WizardWindow` and `WindowDrag` with its
  own page enum; do not copy the welcome page.

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

