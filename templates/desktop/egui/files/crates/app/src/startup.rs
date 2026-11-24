//! The screen shown when the app cannot start: settings, paths, migrations, or
//! the database failed before the main window could be built.

use std::path::PathBuf;

use egui::{CentralPanel, Frame, Ui};

use crate::components::{StatusAction, StatusPage, Tone};
use crate::icons::Icon;
use crate::{system, t, theme};

/// Where the app writes logs, if the platform directories resolve at all.
#[must_use]
pub fn logs_folder(app_name: &str) -> Option<PathBuf> {
    crate::storage::Paths::discover(app_name)
        .ok()
        .map(|paths| paths.logs)
}

/// What the user chose on the failure screen.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StartupAction {
    /// Try to start the app again (for example after freeing disk space).
    Retry,
    /// Close the window.
    Quit,
}

/// A startup error with enough context to report it.
#[derive(Debug, Clone)]
pub struct StartupFailure {
    error: String,
    logs: Option<PathBuf>,
    theme_installed: bool,
}

impl StartupFailure {
    /// `error` should be the full chain (`format!("{error:#}")`).
    #[must_use]
    pub fn new(error: String, logs: Option<PathBuf>) -> Self {
        Self {
            error,
            logs,
            theme_installed: false,
        }
    }

    /// The error text shown under "Details".
    #[must_use]
    pub fn error(&self) -> &str {
        &self.error
    }

    /// Draws the failure screen in the whole window.
    pub fn show(&mut self, ui: &mut Ui) -> Option<StartupAction> {
        if !self.theme_installed {
            theme::apply_now(ui, theme::ThemeSettings::default());
            self.theme_installed = true;
        }
        let palette = theme::palette(ui.ctx());
        let mut action = None;
        CentralPanel::no_frame()
            .frame(Frame::new().fill(palette.canvas))
            .show(ui, |ui| {
                let page = StatusPage::new(
                    Icon::Close,
                    Tone::Danger,
                    &t("startup.title"),
                    &t("startup.body"),
                )
                .details(&self.error)
                .primary(&t("startup.retry"))
                .secondary(&t("startup.open_logs"))
                .show(ui);
                match page {
                    Some(StatusAction::Primary) => action = Some(StartupAction::Retry),
                    Some(StatusAction::Secondary) => {
                        if let Some(logs) = &self.logs {
                            let _ = system::open_folder(logs);
                        }
                    }
                    None => {}
                }
                ui.vertical_centered(|ui| {
                    if ui.link(t("startup.quit")).clicked() {
                        action = Some(StartupAction::Quit);
                    }
                });
            });
        action
    }
}

#[cfg(test)]
mod tests {
    use egui_kittest::Harness;
    use egui_kittest::kittest::Queryable;

    use super::*;

    #[test]
    fn failure_screen_names_the_problem_and_offers_retry() {
        let failure = StartupFailure::new("migration V3 failed: disk is full".to_owned(), None);
        let mut harness = Harness::builder()
            .with_size(egui::vec2(900.0, 640.0))
            .build_ui_state(
                |ui, state: &mut (StartupFailure, Option<StartupAction>)| {
                    if let Some(action) = state.0.show(ui) {
                        state.1 = Some(action);
                    }
                },
                (failure, None),
            );
        harness.run();
        harness.get_by_label("migration V3 failed: disk is full");
        harness.get_by_label("Try again").click();
        harness.run();
        assert_eq!(harness.state().1, Some(StartupAction::Retry));
    }
}
