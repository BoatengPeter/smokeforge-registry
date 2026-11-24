//! An icon tile beside a short title and explanation, for tours and empty pages.

use egui::{RichText, Sense, StrokeKind, Ui, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, size, space};

const TILE: f32 = 32.0;

/// "What this app does" row: icon tile, semibold title, wrapped body.
#[must_use = "call `show` to draw the row"]
#[derive(Clone, Copy, Debug)]
pub struct FeatureRow<'a> {
    icon: Icon,
    title: &'a str,
    body: &'a str,
}

impl<'a> FeatureRow<'a> {
    pub const fn new(icon: Icon, title: &'a str, body: &'a str) -> Self {
        Self { icon, title, body }
    }

    pub fn show(self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = space::M;
            let (tile, _) = ui.allocate_exact_size(vec2(TILE, TILE), Sense::hover());
            ui.painter().rect(
                tile,
                radius::LG - 2,
                palette.field,
                egui::Stroke::new(1.0, palette.border),
                StrokeKind::Inside,
            );
            let icon_rect =
                egui::Rect::from_center_size(tile.center(), vec2(size::ICON, size::ICON));
            icons::paint(ui.painter(), icon_rect, self.icon, palette.accent_text);
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = space::XS;
                ui.label(RichText::new(self.title).font(theme::semibold(14.0)));
                ui.label(RichText::new(self.body).color(palette.text_secondary));
            });
        });
    }
}
