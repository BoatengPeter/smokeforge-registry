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
pub fn apply_visuals(ctx: &egui::Context, theme: &str) {
    let mut visuals = if theme == "light" {
        Visuals::light()
    } else {
        Visuals::dark()
    };
    visuals.panel_fill = BACKGROUND;
    visuals.window_fill = PANEL;
    visuals.extreme_bg_color = EXTREME_BACKGROUND;
    visuals.faint_bg_color = FAINT_BACKGROUND;
    visuals.override_text_color = Some(TEXT);
    visuals.widgets.inactive.bg_fill = RAISED;
    visuals.widgets.hovered.bg_fill = HOVERED;
    visuals.widgets.active.bg_fill = SELECTION;
    visuals.selection.bg_fill = SELECTION;
    ctx.set_visuals(visuals);
}
