//! Sidebar navigation rows.

use egui::{Response, Role, Sense, TextStyle, Ui, Widget, WidgetInfo, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, size, space};

const ROW_HEIGHT: f32 = 36.0;

/// Whether the sidebar shows labels or icons only.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum NavDensity {
    #[default]
    Expanded,
    /// Icon only; the label becomes the tooltip and accessible name.
    Collapsed,
}

/// One navigation destination in the sidebar.
#[must_use = "add the item to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct NavItem<'a> {
    icon: Icon,
    label: &'a str,
    selected: bool,
    density: NavDensity,
}

impl<'a> NavItem<'a> {
    pub const fn new(icon: Icon, label: &'a str) -> Self {
        Self {
            icon,
            label,
            selected: false,
            density: NavDensity::Expanded,
        }
    }

    pub const fn selected(mut self, selected: bool) -> Self {
        self.selected = selected;
        self
    }

    pub const fn density(mut self, density: NavDensity) -> Self {
        self.density = density;
        self
    }
}

impl Widget for NavItem<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let width = match self.density {
            NavDensity::Expanded => ui.available_width(),
            NavDensity::Collapsed => ROW_HEIGHT,
        };
        let (rect, response) = ui.allocate_exact_size(vec2(width, ROW_HEIGHT), Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(Role::Tab, ui.is_enabled(), self.selected, self.label)
        });

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();
            let (fill, icon_color, text_color) = if self.selected {
                (palette.selected, palette.accent_text, palette.text)
            } else if hovered {
                (palette.hover, palette.text, palette.text)
            } else {
                (
                    egui::Color32::TRANSPARENT,
                    palette.text_secondary,
                    palette.text_secondary,
                )
            };
            ui.painter().rect_filled(rect, radius::MD, fill);

            let icon_left = match self.density {
                NavDensity::Expanded => rect.left() + space::S + 2.0,
                NavDensity::Collapsed => rect.center().x - size::ICON / 2.0,
            };
            let icon_rect = egui::Rect::from_min_size(
                egui::pos2(icon_left, rect.center().y - size::ICON / 2.0),
                vec2(size::ICON, size::ICON),
            );
            icons::paint(ui.painter(), icon_rect, self.icon, icon_color);

            if self.density == NavDensity::Expanded {
                let text_left = icon_rect.right() + space::S + 2.0;
                let font = TextStyle::Body.resolve(ui.style());
                let galley = super::single_line(
                    ui,
                    self.label,
                    font,
                    text_color,
                    rect.right() - text_left - space::S,
                );
                let pos = egui::pos2(text_left, rect.center().y - galley.size().y / 2.0);
                ui.painter().galley(pos, galley, text_color);
            }
            super::focus_ring(ui, &response, rect, radius::MD);
        }

        match self.density {
            NavDensity::Expanded => response,
            NavDensity::Collapsed => response.on_hover_text(self.label),
        }
    }
}
