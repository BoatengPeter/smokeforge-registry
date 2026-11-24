//! An on/off toggle switch.

use egui::{Response, Role, Sense, Ui, Widget, WidgetInfo, vec2};

use crate::theme;

const TRACK: egui::Vec2 = vec2(32.0, 18.0);
const KNOB_INSET: f32 = 2.0;

/// A toggle bound to `on`. `label` is its accessible name; show a visible label
/// next to it (see the settings rows).
#[must_use = "add the switch to a Ui with `ui.add(...)`"]
pub struct Switch<'a> {
    on: &'a mut bool,
    label: &'a str,
}

impl<'a> Switch<'a> {
    pub const fn new(on: &'a mut bool, label: &'a str) -> Self {
        Self { on, label }
    }
}

impl Widget for Switch<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let (rect, mut response) = ui.allocate_exact_size(TRACK, Sense::click());
        if response.clicked() {
            *self.on = !*self.on;
            response.mark_changed();
        }
        let on = *self.on;
        response
            .widget_info(|| WidgetInfo::selected(Role::Switch, ui.is_enabled(), on, self.label));

        if ui.is_rect_visible(rect) {
            let progress = ui.ctx().animate_bool_responsive(response.id, on);
            let track = if on {
                palette.accent
            } else {
                palette.border_strong
            };
            ui.painter().rect_filled(rect, rect.height() / 2.0, track);
            let knob_radius = rect.height() / 2.0 - KNOB_INSET;
            let travel = rect.width() - 2.0 * (knob_radius + KNOB_INSET);
            let center = egui::pos2(
                rect.left() + KNOB_INSET + knob_radius + travel * progress,
                rect.center().y,
            );
            ui.painter()
                .circle_filled(center, knob_radius, palette.on_accent);
            super::focus_ring(ui, &response, rect, 9);
        }
        response
    }
}
