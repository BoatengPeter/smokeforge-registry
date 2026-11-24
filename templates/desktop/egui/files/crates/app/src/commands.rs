//! Command registry for the keyboard-first shell. The "Go to…" window
//! (`crate::go_to`) lists every non-navigation command as an action.

use crate::Pane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Navigate(Pane),
    NewRecord,
    ToggleSidebar,
    ToggleTheme,
    ToggleHighContrast,
    ShowWelcome,
}

impl Command {
    pub const ALL: [Self; 11] = [
        Self::Navigate(Pane::Dashboard),
        Self::Navigate(Pane::Library),
        Self::Navigate(Pane::Activity),
        Self::Navigate(Pane::Settings),
        Self::Navigate(Pane::Account),
        Self::Navigate(Pane::About),
        Self::NewRecord,
        Self::ToggleSidebar,
        Self::ToggleTheme,
        Self::ToggleHighContrast,
        Self::ShowWelcome,
    ];

    #[must_use]
    pub const fn label_key(self) -> &'static str {
        match self {
            Self::Navigate(Pane::Dashboard) => "command.dashboard",
            Self::Navigate(Pane::Library) => "command.library",
            Self::Navigate(Pane::Activity) => "command.activity",
            Self::Navigate(Pane::Settings) => "command.settings",
            Self::Navigate(Pane::Account) => "command.account",
            Self::Navigate(Pane::About) => "command.about",
            Self::NewRecord => "command.new_record",
            Self::ToggleSidebar => "command.toggle_sidebar",
            Self::ToggleTheme => "command.toggle_theme",
            Self::ToggleHighContrast => "command.toggle_contrast",
            Self::ShowWelcome => "command.welcome",
        }
    }
}
