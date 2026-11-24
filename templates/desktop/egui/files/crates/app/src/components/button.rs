//! Text buttons with an optional leading or trailing icon.

use egui::{Response, Role, Sense, StrokeKind, TextStyle, Ui, Widget, WidgetInfo, vec2};

use crate::icons::{self, Icon};
use crate::theme::{self, radius, size, space};

/// How much visual weight a button carries. Use one `Primary` per group.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonKind {
    /// The main action of a page or dialog: accent fill.
    Primary,
    /// Other actions: soft neutral fill.
    Secondary,
    /// Low-emphasis actions in toolbars and rows: no fill until hovered.
    Ghost,
    /// Destructive actions: red text, red tint on hover.
    Danger,
}

/// A themed button. Add it with `ui.add(Button::primary("Save"))`.
#[must_use = "add the button to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct Button<'a> {
    kind: ButtonKind,
    label: &'a str,
    icon: Option<Icon>,
    icon_side: IconSide,
}

/// Which side of the label a button's icon sits on.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
enum IconSide {
    #[default]
    Leading,
    Trailing,
}

impl<'a> Button<'a> {
    pub const fn new(kind: ButtonKind, label: &'a str) -> Self {
        Self {
            kind,
            label,
            icon: None,
            icon_side: IconSide::Leading,
        }
    }

    pub const fn primary(label: &'a str) -> Self {
        Self::new(ButtonKind::Primary, label)
    }

    pub const fn secondary(label: &'a str) -> Self {
        Self::new(ButtonKind::Secondary, label)
    }

    pub const fn ghost(label: &'a str) -> Self {
        Self::new(ButtonKind::Ghost, label)
    }

    pub const fn danger(label: &'a str) -> Self {
        Self::new(ButtonKind::Danger, label)
    }

    /// Shows `icon` before the label.
    pub const fn icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self.icon_side = IconSide::Leading;
        self
    }

    /// Shows `icon` after the label, for forward actions ("Next →").
    pub const fn trailing_icon(mut self, icon: Icon) -> Self {
        self.icon = Some(icon);
        self.icon_side = IconSide::Trailing;
        self
    }

    /// The width this button will take in `ui`, for centring button rows.
    #[must_use]
    pub fn width(&self, ui: &Ui) -> f32 {
        let font = TextStyle::Button.resolve(ui.style());
        let text = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), font, egui::Color32::PLACEHOLDER)
            .size()
            .x;
        space::M * 2.0 + self.icon_width() + text
    }

    const fn icon_width(&self) -> f32 {
        if self.icon.is_some() {
            size::ICON + space::XS + 2.0
        } else {
            0.0
        }
    }
}

impl Widget for Button<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let text_color = match self.kind {
            ButtonKind::Primary => palette.on_accent,
            ButtonKind::Secondary | ButtonKind::Ghost => palette.text,
            ButtonKind::Danger => palette.danger,
        };
        let font = TextStyle::Button.resolve(ui.style());
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), font, text_color);
        let icon_width = self.icon_width();
        let width = space::M * 2.0 + icon_width + galley.size().x;
        let (rect, response) = ui.allocate_exact_size(vec2(width, size::CONTROL), Sense::click());
        response.widget_info(|| WidgetInfo::labeled(Role::Button, ui.is_enabled(), self.label));

        if ui.is_rect_visible(rect) {
            let hovered = response.hovered();
            let pressed = response.is_pointer_button_down_on();
            let fill = match (self.kind, pressed, hovered) {
                (ButtonKind::Primary, _, true) => palette.accent_hover,
                (ButtonKind::Primary, _, false) => palette.accent,
                (ButtonKind::Secondary | ButtonKind::Ghost, true, _) => palette.pressed,
                (ButtonKind::Secondary | ButtonKind::Ghost, false, true) => palette.hover,
                (ButtonKind::Secondary, false, false) => palette.field,
                (ButtonKind::Danger, _, true) => palette.danger_fill.gamma_multiply(0.14),
                (ButtonKind::Ghost | ButtonKind::Danger, _, false) => egui::Color32::TRANSPARENT,
            };
            let stroke = match self.kind {
                ButtonKind::Secondary => egui::Stroke::new(1.0, palette.border),
                _ => egui::Stroke::NONE,
            };
            ui.painter()
                .rect(rect, radius::MD, fill, stroke, StrokeKind::Inside);

            let leading = self.icon_side == IconSide::Leading;
            let text_left = rect.left() + space::M + if leading { icon_width } else { 0.0 };
            if let Some(icon) = self.icon {
                let icon_left = if leading {
                    rect.left() + space::M
                } else {
                    rect.right() - space::M - size::ICON
                };
                let icon_rect = egui::Rect::from_min_size(
                    egui::pos2(icon_left, rect.center().y - size::ICON / 2.0),
                    vec2(size::ICON, size::ICON),
                );
                icons::paint(ui.painter(), icon_rect, icon, text_color);
            }
            let text_pos = egui::pos2(text_left, rect.center().y - galley.size().y / 2.0);
            ui.painter().galley(text_pos, galley, text_color);
            super::focus_ring(ui, &response, rect, radius::MD);
        }

        response
    }
}
