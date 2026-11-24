//! A metric card: label, large value, optional caption.

use egui::{RichText, Ui};

use super::card;
use crate::icons::{self, Icon};
use crate::theme::{self, size};

const MIN_WIDTH: f32 = 170.0;

/// One number on the dashboard.
#[must_use = "call `show` to draw the card"]
#[derive(Clone, Debug)]
pub struct StatCard<'a> {
    label: &'a str,
    value: &'a str,
    icon: Icon,
    caption: Option<&'a str>,
}

impl<'a> StatCard<'a> {
    pub const fn new(icon: Icon, label: &'a str, value: &'a str) -> Self {
        Self {
            label,
            value,
            icon,
            caption: None,
        }
    }

    pub const fn caption(mut self, caption: &'a str) -> Self {
        self.caption = Some(caption);
        self
    }

    pub fn show(self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        card(ui, |ui| {
            ui.set_min_width(MIN_WIDTH.min(ui.available_width()));
            ui.set_width(ui.available_width());
            // A card inherits its parent's layout; force a column so the value
            // sits under the label even inside a horizontal row.
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    let (rect, _) = ui.allocate_exact_size(
                        egui::vec2(size::ICON, size::ICON),
                        egui::Sense::hover(),
                    );
                    icons::paint(ui.painter(), rect, self.icon, palette.text_tertiary);
                    ui.label(
                        RichText::new(self.label)
                            .small()
                            .color(palette.text_secondary),
                    );
                });
                ui.label(RichText::new(self.value).text_style(theme::display_style()));
                if let Some(caption) = self.caption {
                    ui.label(RichText::new(caption).small().color(palette.text_tertiary));
                }
            });
        });
    }
}
