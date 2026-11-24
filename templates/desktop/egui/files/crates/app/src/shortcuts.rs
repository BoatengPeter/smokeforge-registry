//! App-wide keyboard shortcuts, defined once.
//!
//! `DesktopApp::handle_shortcuts` consumes them and the welcome tour displays
//! them (formatted per platform with `Context::format_shortcut`), so the two
//! cannot drift. Interface size uses egui's standard
//! `egui::gui_zoom::kb_shortcuts` (Ctrl/Cmd `+`, `−`, `0`).

use egui::{Key, KeyboardShortcut, Modifiers};

/// Open or close the command palette.
pub const PALETTE: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::K);
/// Start a new Library record.
pub const NEW_RECORD: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::N);
/// Show or hide the sidebar.
pub const SIDEBAR: KeyboardShortcut = KeyboardShortcut::new(Modifiers::COMMAND, Key::B);
