---
name: egui-component-engineering
description: Use when building or changing egui/eframe 0.36 or 0.37 components, panels, toolbars, dialogs, tables, forms, or other Rust desktop UI; especially when the work needs a reusable API, keyboard and screen-reader access, theme-token adherence, stable widget IDs, or testable interaction behavior.
---

# egui Component Engineering (0.36 / 0.37)

Build small, reusable Rust UI components that feel native to the product and behave correctly under keyboard, mouse, IME, resizing, and accessibility use.

## This Template's Design System

Before writing UI, read `DESIGN.md`, `docs/UI.md`, and `docs/STATES.md`. Then
reuse what exists instead of drawing new widgets:

- Tokens: `crate::theme` (palette, `space`, `radius`, `size`, text styles).
- Icons: `crate::icons::Icon` (drawn in code; add new ones there).
- Components: `crate::components` — `Button`, `IconButton`, `NavItem`, `card`,
  `section`, `PageHeader`, `StatCard`, `ListRow`, `Badge`, `Segmented`,
  `Switch`, fields, `Kbd`, `EmptyState`, `StatusPage`, `Notice`, `ConfirmDialog`.
- States: pick the pattern from `docs/STATES.md` (status page, notice, empty
  state, confirmation, undo) — do not invent ad-hoc error frames.

When you add a token, icon, component, page, or state, update the matching
section of `DESIGN.md`, `docs/UI.md`, or `docs/STATES.md` in the same change.

## Before Writing Code

1. Read `DESIGN.md`, `docs/UI.md`, and `crates/app/src/theme.rs`.
2. Check the pinned egui/eframe version in `Cargo.lock`, then read
   [REFERENCE.md](REFERENCE.md) for the version table, immediate-mode, repaint,
   ID, custom-widget, and `egui_kittest` rules. This skill supports 0.36 and
   0.37; use the API names for the version in the project and verify unfamiliar
   APIs in the installed source. Do not copy a 0.37 example into 0.36 blindly,
   or a 0.36 example into 0.37 blindly.
3. Read the parent view and the `Action` enum. Identify which state is owned by `DesktopApp`, which is local UI state, and which is domain data.
3. Name the component's job in one sentence. If it needs "and", split it.
4. List every state it must show: loading, empty, error, disabled, selected, conflict, and success.
5. Decide whether it is a **control** (one `Widget`), a **composed component** (struct + `show`), or a **feature view**. Do not make a reusable component know about the database, HTTP, or keyring.

## Component Shape

Use a composed component when it has layout, state, or multiple widgets. Keep its public surface small and explicit.

```rust
pub struct LibraryFilterBar<'a> {
    pub search: &'a mut String,
    pub status: StatusFilter,
    pub sort: Sort,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterAction {
    SearchChanged,
    StatusChanged(StatusFilter),
    SortChanged(Sort),
    ClearSearch,
}

impl LibraryFilterBar<'_> {
    pub fn show(self, ui: &mut egui::Ui) -> Option<FilterAction> {
        let mut action = None;

        ui.horizontal(|ui| {
            let label = ui.label("Search");
            ui.add(
                egui::TextEdit::singleline(self.search)
                    .hint_text("Search records")
                    .labelled_by(label.id),
            );
        });

        action
    }
}
```

Rules:

- Presentational components **emit actions**; they never dispatch business mutations or lock the database.
- The parent view fetches data, then renders the component, then dispatches the returned action.
- Use `&mut` for focused local state and owned/copy values for configuration.
- Use `impl Widget` only for a single, self-contained control. Use a named struct + `show` for composed UI.
- Return `Response`, `Option<Action>`, or a small response struct. Never return a bare `bool` for several outcomes.
- Keep files focused. A component crossing roughly 200 lines should be split or simplified.
- A custom interactive widget must use `Sense`, call `mark_changed()` when it edits a value, and provide a `WidgetInfo` accessible name.
- Colocate feature components under `crates/app/src/features/<feature>/components/`; shared primitives under `crates/app/src/components/`.

## Immediate-Mode Rules

Read [REFERENCE.md](REFERENCE.md) for the full framework contract. The minimum is:

- The app struct owns state; never pass a temporary value to a stateful widget.
- Keep the frame cheap. No blocking I/O, locks, or long computation in `ui`.
- Background work requests `ctx.request_repaint()` when it finishes.
- Give every persistent widget a stable ID derived from domain identity, not a loop index. Use `ui.push_id(("library-row", item.id), ...)`.
- Use `ui.push_id(stable_domain_key, ...)` for list items. For a standalone
  stable ID, use `egui::Id::unique(...)` on 0.37 and `egui::Id::new(...)` on
  0.36. Never invent a new ID every frame from a timestamp or random value.
- Do not hold a lock, HTTP request, or long computation across a `Ui` closure. Prepare data first; dispatch work after the frame.
- Do not request focus unconditionally. Check focus or an explicit shortcut first.
- Use `ui.horizontal_wrapped` for filter chips; use `ui.horizontal` when wrapping would break layout semantics.
- Use `ui.scope` or `egui::Frame` for surfaces. Keep spacing on the 4/8/12/16/24 scale. On 0.36 use `Frame::NONE` when you intentionally want no frame; on 0.37 use `Frame::new()` when you need frame styling.
- Prefer semantic widgets (`Button`, `selectable_value`, `ComboBox`, `TextEdit`) over painted rectangles with `Sense`.
- Use `ui.visuals()` for semantic surfaces, text, and strokes. Read palette values only from `theme.rs`.

## Keyboard and Accessibility

Every interactive component must be usable without a mouse.

- Associate text fields with visible labels using `.labelled_by(label.id)`.
- Put `.accessible_name(...)` on icon-only buttons and controls with no visible label.
- Make selection state visible in the widget and in the accessibility tree; use selectable controls rather than color-only changes.
- Support Enter/Space for activation and Escape for dismiss/clear only when the component has focus.
- Provide a focus target after a destructive or navigation action. Do not steal focus every frame.
- Do not convey status by color alone; pair it with text, an icon, or a shape.
- Verify focus rings remain visible against panel and raised surfaces.
- For dialogs and menus, restore focus to the opener when closed.

## Visual Quality

- Follow the tokens in `theme.rs` (`theme::palette(ctx)`, `space`, `radius`, `size`); never add a second palette or raw component colors. `DESIGN.md` documents every token.
- Use density appropriate to desktop: roughly 28–36 px control heights, 12–16 px card padding, and one clear primary action per group.
- Mark the primary action with `components::Button::primary`; use `secondary`, `ghost`, or `danger` for everything else. One primary per area.
- Avoid generic dashboard cards, oversized heading blocks, gradients, and decorative shadows.
- Use hierarchy: page title, section label, body, helper text. Configure three or four text sizes through `TextStyle`, not per-widget `.size()`.
- Show real empty, loading, error, and permission states. Empty states should name the next useful action.
- Use semantic colours from `Visuals` (`error_fg_color`, `warn_fg_color`, `hyperlink_color`) and brand tokens from `theme.rs`.
- Disable an unavailable control with a reason instead of hiding it; confirm destructive actions in a modal or make them undoable.
- Buttons name their action; errors say what happened and what to do next.
- Long text must wrap or truncate with a tooltip; tables need stable columns and a focusable row action.
- Do not use a spinner for content that can use a skeleton/placeholder.
- `ui.available_width()` is the space left at the cursor, not the window width. `ui.horizontal` does not right-align; use `egui::Sides` or a right-to-left layout.
- Allow translated strings to grow 30–40%; do not give text widgets fixed sizes.

## Testing

- Unit-test pure helpers: labels, filtering, sorting, summaries, validation mapping.
- Use exact-pinned `egui_kittest` matching the project's egui version for interaction and AccessKit checks.
- Test at least one keyboard path and one screen-reader-name path for interactive components.
- Advance a frame after click/key input; use `step`/`run_steps` when the UI continuously repaints.
- Make every absence check prove the section is visible first.
- Keep pixel snapshots Linux-only; assert semantics on every OS.
- Run the project gate before claiming completion:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
```

## Related Rust Skills

Use `rust-async-patterns` when the component starts background/Tokio work,
`rust-testing` when adding tests or fixtures, `rust-patterns` when changing
state/error APIs, and the two Rust best-practice skills for review. The component
skill owns UI behavior; the Rust skills own language and concurrency idioms.

## Common Mistakes

| Mistake | Fix |
| --- | --- |
| Component reads `Database` or calls the API | Take prepared values; emit an action |
| Icon-only button has no name | Add `accessible_name` and a hover label |
| Every row uses `ui.push_id(i, ...)` | Use the item's stable ID |
| Search field has no label | Use a visible label and `labelled_by` |
| Clear button steals focus every frame | Focus only after an explicit action |
| Domain status mapped to an invented UI status | Use the core enum or an explicit mapping with tests |
| One 300-line `show` function | Split rows, toolbar, empty state, and actions |
| Colors copied from a screenshot | Add a token to `theme.rs`, then use it |
| Database/API call inside a widget closure | Load before rendering; apply after the frame |
| Count label uses pre-filter results | Compute after filters, or label it as a total |
| Ignoring disabled/loading/error states | Render them explicitly and test them |

## Red Flags

- Raw `Color32::from_rgb` outside `theme.rs`
- `Mutex`, `reqwest`, or `rusqlite` inside a component
- Widget labels that only make sense visually
- No focus path for a keyboard user
- Duplicate IDs or IDs derived from positions
- A component with no tests and no reusable API
- A business mutation triggered directly by a click without an `Action`
