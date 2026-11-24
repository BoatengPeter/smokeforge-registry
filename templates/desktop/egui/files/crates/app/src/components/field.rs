//! Labelled text fields.
//!
//! Each field shows a visible label above the input and links the two for
//! screen readers with `labelled_by`.

use egui::{Margin, Response, RichText, TextEdit, Ui, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, size, space};

const FIELD_MARGIN: Margin = Margin::symmetric(10, 6);

/// A single-line text field with a label above it.
pub fn text_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
    labelled(ui, label, |ui, label_id| {
        ui.add(input(TextEdit::singleline(value), hint))
            .labelled_by(label_id)
    })
}

/// A masked single-line field for passwords.
pub fn password_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
    labelled(ui, label, |ui, label_id| {
        ui.add(input(TextEdit::singleline(value), hint).password(true))
            .labelled_by(label_id)
    })
}

/// A multi-line field showing `rows` lines.
pub fn text_area(ui: &mut Ui, label: &str, value: &mut String, rows: usize) -> Response {
    labelled(ui, label, |ui, label_id| {
        ui.add(input(TextEdit::multiline(value), "").desired_rows(rows))
            .labelled_by(label_id)
    })
}

/// A search field with a leading magnifier and no visible label; `label` is
/// its accessible name.
pub fn search_field(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
    let palette = theme::palette(ui.ctx());
    let edit = TextEdit::singleline(value)
        .hint_text(RichText::new(hint).color(palette.text_tertiary))
        .desired_width(f32::INFINITY)
        .margin(Margin {
            left: 32,
            ..FIELD_MARGIN
        })
        .min_size(vec2(0.0, size::CONTROL));
    let response = ui.add(edit);
    response
        .widget_info(|| egui::WidgetInfo::labeled(egui::Role::SearchInput, ui.is_enabled(), label));
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(
            response.rect.left() + space::M - 2.0,
            response.rect.center().y - 7.0,
        ),
        vec2(14.0, 14.0),
    );
    icons::paint(ui.painter(), icon_rect, Icon::Search, palette.text_tertiary);
    response
}

fn input<'t>(edit: TextEdit<'t>, hint: &str) -> TextEdit<'t> {
    edit.hint_text(hint)
        .desired_width(f32::INFINITY)
        .margin(FIELD_MARGIN)
        .min_size(vec2(0.0, size::CONTROL))
}

fn labelled(
    ui: &mut Ui,
    label: &str,
    add_input: impl FnOnce(&mut Ui, egui::Id) -> Response,
) -> Response {
    let palette = theme::palette(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = space::XS + 2.0;
        let label = ui.label(RichText::new(label).small().color(palette.text_secondary));
        add_input(ui, label.id)
    })
    .inner
}
