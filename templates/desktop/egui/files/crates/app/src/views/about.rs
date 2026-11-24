//! About: app identity and runtime details.

use egui::{RichText, Ui};

use super::detail_row;
use crate::components::{Button, PageHeader, section};
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{DesktopApp, system, t};

impl DesktopApp {
    pub(crate) fn about_view(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        PageHeader::new(&t("about.title"))
            .subtitle(&t("about.subtitle"))
            .show(ui, |_| {});

        section(ui, &self.app_name, |ui| {
            ui.horizontal(|ui| {
                ui.add(
                    egui::Image::new(egui::include_image!("../../../../assets/icon.png"))
                        .fit_to_exact_size(egui::vec2(48.0, 48.0))
                        .corner_radius(10),
                );
                ui.vertical(|ui| {
                    ui.label(RichText::new(&self.app_name).text_style(theme::title_style()));
                    ui.label(RichText::new(t("app.tagline")).color(palette.text_secondary));
                });
            });
            ui.add_space(space::M);
            detail_row(ui, &t("about.version"), env!("CARGO_PKG_VERSION"));
            detail_row(ui, &t("about.toolkit"), "eframe / egui 0.37");
            detail_row(
                ui,
                &t("about.platform"),
                &format!("{} · {}", std::env::consts::OS, std::env::consts::ARCH),
            );
            detail_row(ui, &t("about.data"), &self.paths.data.display().to_string());
            detail_row(ui, &t("about.logs"), &self.paths.logs.display().to_string());
            ui.add_space(space::M);
            if ui
                .add(Button::secondary(&t("state.open_logs")).icon(Icon::Terminal))
                .clicked()
                && let Err(error) = system::open_folder(&self.paths.logs)
            {
                self.toasts.error(error.to_string());
            }
        });
    }
}
