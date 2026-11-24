//! Selectable list rows with a title, optional subtitle, and optional badge.

use egui::{Response, Role, Sense, TextStyle, Ui, Widget, WidgetInfo, vec2};

use super::{Badge, Tone};
use crate::theme::{self, radius, space};

const HEIGHT_SINGLE: f32 = 38.0;
const HEIGHT_DOUBLE: f32 = 52.0;
/// Space reserved for the badge on the right.
const BADGE_RESERVE: f32 = 88.0;

/// A clickable row in a list. Its accessible name is the title.
#[must_use = "add the row to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct ListRow<'a> {
    title: &'a str,
    subtitle: Option<&'a str>,
    badge: Option<(&'a str, Tone)>,
    selected: bool,
}

impl<'a> ListRow<'a> {
    pub const fn new(title: &'a str) -> Self {
        Self {
            title,
            subtitle: None,
            badge: None,
            selected: false,
        }
    }

    pub const fn subtitle(mut self, subtitle: &'a str) -> Self {
        self.subtitle = Some(subtitle);
        self
    }

    pub const fn badge(mut self, label: &'a str, tone: Tone) -> Self {
        self.badge = Some((label, tone));
        self
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }
}

impl Widget for ListRow<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let height = if self.subtitle.is_some() {
            HEIGHT_DOUBLE
        } else {
            HEIGHT_SINGLE
        };
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), height), Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(Role::ListItem, ui.is_enabled(), self.selected, self.title)
        });

        if ui.is_rect_visible(rect) {
            let fill = if self.selected {
                palette.selected
            } else if response.hovered() {
                palette.hover
            } else {
                egui::Color32::TRANSPARENT
            };
            ui.painter().rect_filled(rect, radius::MD, fill);

            let reserve = if self.badge.is_some() {
                BADGE_RESERVE
            } else {
                0.0
            };
            let text_width = rect.width() - space::M * 2.0 - reserve;
            let left = rect.left() + space::M;
            let title = super::single_line(
                ui,
                self.title,
                TextStyle::Body.resolve(ui.style()),
                palette.text,
                text_width,
            );
            match self.subtitle {
                Some(subtitle) => {
                    let sub = super::single_line(
                        ui,
                        subtitle,
                        TextStyle::Small.resolve(ui.style()),
                        palette.text_secondary,
                        text_width,
                    );
                    let top = rect.center().y - (title.size().y + sub.size().y + 2.0) / 2.0;
                    let sub_top = top + title.size().y + 2.0;
                    ui.painter()
                        .galley(egui::pos2(left, top), title, palette.text);
                    ui.painter()
                        .galley(egui::pos2(left, sub_top), sub, palette.text_secondary);
                }
                None => {
                    let top = rect.center().y - title.size().y / 2.0;
                    ui.painter()
                        .galley(egui::pos2(left, top), title, palette.text);
                }
            }

            if let Some((label, tone)) = self.badge {
                let badge_rect = egui::Rect::from_min_max(
                    egui::pos2(rect.right() - BADGE_RESERVE, rect.top()),
                    egui::pos2(rect.right() - space::M, rect.bottom()),
                );
                ui.put(badge_rect, |ui: &mut Ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add(Badge::new(label, tone))
                    })
                    .inner
                });
            }
            super::focus_ring(ui, &response, rect, radius::MD);
        }
        response
    }
}
