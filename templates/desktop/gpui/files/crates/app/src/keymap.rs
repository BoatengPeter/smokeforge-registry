//! The app's keyboard shortcuts, defined once.
//!
//! `secondary` is Ctrl on Windows and Linux and Cmd on macOS. Only
//! `KeyBinding::load` resolves it (it needs the host OS); `KeyBinding::new`
//! panics on it. Anything that shows a shortcut (title bar, welcome tour)
//! reads it back with `Kbd::global_binding_for_action`, so labels follow this
//! table.

use gpui_kit::{Action, App, KeyBinding};

use crate::{
    NewRecord, ShowAbout, ShowAccount, ShowActivity, ShowDashboard, ShowLibrary, ShowSettings,
    ToggleCommand, ToggleSidebar, ToggleTheme, ZoomIn, ZoomOut, ZoomReset,
};

/// Every global keybinding, resolved for the host OS.
///
/// # Panics
///
/// Panics if an entry in the table is not a valid keystroke; the
/// `every_binding_loads` test catches that before release.
#[must_use]
pub fn bindings(cx: &App) -> Vec<KeyBinding> {
    table()
        .into_iter()
        .map(|(keys, action)| {
            KeyBinding::load(keys, action, None, false, None, cx)
                .unwrap_or_else(|error| panic!("invalid built-in keybinding {keys}: {error}"))
        })
        .collect()
}

fn table() -> Vec<(&'static str, Box<dyn Action>)> {
    vec![
        ("secondary-1", Box::new(ShowDashboard)),
        ("secondary-2", Box::new(ShowLibrary)),
        ("secondary-3", Box::new(ShowActivity)),
        ("secondary-4", Box::new(ShowSettings)),
        ("secondary-5", Box::new(ShowAccount)),
        ("secondary-6", Box::new(ShowAbout)),
        ("secondary-k", Box::new(ToggleCommand)),
        ("secondary-b", Box::new(ToggleSidebar)),
        ("secondary-n", Box::new(NewRecord)),
        ("secondary-shift-l", Box::new(ToggleTheme)),
        ("secondary-=", Box::new(ZoomIn)),
        ("secondary-+", Box::new(ZoomIn)),
        ("secondary--", Box::new(ZoomOut)),
        ("secondary-0", Box::new(ZoomReset)),
    ]
}

#[cfg(test)]
mod tests {
    use super::{bindings, table};
    use gpui_kit::TestAppContext;

    #[gpui_kit::test]
    fn every_binding_loads(cx: &mut TestAppContext) {
        let loaded = cx.update(|cx| bindings(cx));
        assert_eq!(loaded.len(), table().len());
    }
}
