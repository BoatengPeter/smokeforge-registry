//! Keyboard shortcut chips such as "Ctrl K".

use egui::{Response, Sense, StrokeKind, TextStyle, Ui, Widget, vec2};

/// Width of the key column in [`shortcut_row`], so descriptions line up.
const KEYS_COLUMN: f32 = 140.0;

use crate::theme::{self, radius};

/// A keycap-style label for a shortcut.
#[must_use = "add the chip to a Ui with `ui.add(...)`"]
#[derive(Clone, Copy, Debug)]
pub struct Kbd<'a>(pub &'a str);

impl Widget for Kbd<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let font = TextStyle::Small.resolve(ui.style());
        let galley = ui
            .painter()
            .layout_no_wrap(self.0.to_owned(), font, palette.text_secondary);
        let size = vec2(galley.size().x + 10.0, 20.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        if ui.is_rect_visible(rect) {
            ui.painter().rect(
                rect,
                radius::SM,
                palette.field,
                egui::Stroke::new(1.0, palette.border),
                StrokeKind::Inside,
            );
            ui.painter().galley(
                rect.center() - galley.size() / 2.0,
                galley,
                palette.text_secondary,
            );
        }
        response
    }
}

/// One "keys — what they do" line in a shortcut list.
pub fn shortcut_row(ui: &mut Ui, keys: &str, description: &str) {
    ui.horizontal(|ui| {
        ui.allocate_ui_with_layout(
            vec2(KEYS_COLUMN, 22.0),
            egui::Layout::left_to_right(egui::Align::Center),
            |ui| ui.add(Kbd(keys)),
        );
        ui.label(description);
    });
}
