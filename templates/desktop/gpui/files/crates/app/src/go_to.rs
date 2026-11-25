//! "Go to…": the pages and actions the search window lists.
//!
//! `shell.rs` turns these into GPUI Kit `CommandGroup`s inside the ui crate's
//! `PickerFrame`. Each row carries the GPUI action it dispatches, so choosing
//! a row is the same as pressing its shortcut. To add an action: add a row to
//! [`ACTIONS`] with a `go_to.*` string; give it a keybinding in `keymap.rs`
//! if it deserves one.

use gpui_kit::Action;
use gpui_kit::component::IconName;

use crate::{
    NewRecord, Pane, ShowAbout, ShowAccount, ShowActivity, ShowDashboard, ShowLibrary,
    ShowSettings, ShowWelcome, ToggleSidebar, ToggleTheme, ZoomIn, ZoomOut,
};

/// The action that opens `pane`.
#[must_use]
pub fn page_action(pane: Pane) -> Box<dyn Action> {
    match pane {
        Pane::Dashboard => Box::new(ShowDashboard),
        Pane::Library => Box::new(ShowLibrary),
        Pane::Activity => Box::new(ShowActivity),
        Pane::Settings => Box::new(ShowSettings),
        Pane::Account => Box::new(ShowAccount),
        Pane::About => Box::new(ShowAbout),
    }
}

/// A "+" row: label key, icon, and the action it dispatches.
pub struct GoToAction {
    pub label_key: &'static str,
    pub icon: IconName,
    pub action: fn() -> Box<dyn Action>,
}

pub const ACTIONS: [GoToAction; 6] = [
    GoToAction {
        label_key: "go_to.new_record",
        icon: IconName::Plus,
        action: || Box::new(NewRecord),
    },
    GoToAction {
        label_key: "go_to.toggle_theme",
        icon: IconName::Moon,
        action: || Box::new(ToggleTheme),
    },
    GoToAction {
        label_key: "go_to.toggle_sidebar",
        icon: IconName::PanelLeft,
        action: || Box::new(ToggleSidebar),
    },
    GoToAction {
        label_key: "go_to.zoom_in",
        icon: IconName::Plus,
        action: || Box::new(ZoomIn),
    },
    GoToAction {
        label_key: "go_to.zoom_out",
        icon: IconName::Minus,
        action: || Box::new(ZoomOut),
    },
    GoToAction {
        label_key: "go_to.welcome",
        icon: IconName::Info,
        action: || Box::new(ShowWelcome),
    },
];

#[cfg(test)]
mod tests {
    use super::ACTIONS;

    #[test]
    fn every_action_has_an_english_label() {
        rust_i18n::set_locale("en");
        for action in &ACTIONS {
            let label = rust_i18n::t!(action.label_key);
            assert_ne!(
                label, action.label_key,
                "missing string for {}",
                action.label_key
            );
        }
    }
}
