//! Command registry for the keyboard-first shell.

use crate::Pane;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Command {
    Navigate(Pane),
    NewRecord,
    ToggleSidebar,
    ToggleTheme,
    ToggleHighContrast,
}

impl Command {
    pub const ALL: [Self; 8] = [
        Self::Navigate(Pane::Dashboard),
        Self::Navigate(Pane::Library),
        Self::Navigate(Pane::Settings),
        Self::Navigate(Pane::About),
        Self::NewRecord,
        Self::ToggleSidebar,
        Self::ToggleTheme,
        Self::ToggleHighContrast,
    ];

    #[must_use]
    pub const fn label_key(self) -> &'static str {
        match self {
            Self::Navigate(Pane::Dashboard) => "command.dashboard",
            Self::Navigate(Pane::Library) => "command.library",
            Self::Navigate(Pane::Settings) => "command.settings",
            Self::Navigate(Pane::About) => "command.about",
            Self::NewRecord => "command.new_record",
            Self::ToggleSidebar => "command.toggle_sidebar",
            Self::ToggleTheme => "command.toggle_theme",
            Self::ToggleHighContrast => "command.toggle_contrast",
        }
    }
}

#[must_use]
pub fn filter_commands(query: &str, label: impl Fn(Command) -> String) -> Vec<Command> {
    let query = query.trim().to_lowercase();
    Command::ALL
        .into_iter()
        .filter(|command| query.is_empty() || label(*command).to_lowercase().contains(&query))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_query_keeps_every_command() {
        assert_eq!(
            filter_commands("", |_| String::new()).len(),
            Command::ALL.len()
        );
    }

    #[test]
    fn filter_is_case_insensitive() {
        let commands = filter_commands("THEME", |command| command.label_key().to_owned());
        assert_eq!(commands, vec![Command::ToggleTheme]);
    }
}
