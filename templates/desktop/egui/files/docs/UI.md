# UI

The shell has a fixed 38px title bar, collapsible 208px/48px sidebar, tiled
content workspace, and 28px status bar. The content tree is `egui_tiles`; the
Dashboard, Library, Settings, and About panes are tabbed and rearrangeable.

Navigation dispatches `Action::Navigate` and makes the matching tile active.
The Library pane uses a local SQLite editor. The title bar is draggable and
window controls are native on macOS and custom on Windows/Linux.

## Components

Read `.agents/skills/egui-component-engineering/SKILL.md` before adding or changing UI. Components emit actions, use `theme.rs` tokens, keep stable IDs, expose accessible names, and are verified with pure tests plus `egui_kittest` interaction checks where applicable.
