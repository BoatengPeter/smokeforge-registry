//! Card surfaces for grouping content on the workspace canvas.

use egui::{CornerRadius, Frame, InnerResponse, Margin, RichText, Stroke, Ui};

use crate::theme::{self, radius, space};

/// Draws `add_contents` on a card: surface fill, hairline border, large radius.
pub fn card<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let palette = theme::palette(ui.ctx());
    Frame::new()
        .fill(palette.surface)
        .stroke(Stroke::new(1.0, palette.border))
        .corner_radius(CornerRadius::same(radius::LG))
        .inner_margin(Margin::same(16))
        .show(ui, add_contents)
}

/// A card with a section heading above its contents.
pub fn section<R>(
    ui: &mut Ui,
    title: &str,
    add_contents: impl FnOnce(&mut Ui) -> R,
) -> InnerResponse<R> {
    card(ui, |ui| {
        ui.set_width(ui.available_width());
        ui.label(RichText::new(title).heading());
        ui.add_space(space::S);
        add_contents(ui)
    })
}
