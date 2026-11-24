//! A − value + control for small numeric settings such as interface size.

use std::ops::RangeInclusive;

use egui::{Response, Role, Sense, StrokeKind, TextStyle, Ui, WidgetInfo, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, size};

const INSET: f32 = 2.0;
const STEP_BUTTON_WIDTH: f32 = 30.0;
const VALUE_WIDTH: f32 = 52.0;

/// How a [`Stepper`] prints its value.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StepperFormat {
    /// `1.5`
    #[default]
    Number,
    /// `150%` (the value is a factor; 1.0 is 100%).
    Percent,
}

impl StepperFormat {
    fn format(self, value: f32) -> String {
        match self {
            Self::Number => format!("{value:.1}"),
            Self::Percent => format!("{:.0}%", value * 100.0),
        }
    }
}

/// Decrease and increase buttons around the current value.
///
/// `label` is the accessible name of the whole control ("Interface size");
/// the buttons are named "{label}: decrease" / "{label}: increase" by the
/// caller-supplied `decrease` and `increase` labels.
#[must_use = "call `show` to draw the control"]
pub struct Stepper<'a> {
    id_salt: &'a str,
    value: &'a mut f32,
    range: RangeInclusive<f32>,
    step: f32,
    format: StepperFormat,
    decrease: &'a str,
    increase: &'a str,
}

impl<'a> Stepper<'a> {
    /// `decrease` and `increase` name the two buttons for screen readers.
    pub const fn new(
        id_salt: &'a str,
        value: &'a mut f32,
        range: RangeInclusive<f32>,
        step: f32,
        decrease: &'a str,
        increase: &'a str,
    ) -> Self {
        Self {
            id_salt,
            value,
            range,
            step,
            format: StepperFormat::Number,
            decrease,
            increase,
        }
    }

    pub const fn format(mut self, format: StepperFormat) -> Self {
        self.format = format;
        self
    }

    /// Draws the control. The returned response is marked changed when the
    /// value changes.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let width = STEP_BUTTON_WIDTH * 2.0 + VALUE_WIDTH + INSET * 2.0;
        let (track, mut response) =
            ui.allocate_exact_size(vec2(width, size::CONTROL), Sense::hover());
        ui.painter().rect(
            track,
            radius::MD,
            palette.field,
            egui::Stroke::new(1.0, palette.border),
            StrokeKind::Inside,
        );

        let inner = track.shrink(INSET);
        let minus = egui::Rect::from_min_size(inner.min, vec2(STEP_BUTTON_WIDTH, inner.height()));
        let plus = egui::Rect::from_min_size(
            egui::pos2(inner.right() - STEP_BUTTON_WIDTH, inner.top()),
            vec2(STEP_BUTTON_WIDTH, inner.height()),
        );
        let current = *self.value;
        let lowered = step_value(current, -self.step, &self.range);
        let raised = step_value(current, self.step, &self.range);

        let mut next = current;
        for (rect, icon, label, target) in [
            (minus, Icon::Minus, self.decrease, lowered),
            (plus, Icon::Plus, self.increase, raised),
        ] {
            let enabled = ui.is_enabled() && (target - current).abs() > f32::EPSILON;
            let id = ui.scope_id().with((self.id_salt, label));
            let button = ui.interact(rect, id, Sense::click());
            button.widget_info(|| WidgetInfo::labeled(Role::Button, enabled, label));
            if enabled && button.clicked() {
                next = target;
            }
            let color = if !enabled {
                palette.text_tertiary
            } else if button.hovered() {
                palette.text
            } else {
                palette.text_secondary
            };
            if enabled && button.hovered() {
                ui.painter()
                    .rect_filled(rect, radius::MD - 1, palette.hover);
            }
            let icon_rect = egui::Rect::from_center_size(rect.center(), vec2(14.0, 14.0));
            icons::paint(ui.painter(), icon_rect, icon, color);
            super::focus_ring(ui, &button, rect, radius::MD);
            response |= button.on_hover_text(label);
        }

        let text = self.format.format(next);
        let galley =
            ui.painter()
                .layout_no_wrap(text, TextStyle::Button.resolve(ui.style()), palette.text);
        ui.painter()
            .galley(inner.center() - galley.size() / 2.0, galley, palette.text);

        if (next - current).abs() > f32::EPSILON {
            *self.value = next;
            response.mark_changed();
        }
        response
    }
}

/// `value + delta`, clamped to `range` and rounded to two decimals so repeated
/// steps do not drift (0.1 + 0.2 stays 0.3).
#[must_use]
pub fn step_value(value: f32, delta: f32, range: &RangeInclusive<f32>) -> f32 {
    let stepped = ((value + delta) * 100.0).round() / 100.0;
    stepped.clamp(*range.start(), *range.end())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn steps_round_and_clamp() {
        let range = 0.7..=2.0;
        assert!((step_value(1.0, 0.1, &range) - 1.1).abs() < f32::EPSILON);
        assert!((step_value(1.2, 0.1, &range) - 1.3).abs() < f32::EPSILON);
        assert!((step_value(0.7, -0.1, &range) - 0.7).abs() < f32::EPSILON);
        assert!((step_value(2.0, 0.1, &range) - 2.0).abs() < f32::EPSILON);
    }

    #[test]
    fn percent_format_has_no_decimals() {
        assert_eq!(StepperFormat::Percent.format(1.3), "130%");
        assert_eq!(StepperFormat::Number.format(1.25), "1.2");
    }
}
