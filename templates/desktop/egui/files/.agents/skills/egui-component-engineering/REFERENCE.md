# egui 0.36 / 0.37 Framework Reference

Adapted for this skill from `gregorycarnegie/egui-agent-skills` (the `egui`,
`egui-ui-design`, and `egui-kittest` skills), reviewed at commit
`82b329557a2921dd9c9455de1a87e76aea75abed`. The upstream material targets egui
0.36; this template pins 0.37. The skill supports both versions, so always
select names from the version in `Cargo.lock` and verify unfamiliar APIs against
the installed source.

## Version Gate

1. Read `egui`/`eframe` in `Cargo.lock`. The installed source is under
   `~/.cargo/registry/src/*/egui-<version>/src/`.
2. Use this table, then verify names in the matching source:

   | Concern | egui 0.36 | egui 0.37 |
   | --- | --- | --- |
   | App entry | `fn ui(&mut self, &mut Ui, &mut Frame)` | `fn ui(...)` |
   | Standalone stable ID | `Id::new(...)` | `Id::unique(...)` (`Id::new` deprecated) |
   | Empty frame | `Frame::NONE` (old `Frame::none()` is gone) | `Frame::new()` (or `Frame::NONE`) |
   | Corner radius | `CornerRadius` | `CornerRadius` |
   | Panel API | `Panel::...`, `.show(ui, ...)` | same |
   | `egui_kittest` | exact `0.36.x` | exact `0.37.x` |

3. The 0.37 template uses `egui_kittest = "=0.37.0"`. A 0.36 project must use
   its matching `egui_kittest`; never mix minor versions.
4. When unsure whether a method exists, search the installed version's source.
   Do not guess, and do not apply the wrong version's example.

## Immediate-Mode Model

- The app struct owns state; widgets borrow it. Never pass a temporary
  `&mut String` or local `bool` to a stateful widget unless it is intentionally
  reset every frame.
- UI code runs many times per second. It must be cheap and must never block.
- egui repaints on input or request. Background work calls
  `ctx.request_repaint()` when its result is ready.
- Layout is single-pass. Use `ui.with_layout`, `egui::Sides`, `Grid`, or
  `egui_extras::StripBuilder`/`TableBuilder` rather than assuming the size of
  widgets that come later.

## Frame Loop

- Do not perform file, network, database, `thread::sleep`, `recv`, `join`, or
  `block_on` work in `ui`.
- Start work on a worker and keep its receiver/handle in the app state. Poll with
  `try_recv()` or `is_finished()` each frame.
- An eframe frame loop is not async. If using Tokio, create the runtime before
  `eframe::run_native` and keep it alive; `rt.enter()` lets `tokio::spawn` find
  the runtime. Never `.await` in `ui`.
- Request continuous repaints only while animation, playback, or polling is
  active. Use `request_repaint_after` for slow updates.
- Cache sorting, filtering, parsing, and text layout. Recompute when input
  changes, not every frame.
- Load textures once and keep the `TextureHandle`; call `set()` only when the
  image changes.
- Use `ScrollArea::show_rows` or `TableBuilder` body rows for long lists.

## Setup Once

- Fonts, image loaders, `Style`/`Visuals`, and persisted app state belong in the
  app/setup constructor, not in `ui`.
- `egui_extras::install_image_loaders(&cc.egui_ctx)` once.
- Add fonts with `ctx.set_fonts` once; rebuilding the font atlas every frame is
  expensive.
- This app uses JSON settings plus eframe layout persistence. Do not move domain
  data, tokens, or SQLite records into eframe storage.
- On 0.36 and 0.37, `App::ui` is the current entry point; `App::update` is the
  old pattern. Check the version table before changing the shell.

## IDs

- Stateful widgets include `CollapsingHeader`, `ScrollArea`, `TextEdit`,
  `ComboBox`, `Grid`, `Window`, and popups.
- Key loops by domain ID, not index: `ui.push_id(("row", item.id), ...)`.
- For a standalone stable ID, use `Id::unique` on 0.37 and `Id::new` on 0.36.
- Conditional widgets shift automatic IDs. Give later stateful widgets an
  explicit `.id_salt(...)` when a branch above them can appear or disappear.
- Window titles are IDs. A changing title loses window position/size; set an
  explicit stable `.id(...)`.
- A red rectangle with an ID message in a debug build is an ID clash. Fix the
  IDs; it is not a rendering bug.

## Root and Panels

- Add top-level panels from the outside in, then the central content, then
  floating windows. Never add a top-level panel after the central panel.
- Do not nest top-level panels inside another panel closure.
- The root `Ui` in `App::ui` has no margin/background in newer eframe. Wrap
  content in `CentralPanel::default().show(ui, ...)` unless the app shell
  intentionally paints its own root surface.
- Use `Panel`/`CentralPanel` with `.show(ui, ...)` for 0.36+ APIs; verify
  against 0.37 source before changing the shell.

## Responses and Custom Widgets

- Use `.changed()` for edited values, not `.clicked()`.
- A custom widget that edits a value must call `response.mark_changed()`.
- Custom interactive widgets reserve space with
  `ui.allocate_response(size, egui::Sense::click())` or `Sense::drag()`, then
  paint with `ui.painter()` and `ui.style().interact(&response)`.
- A `Response` describes one frame only. Do not store it in app state.
- Give custom interactive widgets an accessible name with
  `response.widget_info(|| egui::WidgetInfo::labeled(egui::WidgetType::Button, true, "Play"))`.

## Accessibility

- AccessKit is enabled by default with eframe; text-bearing widgets already have
  names, but icon-only controls do not.
- Prefer visible text. Use `.accessible_name(...)` plus hover text when an icon
  must stand alone.
- Use `.labelled_by(label.id)` for text fields.
- Ensure focus is visible in both themes and that dialogs close with Escape.
- Test at 150% zoom; no text may clip or overlap.

## UI Design Rules

- One primary action per group; use `ui.visuals().selection` for its emphasis.
- Use semantic colors from `Visuals` (`error_fg_color`, `warn_fg_color`,
  `hyperlink_color`) and brand tokens from `theme.rs`; never raw state colors.
- Use three or four text sizes per screen, configured through `TextStyle`, not
  `.size()` on every label.
- Prefer disabling a control with a reason over hiding it:
  `ui.add_enabled(can_save, Button::new("Save")).on_disabled_hover_text("Nothing to save")`.
- Confirm destructive actions in a modal or make them undoable.
- Buttons name their action. Errors say what happened and what to do next.
  Empty states offer the next action.
- Frames and separators mark real groups only; do not box every section.
- Measure with `ui.available_width()` (space left at the cursor), not the window
  width.
- `ui.horizontal` does not right-align. Use `egui::Sides` or a right-to-left
  layout.
- Test strings that may grow 30–40% in translation.

## egui_kittest

- Keep `egui`, `eframe`, and `egui_kittest` on the same exact version. A mismatch
  produces duplicate egui types. Use `0.36.x` together for a 0.36 app and
  `0.37.x` together for this template.
- Harness choices:
  - `Harness::new_ui(|ui| ...)` for stateless widgets/UI functions.
  - `Harness::new_ui_state(|ui, state| ..., state)` for widgets that read/write
    state.
  - `Harness::builder().with_size(...).build_eframe(|cc| App::new(cc))` for a
    whole app. Use a test-only constructor that avoids network/files.
- Set an explicit harness size when layout depends on `available_width()`.
- Queries come from `kittest::Queryable`: `get_by_label("Save")`,
  `get_by_label_contains(...)`, `query_by_label(...)`, and `get_all_by_*`.
- After a click or key event, advance at least one frame with `run()` before
  checking state. Use `run_steps(n)` for continuous repaint, `step()` for exactly
  one frame, and `try_run()` when a repaint loop should be an error.
- Advancing one action is not enough for a hover/press/release sequence; give
  hover, drag, and drop a frame each.
- Absence checks must first prove the section is on screen; a collapsed section
  also has no matching label.
- Snapshots need `snapshot` and `wgpu` features and a graphics adapter. Keep
  image snapshots Linux-only; use semantic/accessibility assertions on all OS.
- Use `UPDATE_SNAPSHOTS=true` only after inspecting the diff and confirming the
  visual change is intended.

## Pre-Output Checklist

- [ ] Stateful widgets in loops use stable domain IDs.
- [ ] No blocking work, lock, or network call occurs in `ui`.
- [ ] Background work requests a repaint.
- [ ] Fonts, image loaders, and theme setup happen once.
- [ ] Icon-only controls have accessible names.
- [ ] Custom interactive widgets use `Sense` and `mark_changed`.
- [ ] The rendered layout was checked in dark and light themes, or the
      limitation was reported.
