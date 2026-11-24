//! Empty, zero-result, and nothing-selected states that name the next action.

use egui::{Response, RichText, Ui, vec2};

use super::Button;
use crate::icons::{self, Icon};
use crate::theme::{self, space};

const BADGE_SIDE: f32 = 40.0;
const MAX_TEXT_WIDTH: f32 = 320.0;

/// A centred icon, title, explanation, and optional primary action.
#[must_use = "call `show` to draw the empty state"]
#[derive(Clone, Debug)]
pub struct EmptyState<'a> {
    icon: Icon,
    title: &'a str,
    body: &'a str,
    action: Option<&'a str>,
}

impl<'a> EmptyState<'a> {
    pub const fn new(icon: Icon, title: &'a str, body: &'a str) -> Self {
        Self {
            icon,
            title,
            body,
            action: None,
        }
    }

    /// Adds a primary button labelled `label`.
    pub const fn action(mut self, label: &'a str) -> Self {
        self.action = Some(label);
        self
    }

    /// Draws the state; returns the action button's response when there is one.
    pub fn show(self, ui: &mut Ui) -> Option<Response> {
        let palette = theme::palette(ui.ctx());
        ui.vertical_centered(|ui| {
            ui.add_space(space::XL);
            let (rect, _) =
                ui.allocate_exact_size(vec2(BADGE_SIDE, BADGE_SIDE), egui::Sense::hover());
            ui.painter()
                .circle_filled(rect.center(), BADGE_SIDE / 2.0, palette.field);
            let icon_rect = egui::Rect::from_center_size(rect.center(), vec2(18.0, 18.0));
            icons::paint(ui.painter(), icon_rect, self.icon, palette.text_secondary);
            ui.add_space(space::S);
            ui.label(RichText::new(self.title).heading());
            ui.set_max_width(MAX_TEXT_WIDTH);
            ui.label(RichText::new(self.body).color(palette.text_secondary));
            let response = self.action.map(|label| {
                ui.add_space(space::S);
                ui.add(Button::primary(label))
            });
            ui.add_space(space::XL);
            response
        })
        .inner
    }
}
