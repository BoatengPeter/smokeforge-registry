//! Activity: background work and sync status.

use egui::{RichText, Ui};

use super::setting_row;
use crate::components::{Badge, EmptyState, Notice, PageHeader, Tone, section};
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{DesktopApp, t};

impl DesktopApp {
    pub(crate) fn activity_view(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let conflicts = self.database.lock().ok().map_or(0, |database| {
            database.conflicts().map_or(0, |rows| rows.len())
        });

        PageHeader::new(&t("activity.title"))
            .subtitle(&t("activity.subtitle"))
            .show(ui, |_| {});

        if let Some(error) = &self.sync_error {
            let _ = Notice::new(Tone::Warning, &t("state.sync_failed_title"))
                .body(error)
                .show(ui);
            ui.add_space(space::S);
        }
        section(ui, &t("activity.sync"), |ui| {
            setting_row(
                ui,
                &t("activity.last_sync"),
                &t("activity.last_sync_help"),
                |ui| {
                    if self.is_syncing() {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(
                                RichText::new(t("state.syncing")).color(palette.text_secondary),
                            );
                        });
                        return;
                    }
                    let status = if self.sync_status.is_empty() {
                        t("activity.never")
                    } else {
                        self.sync_status.clone()
                    };
                    ui.label(RichText::new(status).color(palette.text_secondary));
                },
            );
            setting_row(
                ui,
                &t("activity.conflicts"),
                &t("activity.conflicts_help"),
                |ui| {
                    let tone = if conflicts == 0 {
                        Tone::Success
                    } else {
                        Tone::Warning
                    };
                    ui.add(Badge::new(&conflicts.to_string(), tone));
                },
            );
        });
        ui.add_space(space::L);
        section(ui, &t("activity.jobs"), |ui| {
            EmptyState::new(
                Icon::Activity,
                &t("activity.empty"),
                &t("activity.empty_body"),
            )
            .show(ui);
        });
    }
}
