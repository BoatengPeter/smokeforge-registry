//! A search-and-pick window near the top of the app ("Go to…").
//!
//! [`PickerWindow`] draws the frame: a title, an "Esc" chip that closes, and
//! the caller's contents. Inside it use [`picker_search`] (an accent-outlined
//! field), [`picker_group`] headings, and [`PickerRow`]s. The caller owns the
//! query, the rows, and the selection (see `crate::go_to`).

use egui::{
    Align2, Color32, Context, Frame, Id, Margin, Response, RichText, Role, Sense, Stroke,
    StrokeKind, TextEdit, TextStyle, Ui, Widget, WidgetInfo, vec2,
};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, space};

const WIDTH: f32 = 640.0;
/// Gap between the top of the app window and the picker.
const TOP_OFFSET: f32 = 40.0;
const SEARCH_HEIGHT: f32 = 36.0;
const ROW_HEIGHT: f32 = 32.0;

/// What [`PickerWindow::show`] reports.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PickerResponse {
    /// Escape, the Esc chip, or a click outside.
    pub close: bool,
}

/// The picker frame. Call [`PickerWindow::show`] every frame while it is open.
#[must_use = "call `show` every frame while the picker is open"]
pub struct PickerWindow<'a> {
    id: Id,
    title: &'a str,
    escape_label: &'a str,
}

impl<'a> PickerWindow<'a> {
    /// `escape_label` is the chip text ("Esc"); it also closes the picker.
    pub const fn new(id: Id, title: &'a str, escape_label: &'a str) -> Self {
        Self {
            id,
            title,
            escape_label,
        }
    }

    pub fn show(self, ctx: &Context, add_contents: impl FnOnce(&mut Ui)) -> PickerResponse {
        let palette = theme::palette(ctx);
        let screen = ctx.content_rect();
        let width = WIDTH.min(screen.width() - space::XXL);
        let area =
            egui::Modal::default_area(self.id).anchor(Align2::CENTER_TOP, vec2(0.0, TOP_OFFSET));
        let mut chip_clicked = false;
        let modal = egui::Modal::new(self.id)
            .area(area)
            .backdrop_color(Color32::from_black_alpha(40))
            .frame(
                Frame::new()
                    .fill(palette.surface_raised)
                    .stroke(Stroke::new(1.0, palette.border))
                    .corner_radius(radius::LG)
                    .shadow(ctx.global_style().visuals.popup_shadow)
                    .inner_margin(Margin::same(18)),
            )
            .show(ctx, |ui| {
                ui.set_width(width - 36.0);
                ui.spacing_mut().item_spacing.y = space::M - 2.0;
                egui::Sides::new().show(
                    ui,
                    |ui| {
                        ui.label(RichText::new(self.title).font(theme::semibold(15.0)));
                    },
                    |ui| {
                        chip_clicked = escape_chip(ui, self.escape_label).clicked();
                    },
                );
                add_contents(ui);
            });
        PickerResponse {
            close: chip_clicked || modal.should_close(),
        }
    }
}

/// The clickable "Esc" chip in the title row.
fn escape_chip(ui: &mut Ui, label: &str) -> Response {
    let palette = theme::palette(ui.ctx());
    let font = TextStyle::Small.resolve(ui.style());
    let galley = ui
        .painter()
        .layout_no_wrap(label.to_owned(), font, palette.text_tertiary);
    let (rect, response) = ui.allocate_exact_size(galley.size() + vec2(12.0, 4.0), Sense::click());
    response.widget_info(|| WidgetInfo::labeled(Role::Button, true, label));
    let color = if response.hovered() {
        ui.painter().rect_filled(rect, radius::SM, palette.hover);
        palette.text
    } else {
        palette.text_tertiary
    };
    ui.painter()
        .galley(rect.center() - galley.size() / 2.0, galley, color);
    response
}

/// The picker's search field: accent outline, search icon, `label` for screen
/// readers. Request focus on the response when the picker opens.
pub fn picker_search(ui: &mut Ui, label: &str, value: &mut String, hint: &str) -> Response {
    let palette = theme::palette(ui.ctx());
    let edit = TextEdit::singleline(value)
        .hint_text(RichText::new(hint).color(palette.text_tertiary))
        .desired_width(f32::INFINITY)
        .frame(Frame::NONE)
        .margin(Margin {
            left: 34,
            right: 10,
            top: 9,
            bottom: 9,
        })
        .min_size(vec2(0.0, SEARCH_HEIGHT));
    let response = ui.add(edit);
    response.widget_info(|| WidgetInfo::labeled(Role::SearchInput, ui.is_enabled(), label));
    ui.painter().rect(
        response.rect,
        radius::LG - 1,
        Color32::TRANSPARENT,
        Stroke::new(1.0, palette.accent),
        StrokeKind::Inside,
    );
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(response.rect.left() + 11.0, response.rect.center().y - 7.5),
        vec2(15.0, 15.0),
    );
    icons::paint(ui.painter(), icon_rect, Icon::Search, palette.text_tertiary);
    response
}

/// A small dim heading above a group of rows.
pub fn picker_group(ui: &mut Ui, label: &str) {
    let palette = theme::palette(ui.ctx());
    ui.add_space(space::XS);
    ui.horizontal(|ui| {
        ui.add_space(space::S + 2.0);
        ui.label(
            RichText::new(label)
                .font(theme::semibold(11.0))
                .color(palette.text_tertiary),
        );
    });
}

/// What sits before a row's label.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RowLead {
    Icon(Icon),
    /// A "+" for rows that create something or run an action.
    Plus,
}

/// One pickable row: lead glyph, label, optional trailing hint.
#[must_use = "add the row to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct PickerRow<'a> {
    lead: RowLead,
    label: &'a str,
    hint: Option<&'a str>,
    selected: bool,
}

impl<'a> PickerRow<'a> {
    pub const fn new(lead: RowLead, label: &'a str) -> Self {
        Self {
            lead,
            label,
            hint: None,
            selected: false,
        }
    }

    /// Dim text at the trailing edge (a shortcut, a status).
    pub const fn hint(mut self, hint: &'a str) -> Self {
        self.hint = Some(hint);
        self
    }

    /// The keyboard selection; Enter runs it.
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for PickerRow<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), ROW_HEIGHT), Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(
                Role::ListBoxOption,
                ui.is_enabled(),
                self.selected,
                self.label,
            )
        });
        if self.selected {
            response.scroll_to_me(None);
        }
        if ui.is_rect_visible(rect) {
            if self.selected || response.hovered() {
                ui.painter()
                    .rect_filled(rect, radius::MD + 1, palette.hover);
            }
            let lead_color = match self.lead {
                RowLead::Icon(_) => palette.text_secondary,
                RowLead::Plus => palette.text_tertiary,
            };
            let icon = match self.lead {
                RowLead::Icon(icon) => icon,
                RowLead::Plus => Icon::Plus,
            };
            let icon_rect = egui::Rect::from_min_size(
                egui::pos2(rect.left() + space::S + 2.0, rect.center().y - 7.0),
                vec2(14.0, 14.0),
            );
            icons::paint(ui.painter(), icon_rect, icon, lead_color);

            let text_color = match self.lead {
                RowLead::Icon(_) => palette.text,
                RowLead::Plus => palette.text_secondary,
            };
            let font = TextStyle::Body.resolve(ui.style());
            let hint_width = self.hint.map_or(0.0, |hint| {
                let small = TextStyle::Small.resolve(ui.style());
                let galley =
                    ui.painter()
                        .layout_no_wrap(hint.to_owned(), small, palette.text_tertiary);
                let width = galley.size().x;
                let pos = egui::pos2(
                    rect.right() - space::S - 2.0 - width,
                    rect.center().y - galley.size().y / 2.0,
                );
                ui.painter().galley(pos, galley, palette.text_tertiary);
                width + space::M
            });
            let text_left = icon_rect.right() + space::S + 2.0;
            let galley = super::single_line(
                ui,
                self.label,
                font,
                text_color,
                rect.right() - text_left - space::S - hint_width,
            );
            ui.painter().galley(
                egui::pos2(text_left, rect.center().y - galley.size().y / 2.0),
                galley,
                text_color,
            );
            super::focus_ring(ui, &response, rect, radius::MD);
        }
        response
    }
}
