//! A row of mutually exclusive options (theme, language, log level).

use egui::{Response, Role, Sense, StrokeKind, TextStyle, Ui, WidgetInfo, vec2};

use crate::theme::{self, radius, size, space};

/// Inset between the track and the selected segment.
const INSET: f32 = 2.0;

/// Shows `options` as segments and writes the clicked one into `value`.
#[must_use = "call `show` to draw the control"]
pub struct Segmented<'a, T> {
    id_salt: &'a str,
    value: &'a mut T,
    options: &'a [(T, String)],
}

impl<'a, T: Copy + PartialEq> Segmented<'a, T> {
    /// `id_salt` keeps each segment's ID stable when several controls share a parent.
    pub const fn new(id_salt: &'a str, value: &'a mut T, options: &'a [(T, String)]) -> Self {
        Self {
            id_salt,
            value,
            options,
        }
    }

    /// Draws the control. The returned response is marked changed when the
    /// value changes.
    pub fn show(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let font = TextStyle::Button.resolve(ui.style());
        let segment_width = self
            .options
            .iter()
            .map(|(_, label)| {
                ui.painter()
                    .layout_no_wrap(label.clone(), font.clone(), palette.text)
                    .size()
                    .x
            })
            .fold(0.0_f32, f32::max)
            + space::L * 2.0;
        let count = self.options.len().max(1);
        #[expect(clippy::cast_precision_loss, reason = "a handful of segments")]
        let track_width = segment_width * count as f32 + INSET * 2.0;

        let (track, mut response) =
            ui.allocate_exact_size(vec2(track_width, size::CONTROL), Sense::hover());
        ui.painter().rect_filled(track, radius::MD, palette.field);

        let mut changed = false;
        for (index, (option, label)) in self.options.iter().enumerate() {
            #[expect(clippy::cast_precision_loss, reason = "a handful of segments")]
            let left = track.left() + INSET + segment_width * index as f32;
            let rect = egui::Rect::from_min_size(
                egui::pos2(left, track.top() + INSET),
                vec2(segment_width, track.height() - INSET * 2.0),
            );
            let id = ui.scope_id().with((self.id_salt, index));
            let segment = ui.interact(rect, id, Sense::click());
            let selected = *self.value == *option;
            segment.widget_info(|| {
                WidgetInfo::selected(Role::RadioButton, ui.is_enabled(), selected, label)
            });
            if segment.clicked() && !selected {
                *self.value = *option;
                changed = true;
            }

            let text_color = if selected || segment.hovered() {
                palette.text
            } else {
                palette.text_secondary
            };
            if selected {
                ui.painter().rect(
                    rect,
                    radius::MD - 1,
                    palette.surface_raised,
                    egui::Stroke::new(1.0, palette.border),
                    StrokeKind::Inside,
                );
            }
            let galley = ui
                .painter()
                .layout_no_wrap(label.clone(), font.clone(), text_color);
            ui.painter()
                .galley(rect.center() - galley.size() / 2.0, galley, text_color);
            super::focus_ring(ui, &segment, rect, radius::MD);
            response |= segment;
        }

        if changed {
            response.mark_changed();
        }
        response
    }
}
