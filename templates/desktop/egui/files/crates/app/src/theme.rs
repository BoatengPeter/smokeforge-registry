//! Design tokens for the Smokeforge shell.
//!
//! Feature code must read colors from this module rather than defining its own
//! palette values.

use egui::{Color32, Stroke, Visuals};

pub const BACKGROUND: Color32 = Color32::from_rgb(14, 15, 17);
pub const PANEL: Color32 = Color32::from_rgb(18, 19, 22);
pub const TITLE_BAR: Color32 = Color32::from_rgb(13, 14, 16);
pub const EXTREME_BACKGROUND: Color32 = Color32::from_rgb(10, 11, 13);
pub const FAINT_BACKGROUND: Color32 = Color32::from_rgb(22, 23, 27);
pub const RAISED: Color32 = Color32::from_rgb(23, 24, 27);
pub const HOVERED: Color32 = Color32::from_rgb(31, 33, 38);
pub const SELECTED_BACKGROUND: Color32 = Color32::from_rgb(26, 27, 31);
pub const SELECTION: Color32 = Color32::from_rgb(47, 89, 158);
pub const BORDER: Color32 = Color32::from_rgb(36, 38, 43);
pub const TEXT: Color32 = Color32::from_rgb(231, 232, 234);
pub const TEXT_SECONDARY: Color32 = Color32::from_rgb(146, 149, 155);
pub const ON_SELECTION: Color32 = Color32::WHITE;
pub const SUCCESS: Color32 = Color32::from_rgb(74, 222, 128);
pub const ACCENT: Color32 = Color32::from_rgb(77, 141, 255);
pub const BORDER_STROKE: Stroke = Stroke {
    width: STROKE_WIDTH,
    color: BORDER,
};

pub const CONTROL_HEIGHT: f32 = 30.0;
pub const STROKE_WIDTH: f32 = 1.0;
pub const SPACE_1: f32 = 4.0;
pub const SPACE_2: f32 = 8.0;
pub const SEARCH_WIDTH: f32 = 240.0;
pub const SORT_WIDTH: f32 = 160.0;

/// Apply the selected theme while keeping the shared palette tokens.
pub fn apply_visuals(ctx: &egui::Context, theme: &str, high_contrast: bool) {
    let resolved = match theme {
        "light" => egui::Theme::Light,
        "system" => ctx.system_theme().unwrap_or(egui::Theme::Dark),
        _ => egui::Theme::Dark,
    };
    let mut visuals = match resolved {
        egui::Theme::Light => Visuals::light(),
        egui::Theme::Dark => Visuals::dark(),
    };

    if resolved == egui::Theme::Dark {
        visuals.panel_fill = BACKGROUND;
        visuals.window_fill = PANEL;
        visuals.extreme_bg_color = EXTREME_BACKGROUND;
        visuals.faint_bg_color = FAINT_BACKGROUND;
        visuals.override_text_color = Some(TEXT);
        visuals.widgets.inactive.bg_fill = RAISED;
        visuals.widgets.hovered.bg_fill = HOVERED;
        visuals.widgets.active.bg_fill = SELECTION;
        visuals.selection.bg_fill = SELECTION;
    } else {
        visuals.panel_fill = Color32::from_rgb(251, 248, 243);
        visuals.window_fill = Color32::from_rgb(255, 253, 249);
        visuals.extreme_bg_color = Color32::from_rgb(241, 236, 227);
        visuals.faint_bg_color = Color32::from_rgb(231, 224, 213);
        visuals.override_text_color = Some(Color32::from_rgb(49, 52, 54));
        visuals.widgets.inactive.bg_fill = Color32::from_rgb(231, 224, 213);
        visuals.widgets.hovered.bg_fill = Color32::from_rgb(241, 236, 227);
        visuals.widgets.active.bg_fill = ACCENT;
        visuals.selection.bg_fill = ACCENT;
    }

    if high_contrast {
        visuals.widgets.noninteractive.bg_stroke = Stroke::new(1.5, TEXT);
        visuals.widgets.inactive.bg_stroke = Stroke::new(1.5, TEXT);
        visuals.override_text_color = Some(if resolved == egui::Theme::Dark {
            Color32::WHITE
        } else {
            Color32::BLACK
        });
    }

    ctx.set_visuals(visuals);
}
