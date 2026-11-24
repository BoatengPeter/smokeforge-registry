//! Settings: appearance, sync, system, help, local data, and conflicts.

use egui::{RichText, Ui};

use super::{detail_row, setting_row};
use crate::components::{
    Button, Notice, PageHeader, Segmented, Stepper, StepperFormat, Switch, Tone, section,
};
use crate::icons::Icon;
use crate::theme::{self, ThemePreference, space, zoom};
use crate::{Action, Confirmation, DesktopApp, t};

impl DesktopApp {
    pub(crate) fn settings_view(&mut self, ui: &mut Ui) {
        PageHeader::new(&t("settings.title"))
            .subtitle(&t("settings.subtitle"))
            .show(ui, |_| {});
        let mut actions = Vec::new();
        self.appearance_section(ui, &mut actions);
        ui.add_space(space::L);
        let behaviour_changed = self.behaviour_section(ui);
        ui.add_space(space::L);
        help_section(ui, &mut actions);
        ui.add_space(space::L);
        self.data_section(ui);
        if behaviour_changed {
            let _ = self.settings_store.save(&self.settings);
        }
        // Each dispatch saves the settings it changed.
        for action in actions {
            self.dispatch(action);
        }
    }

    fn appearance_section(&mut self, ui: &mut Ui, actions: &mut Vec<Action>) {
        section(ui, &t("settings.appearance"), |ui| {
            setting_row(ui, &t("settings.theme"), &t("settings.theme_help"), |ui| {
                let mut preference = ThemePreference::from_setting(&self.settings.theme);
                let options = [
                    (ThemePreference::Light, t("settings.light")),
                    (ThemePreference::Dark, t("settings.dark")),
                    (ThemePreference::System, t("settings.system")),
                ];
                if Segmented::new("theme", &mut preference, &options)
                    .show(ui)
                    .changed()
                {
                    actions.push(Action::SetTheme(preference));
                }
            });
            setting_row(
                ui,
                &t("settings.language"),
                &t("settings.language_help"),
                |ui| {
                    let mut language = self.settings.language.clone();
                    let options = [("en", "English".to_owned()), ("es", "Español".to_owned())];
                    let mut selected = options
                        .iter()
                        .map(|(code, _)| *code)
                        .find(|code| *code == language)
                        .unwrap_or("en");
                    if Segmented::new("language", &mut selected, &options)
                        .show(ui)
                        .changed()
                    {
                        selected.clone_into(&mut language);
                        actions.push(Action::SetLanguage(language));
                    }
                },
            );
            setting_row(
                ui,
                &t("settings.high_contrast"),
                &t("settings.high_contrast_help"),
                |ui| {
                    let mut on = self.settings.high_contrast;
                    if ui
                        .add(Switch::new(&mut on, &t("settings.high_contrast")))
                        .changed()
                    {
                        actions.push(Action::ToggleHighContrast);
                    }
                },
            );
            setting_row(ui, &t("settings.size"), &t("settings.size_help"), |ui| {
                let mut zoom = self.settings.zoom;
                if Stepper::new(
                    "settings-size",
                    &mut zoom,
                    zoom::RANGE,
                    zoom::STEP,
                    &t("settings.size_smaller"),
                    &t("settings.size_larger"),
                )
                .format(StepperFormat::Percent)
                .show(ui)
                .changed()
                {
                    actions.push(Action::SetZoom(zoom));
                }
            });
        });
    }

    /// Returns whether a setting in this section changed.
    fn behaviour_section(&mut self, ui: &mut Ui) -> bool {
        let mut changed = false;
        section(ui, &t("settings.behaviour"), |ui| {
            setting_row(
                ui,
                &t("settings.sync_policy"),
                &t("settings.sync_help"),
                |ui| {
                    let options = [
                        ("manual", t("settings.manual")),
                        ("automatic", t("settings.automatic")),
                    ];
                    let mut policy = if self.settings.sync_policy == "automatic" {
                        "automatic"
                    } else {
                        "manual"
                    };
                    if Segmented::new("sync", &mut policy, &options)
                        .show(ui)
                        .changed()
                    {
                        policy.clone_into(&mut self.settings.sync_policy);
                        changed = true;
                    }
                },
            );
            setting_row(
                ui,
                &t("settings.log_level"),
                &t("settings.log_help"),
                |ui| {
                    let options = [
                        ("warn", "warn".to_owned()),
                        ("info", "info".to_owned()),
                        ("debug", "debug".to_owned()),
                    ];
                    let mut level = options
                        .iter()
                        .map(|(value, _)| *value)
                        .find(|value| *value == self.settings.log_level)
                        .unwrap_or("info");
                    if Segmented::new("log-level", &mut level, &options)
                        .show(ui)
                        .changed()
                    {
                        level.clone_into(&mut self.settings.log_level);
                        changed = true;
                    }
                },
            );
            setting_row(
                ui,
                &t("settings.launch_at_login"),
                &t("settings.launch_help"),
                |ui| {
                    let label = t("settings.launch_at_login");
                    if ui
                        .add(Switch::new(&mut self.settings.launch_at_login, &label))
                        .changed()
                    {
                        changed = true;
                    }
                },
            );
        });
        changed
    }

    #[expect(
        clippy::too_many_lines,
        reason = "data actions stay together in one card"
    )]
    fn data_section(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let conflicts = self
            .database
            .lock()
            .ok()
            .and_then(|database| database.conflicts().ok())
            .unwrap_or_default();

        let mut clear_requested = false;
        section(ui, &t("settings.data"), |ui| {
            ui.label(RichText::new(t("settings.database_help")).color(palette.text_secondary));
            ui.add_space(space::XS);
            detail_row(
                ui,
                &t("settings.database"),
                &self.paths.data.join("local.sqlite").display().to_string(),
            );
            if let Some(warning) = self.tokens.warning() {
                ui.add_space(space::S);
                let _ = Notice::new(Tone::Warning, &t("state.keyring_title"))
                    .body(warning)
                    .show(ui);
            }
            ui.add_space(space::M);
            ui.horizontal_wrapped(|ui| {
                if ui
                    .add(Button::secondary(&t("settings.export")).icon(Icon::Download))
                    .clicked()
                    && let Some(path) = rfd::FileDialog::new()
                        .set_file_name("library.json")
                        .save_file()
                {
                    match self.export_library(&path) {
                        Ok(()) => {
                            self.toasts.success("Library exported.");
                        }
                        Err(error) => {
                            self.toasts.error(error);
                        }
                    }
                }
                if ui
                    .add(Button::secondary(&t("settings.import")).icon(Icon::Upload))
                    .clicked()
                    && let Some(path) = rfd::FileDialog::new().pick_file()
                {
                    match self.import_library(&path) {
                        Ok(count) => {
                            self.toasts.success(format!("Imported {count} records."));
                        }
                        Err(error) => {
                            self.toasts.error(error);
                        }
                    }
                }
                if ui
                    .add(Button::secondary(&t("settings.backup")).icon(Icon::Database))
                    .clicked()
                    && let Some(path) = rfd::FileDialog::new()
                        .set_file_name("local-backup.sqlite")
                        .save_file()
                {
                    match self.backup_database(&path) {
                        Ok(()) => {
                            self.toasts.success("Database backed up.");
                        }
                        Err(error) => {
                            self.toasts.error(error);
                        }
                    }
                }
                if ui
                    .add(Button::danger(&t("settings.clear")).icon(Icon::Trash))
                    .clicked()
                {
                    clear_requested = true;
                }
            });

            if conflicts.is_empty() {
                return;
            }
            ui.add_space(space::L);
            ui.label(RichText::new(t("settings.conflicts")).heading());
            for (item_id, _, _) in conflicts {
                ui.push_id(("conflict", item_id.clone()), |ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.label(RichText::new(&item_id).monospace());
                        },
                        |ui| {
                            if ui
                                .add(Button::secondary(&t("settings.use_server")))
                                .clicked()
                            {
                                match self.resolve_conflict(&item_id, false) {
                                    Ok(()) => {
                                        self.toasts.success("Applied server record.");
                                    }
                                    Err(error) => {
                                        self.toasts.error(error);
                                    }
                                }
                            }
                            if ui
                                .add(Button::secondary(&t("settings.keep_local")))
                                .clicked()
                            {
                                match self.resolve_conflict(&item_id, true) {
                                    Ok(()) => {
                                        self.toasts.success("Kept local record.");
                                    }
                                    Err(error) => {
                                        self.toasts.error(error);
                                    }
                                }
                            }
                        },
                    );
                });
            }
        });
        if clear_requested {
            self.dispatch(Action::RequestConfirmation(Confirmation::ClearLocalData));
        }
    }
}

/// Help: reopen the welcome tour.
fn help_section(ui: &mut Ui, actions: &mut Vec<Action>) {
    section(ui, &t("settings.help"), |ui| {
        setting_row(ui, &t("settings.tour"), &t("settings.tour_help"), |ui| {
            if ui
                .add(Button::secondary(&t("settings.tour_open")))
                .clicked()
            {
                actions.push(Action::ShowWelcome);
            }
        });
    });
}
