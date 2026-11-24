//! Page title, supporting line, and right-aligned page actions.

use egui::{RichText, Ui};

use crate::theme::{self, space};

/// The heading block at the top of every page.
#[must_use = "call `show` to draw the header"]
#[derive(Clone, Debug)]
pub struct PageHeader<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
}

impl<'a> PageHeader<'a> {
    pub const fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
        }
    }

    pub const fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    /// Draws the header; `actions` fills the right side (pass `|_| {}` for none).
    pub fn show(self, ui: &mut Ui, actions: impl FnOnce(&mut Ui)) {
        let palette = theme::palette(ui.ctx());
        egui::Sides::new().show(
            ui,
            |ui| {
                ui.vertical(|ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    ui.label(RichText::new(self.title).text_style(theme::title_style()));
                    if let Some(subtitle) = self.subtitle {
                        ui.label(RichText::new(subtitle).color(palette.text_secondary));
                    }
                });
            },
            actions,
        );
        ui.add_space(space::L);
    }
}
