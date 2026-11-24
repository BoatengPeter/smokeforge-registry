//! Surfaces: cards on the workspace canvas, insets and callouts in dialogs.

use egui::{CornerRadius, Frame, InnerResponse, Margin, RichText, Stroke, Ui};

use crate::theme::{self, radius, space};

const INSET_PADDING: i8 = 14;

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

/// A quiet filled box for secondary information inside a dialog or tour page
/// (a card would sit too heavy on a raised surface).
pub fn inset<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let palette = theme::palette(ui.ctx());
    Frame::new()
        .fill(palette.field)
        .stroke(Stroke::new(1.0, palette.border))
        .corner_radius(CornerRadius::same(radius::LG))
        .inner_margin(Margin::same(INSET_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            ui.spacing_mut().item_spacing.y = space::S;
            add_contents(ui)
        })
}

/// An accent-outlined box for the one thing on a page worth remembering
/// ("Ctrl K opens everything"). Use at most one per page.
pub fn callout<R>(ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
    let palette = theme::palette(ui.ctx());
    Frame::new()
        .fill(palette.selected)
        .stroke(Stroke::new(1.0, palette.accent))
        .corner_radius(CornerRadius::same(radius::LG))
        .inner_margin(Margin::same(INSET_PADDING))
        .show(ui, |ui| {
            ui.set_width(ui.available_width());
            add_contents(ui)
        })
}
