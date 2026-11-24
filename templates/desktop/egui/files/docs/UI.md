# UI

How the interface is built and how to extend it without drifting from the
design. Read `DESIGN.md` first for the tokens and rules, and
`docs/STATES.md` for error, empty, and confirmation states.

## Shell

```text
┌ title bar (44) ── logo · app name ······ [ Search or run a command  Ctrl K ] ······ theme · window controls ┐
│ sidebar │ tabs (egui_tiles) ─────────────────────────────────────────────────────────────────────────────── │
│ 220/52  │  canvas · centred page column (max 1200, gutter 28)                                               │
│         │     PageHeader                                                                                    │
│         │     cards / sections …                                                                            │
└ status bar (30) ── ● Local · Undo ··························································· Commands Ctrl K · v0.1.0 ┘
```

- `shell.rs` owns the frame: `Panel::top` title bar, `Panel::bottom` status
  bar, `Panel::left` sidebar, `CentralPanel` workspace, the command palette
  (`egui::Modal`), the confirmation dialog, and the welcome tour.
- The title bar is draggable and double-click maximizes. Window controls are
  drawn on Windows/Linux; macOS keeps native traffic lights.
- Navigation dispatches `Action::Navigate` and activates the matching tile.
  Panes are tabbed (`egui_tiles`) and rearrangeable; tab colours come from the
  palette in `PaneBehavior`.
- Keyboard: `Ctrl/Cmd+K` command palette (arrow keys + Enter), `Ctrl/Cmd+N`
  new record, `Ctrl/Cmd+B` toggle sidebar, `Ctrl/Cmd +`/`−`/`0` interface
  size, `Esc` closes overlays. Shortcuts are defined once in `shortcuts.rs`;
  anything that displays a shortcut formats those constants with
  `Context::format_shortcut`, never a hand-typed string.
- Interface size is `settings.zoom` (tokens in `theme::zoom`). `sync_zoom`
  applies it with `Context::set_zoom_factor`; egui's own keyboard zoom is
  turned off so the setting is the single source of truth.

## Welcome tour

```text
┌ [logo] Welcome to App  v0.1.0 ·················· Drag to move  × ┐  header (56): drags the window
│ Title                                                             │
│ paragraph …                                                       │  body: scrolls, fixed height
│ [icon] Feature title / body   × 3                                 │
│ ───────────────────────────────────────────────────────────────── │
│ Appearance  [ System | Light | Dark ]                              │
│ Size        [ −  130%  + ]                                         │
├───────────────────────────────────────────────────────────────────┤
└ ▬ • •                                              Back  [Next →] ┘  footer (64)
```

- Opens on first launch (`settings.onboarded == false`) and from Settings ›
  Help › Show welcome tour (`Action::ShowWelcome`). Any close sets
  `onboarded = true` and logs the `WelcomeOutcome` (finished or skipped).
- Split by layer: `welcome.rs` is state (`WelcomePage`, `WelcomeOutcome`),
  `components::WizardWindow` is the reusable chrome, and `views/welcome.rs`
  draws the pages from components. `DesktopApp.welcome: Option<WelcomePage>` is
  the only state; `WelcomeGo` and `CloseWelcome` change it.
- It floats over the live app with no backdrop, so settings changed in it
  (theme, size) take effect behind it and the user can drag it aside.
- Keys: `Esc` skips; `Enter`/`→` next and `←` back when no control has focus.
- **Add a page:** add a `WelcomePage` variant (in order) and list it in
  `WelcomePage::ALL`; add `welcome.*` strings to both locale files; add a
  match arm in `views/welcome.rs` built from components only; extend the
  kittest tour test.
- **Another multi-step flow** (import, setup): reuse `WizardWindow` with its
  own page enum and actions; do not copy the welcome view.

## Pages

One module per page in `crates/app/src/views/`, each an `impl DesktopApp`
block with a `*_view(&mut self, ui)` method:

1. Load data first (`self.records(...)`), handling `Err` with a state from
   `docs/STATES.md`.
2. Draw `PageHeader` (title, subtitle, at most one primary action).
3. Compose `card`/`section` blocks from components.
4. Collect at most one `Action` in a local `action` variable and dispatch it
   after drawing.

## Components

| Component | Use |
| --- | --- |
| `Button` (`Primary`/`Secondary`/`Ghost`/`Danger`, `icon` or `trailing_icon`) | every text command; `trailing_icon(ArrowRight)` for forward steps |
| `IconButton` | icon-only commands; label is the tooltip and accessible name |
| `NavItem` | sidebar destinations (expanded/collapsed) |
| `card`, `section` | the only surfaces on the canvas; never nested |
| `inset`, `callout` | boxes inside dialogs and tours; `callout` (accent outline) at most once per page |
| `PageHeader` | the top of every page |
| `StatCard` | one metric |
| `ListRow` | selectable rows with subtitle and badge |
| `Badge` + `Tone` | short status; neutral unless the state has meaning |
| `Segmented` | 2–4 mutually exclusive options |
| `Switch` | on/off settings |
| `Stepper` + `StepperFormat` | − value + for small numeric settings (interface size) |
| `FeatureRow` | icon tile + title + body, for tours and intros |
| `WizardWindow` + `WizardText` → `WizardEvent` | floating, draggable multi-step window (welcome tour) |
| `text_field`, `password_field`, `text_area`, `search_field` | labelled inputs |
| `Kbd`, `shortcut_row` | shortcut chips; a keys/description line in shortcut lists |
| `EmptyState`, `StatusPage`, `Notice`, `ConfirmDialog` | states, see `docs/STATES.md` |

All components are presentational: they take prepared values and return a
`Response` (or a small enum). They never lock the database, call the API, or
dispatch.

## Recipes

### Add a page

1. Add a variant to `Pane` in `lib.rs.tmpl` (title key and `Icon` in
   `title_key`/`icon`, position in `Pane::all`).
2. Create `views/<page>.rs` with `pub(crate) fn <page>_view(&mut self, ui)`,
   register it in `views/mod.rs`, and route it in `PaneBehavior::pane_ui`.
3. Add a `Command::Navigate` entry in `commands.rs` with a `command.*` label.
4. Add every string to `locales/en.yml` and `locales/es.yml`.
5. Add a kittest that navigates to the page by its sidebar label.

### Add a component

1. Check this table and `DESIGN.md` first; extend an existing component before
   adding a new one.
2. Create `components/<name>.rs`: a builder struct with `#[must_use]`,
   `const fn` setters, and either `impl Widget` (single control) or
   `fn show(self, ui) -> …` (composed).
3. Colours from `theme::palette`, sizes from `theme::{space, radius, size}`,
   icons from `icons`. No raw colours or magic numbers; name constants.
4. Interactive: `Sense::click()`, `response.widget_info(...)` with a role and
   label, `focus_ring` for keyboard focus, `mark_changed()` when it edits a value.
5. Re-export it from `components/mod.rs`, add it to the table above, and test
   its behaviour (kittest) or its maths (unit test).

### Add an icon

Add a variant to `Icon`, add it to `Icon::ALL`, and describe it in `shapes`
using the `Pen` helpers on the 16-unit grid (1.5 stroke, keep 1.5 units of
margin). `cargo test` checks every icon stays inside its box.

### Add a colour or token

Add the role to `Palette` with values in **both** `light()` and `dark()` (and
adjust `with_high_contrast` if it is text or a border), cover it in the
contrast tests, and document it in `DESIGN.md`. If the GPUI template has the
same role, keep the hex values identical.

## Verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

Then run the app (`make dev`) and check the change in light, dark, and high
contrast, with the sidebar collapsed, at the minimum window size (760×520),
and with Spanish selected (longer strings).
