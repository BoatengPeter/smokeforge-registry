//! Square icon-only buttons for toolbars, title bars, and rows.

use egui::{Response, Role, Sense, Ui, Widget, WidgetInfo, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, size};

/// The hover treatment of an [`IconButton`].
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum HoverTone {
    /// Neutral hover fill.
    #[default]
    Neutral,
    /// Red hover fill with white icon (window close).
    Danger,
}

/// An icon-only button. `label` is its accessible name and tooltip.
#[must_use = "add the button to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct IconButton<'a> {
    icon: Icon,
    label: &'a str,
    selected: bool,
    hover: HoverTone,
    side: f32,
}

impl<'a> IconButton<'a> {
    pub const fn new(icon: Icon, label: &'a str) -> Self {
        Self {
            icon,
            label,
            selected: false,
            hover: HoverTone::Neutral,
            side: size::CONTROL,
        }
    }

    /// Marks the button as the active choice (pressed toggle).
    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub const fn hover_tone(mut self, hover: HoverTone) -> Self {
        self.hover = hover;
        self
    }

    /// Overrides the square's side length.
    pub const fn side(mut self, side: f32) -> Self {
        self.side = side;
        self
    }
}

impl Widget for IconButton<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let (rect, response) = ui.allocate_exact_size(vec2(self.side, self.side), Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(Role::Button, ui.is_enabled(), self.selected, self.label)
        });

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();
            let (fill, color) = match (self.hover, hovered, self.selected) {
                (HoverTone::Danger, true, _) => (palette.danger_fill, palette.on_accent),
                (_, _, true) => (palette.selected, palette.accent_text),
                (HoverTone::Neutral, true, false) => (palette.hover, palette.text),
                (_, false, false) => (egui::Color32::TRANSPARENT, palette.text_secondary),
            };
            ui.painter().rect_filled(rect, radius::MD, fill);
            let icon_rect =
                egui::Rect::from_center_size(rect.center(), vec2(size::ICON, size::ICON));
            icons::paint(ui.painter(), icon_rect, self.icon, color);
            super::focus_ring(ui, &response, rect, radius::MD);
        }

        response.on_hover_text(self.label)
    }
}
