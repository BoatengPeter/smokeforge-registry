//! Design system: colour tokens, egui visuals, type scale, spacing, and fonts.
//!
//! Every colour, size, radius, and shadow the UI uses is defined here. Feature
//! code reads colours through [`palette`] and sizes through the constants below;
//! it never writes a raw colour. `DESIGN.md` documents the same values.
//!
//! The theme is installed once with [`apply`] and only re-applied when the
//! user's [`ThemeSettings`] change. egui keeps a light and a dark style and
//! follows the operating system on its own when the preference is
//! [`ThemePreference::System`].

use std::sync::Arc;

use egui::{
    Color32, Context, CornerRadius, FontData, FontDefinitions, FontFamily, FontId, Margin, Shadow,
    Stroke, TextStyle, Theme, Visuals,
};

/// Spacing scale in points. Layouts use these steps only.
pub mod space {
    pub const XS: f32 = 4.0;
    pub const S: f32 = 8.0;
    pub const M: f32 = 12.0;
    pub const L: f32 = 16.0;
    pub const XL: f32 = 24.0;
    pub const XXL: f32 = 32.0;
}

/// Corner radii in points.
pub mod radius {
    /// Badges, checkbox boxes, small chips.
    pub const SM: u8 = 4;
    /// Buttons, fields, list rows, menus.
    pub const MD: u8 = 6;
    /// Cards, dialogs, popovers.
    pub const LG: u8 = 10;
}

/// Shell and control metrics in points.
pub mod size {
    /// Minimum height of buttons, fields, and list rows.
    pub const CONTROL: f32 = 32.0;
    /// Height of the custom title bar.
    pub const TITLE_BAR: f32 = 44.0;
    /// Height of the status bar.
    pub const STATUS_BAR: f32 = 30.0;
    /// Width of the expanded sidebar.
    pub const SIDEBAR: f32 = 220.0;
    /// Width of the collapsed (icon-only) sidebar.
    pub const SIDEBAR_COLLAPSED: f32 = 52.0;
    /// Icon box for toolbar and navigation icons.
    pub const ICON: f32 = 16.0;
}

/// Font family name for the semibold weight (headings, labels that need weight).
pub const SEMIBOLD: &str = "semibold";
/// Text style for page titles.
pub const TITLE: &str = "title";
/// Text style for large numbers in stat cards.
pub const DISPLAY: &str = "display";

const INTER_REGULAR: &str = "inter-regular";
const INTER_SEMIBOLD: &str = "inter-semibold";
const ANIMATION_SECONDS: f32 = 0.12;

/// Every colour the UI uses, for one theme.
///
/// Text colours are chosen to stay readable on every surface they are drawn
/// on; the tests at the end of this module pin the contrast ratios.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Palette {
    /// Workspace background behind cards and content.
    pub canvas: Color32,
    /// Title bar, sidebar, status bar, cards.
    pub surface: Color32,
    /// Menus, popovers, dialogs, tooltips.
    pub surface_raised: Color32,
    /// Text fields and filled controls at rest.
    pub field: Color32,
    /// Hovered rows and controls.
    pub hover: Color32,
    /// Pressed controls.
    pub pressed: Color32,
    /// Selected rows and the active navigation item.
    pub selected: Color32,
    /// Hairlines, card outlines, separators.
    pub border: Color32,
    /// Field outlines on hover and emphasized dividers.
    pub border_strong: Color32,
    /// Primary text and icons.
    pub text: Color32,
    /// Supporting text: subtitles, captions, inactive navigation.
    pub text_secondary: Color32,
    /// Dim text: placeholders, disabled labels, hints.
    pub text_tertiary: Color32,
    /// Primary action fill.
    pub accent: Color32,
    /// Primary action fill on hover.
    pub accent_hover: Color32,
    /// Text and icons on an accent fill.
    pub on_accent: Color32,
    /// Accent-coloured text and icons on surfaces: links, active navigation.
    pub accent_text: Color32,
    /// Keyboard focus ring.
    pub focus: Color32,
    /// Positive status text.
    pub success: Color32,
    /// Warning status text.
    pub warning: Color32,
    /// Error and destructive text.
    pub danger: Color32,
    /// Destructive action fill (white text on top).
    pub danger_fill: Color32,
    /// Drop shadow under popovers and dialogs.
    pub shadow: Color32,
    /// Series colours for charts, in order.
    pub chart: [Color32; 3],
}

impl Palette {
    /// The light theme.
    #[must_use]
    pub const fn light() -> Self {
        Self {
            canvas: Color32::from_rgb(0xF5, 0xF5, 0xF6),
            surface: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            surface_raised: Color32::from_rgb(0xFF, 0xFF, 0xFF),
            field: Color32::from_rgb(0xF4, 0xF4, 0xF5),
            hover: Color32::from_rgb(0xEC, 0xEC, 0xEE),
            pressed: Color32::from_rgb(0xE2, 0xE2, 0xE5),
            selected: Color32::from_rgb(0xE8, 0xF0, 0xFE),
            border: Color32::from_rgb(0xE4, 0xE4, 0xE7),
            border_strong: Color32::from_rgb(0xD1, 0xD1, 0xD6),
            text: Color32::from_rgb(0x18, 0x18, 0x1B),
            text_secondary: Color32::from_rgb(0x5C, 0x5C, 0x63),
            text_tertiary: Color32::from_rgb(0x7C, 0x7C, 0x84),
            accent: Color32::from_rgb(0x25, 0x63, 0xEB),
            accent_hover: Color32::from_rgb(0x1D, 0x4E, 0xD8),
            on_accent: Color32::WHITE,
            accent_text: Color32::from_rgb(0x1D, 0x4E, 0xD8),
            focus: Color32::from_rgb(0x25, 0x63, 0xEB),
            success: Color32::from_rgb(0x16, 0x65, 0x34),
            warning: Color32::from_rgb(0xB4, 0x53, 0x09),
            danger: Color32::from_rgb(0xC8, 0x1E, 0x1E),
            danger_fill: Color32::from_rgb(0xDC, 0x26, 0x26),
            shadow: Color32::from_black_alpha(26),
            chart: [
                Color32::from_rgb(0xA1, 0xA1, 0xAA),
                Color32::from_rgb(0x25, 0x63, 0xEB),
                Color32::from_rgb(0x16, 0xA3, 0x4A),
            ],
        }
    }

    /// The dark theme.
    #[must_use]
    pub const fn dark() -> Self {
        Self {
            canvas: Color32::from_rgb(0x16, 0x16, 0x19),
            surface: Color32::from_rgb(0x1F, 0x1F, 0x23),
            surface_raised: Color32::from_rgb(0x27, 0x27, 0x2C),
            field: Color32::from_rgb(0x27, 0x27, 0x2C),
            hover: Color32::from_rgb(0x2C, 0x2C, 0x32),
            pressed: Color32::from_rgb(0x35, 0x35, 0x3C),
            selected: Color32::from_rgb(0x1E, 0x2C, 0x48),
            border: Color32::from_rgb(0x2F, 0x2F, 0x35),
            border_strong: Color32::from_rgb(0x44, 0x44, 0x4C),
            text: Color32::from_rgb(0xF4, 0xF4, 0xF5),
            text_secondary: Color32::from_rgb(0xA6, 0xA6, 0xAF),
            text_tertiary: Color32::from_rgb(0x80, 0x80, 0x8A),
            accent: Color32::from_rgb(0x25, 0x63, 0xEB),
            accent_hover: Color32::from_rgb(0x1D, 0x4E, 0xD8),
            on_accent: Color32::WHITE,
            accent_text: Color32::from_rgb(0x6B, 0xA6, 0xFF),
            focus: Color32::from_rgb(0x6B, 0xA6, 0xFF),
            success: Color32::from_rgb(0x4A, 0xDE, 0x80),
            warning: Color32::from_rgb(0xFB, 0xBF, 0x24),
            danger: Color32::from_rgb(0xF8, 0x71, 0x71),
            danger_fill: Color32::from_rgb(0xDC, 0x26, 0x26),
            shadow: Color32::from_black_alpha(110),
            chart: [
                Color32::from_rgb(0x71, 0x71, 0x7A),
                Color32::from_rgb(0x3B, 0x82, 0xF6),
                Color32::from_rgb(0x22, 0xC5, 0x5E),
            ],
        }
    }

    /// The palette for `theme` at `contrast`.
    #[must_use]
    pub const fn for_theme(theme: Theme, contrast: Contrast) -> Self {
        let palette = match theme {
            Theme::Light => Self::light(),
            Theme::Dark => Self::dark(),
        };
        match contrast {
            Contrast::Standard => palette,
            Contrast::High => palette.with_high_contrast(theme),
        }
    }

    /// Strengthens text, borders, and focus for the high-contrast setting.
    const fn with_high_contrast(mut self, theme: Theme) -> Self {
        match theme {
            Theme::Light => {
                self.text = Color32::BLACK;
                self.text_secondary = Color32::from_rgb(0x2E, 0x2E, 0x33);
                self.text_tertiary = Color32::from_rgb(0x52, 0x52, 0x5A);
                self.border = Color32::from_rgb(0x8C, 0x8C, 0x94);
                self.border_strong = Color32::from_rgb(0x52, 0x52, 0x5A);
                self.accent_text = Color32::from_rgb(0x1E, 0x40, 0xAF);
                self.focus = Color32::from_rgb(0x1E, 0x40, 0xAF);
            }
            Theme::Dark => {
                self.text = Color32::WHITE;
                self.text_secondary = Color32::from_rgb(0xD4, 0xD4, 0xD8);
                self.text_tertiary = Color32::from_rgb(0xA6, 0xA6, 0xAF);
                self.border = Color32::from_rgb(0x6B, 0x6B, 0x75);
                self.border_strong = Color32::from_rgb(0xA1, 0xA1, 0xAA);
                self.accent_text = Color32::from_rgb(0x93, 0xC0, 0xFF);
                self.focus = Color32::from_rgb(0x93, 0xC0, 0xFF);
            }
        }
        self
    }
}

/// The user's light/dark choice, as stored in settings.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ThemePreference {
    Light,
    #[default]
    Dark,
    System,
}

impl ThemePreference {
    /// Parses the value stored in settings; unknown values fall back to dark.
    #[must_use]
    pub fn from_setting(value: &str) -> Self {
        match value {
            "light" => Self::Light,
            "system" => Self::System,
            _ => Self::Dark,
        }
    }

    /// The value stored in settings.
    #[must_use]
    pub const fn as_setting(self) -> &'static str {
        match self {
            Self::Light => "light",
            Self::Dark => "dark",
            Self::System => "system",
        }
    }
}

impl From<ThemePreference> for egui::ThemePreference {
    fn from(preference: ThemePreference) -> Self {
        match preference {
            ThemePreference::Light => Self::Light,
            ThemePreference::Dark => Self::Dark,
            ThemePreference::System => Self::System,
        }
    }
}

/// Standard or high-contrast colours.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Contrast {
    #[default]
    Standard,
    High,
}

/// Everything that decides how the theme is installed.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct ThemeSettings {
    pub preference: ThemePreference,
    pub contrast: Contrast,
}

/// Installs the fonts (first call only), both palettes as egui's light and
/// dark visuals, the shared style, and the theme preference. Call it once at
/// startup and again only when [`ThemeSettings`] change. Prefer [`apply_now`]
/// from inside a frame.
pub fn apply(ctx: &Context, settings: ThemeSettings) {
    if !ctx.data(|data| data.get_temp::<bool>(fonts_id()).unwrap_or(false)) {
        install_fonts(ctx);
    }
    for theme in [Theme::Light, Theme::Dark] {
        let palette = Palette::for_theme(theme, settings.contrast);
        ctx.set_visuals_of(theme, visuals(&palette, theme));
    }
    ctx.all_styles_mut(apply_style);
    ctx.set_theme(settings.preference);
    ctx.data_mut(|data| data.insert_temp(contrast_id(), settings.contrast));
}

/// [`apply`], plus the style of `ui` itself, so text styles and colours are
/// already correct in the frame that applies them.
pub fn apply_now(ui: &mut egui::Ui, settings: ThemeSettings) {
    let ctx = ui.ctx().clone();
    apply(&ctx, settings);
    ui.set_style(ctx.global_style());
}

/// The palette for the theme egui is showing this frame.
#[must_use]
pub fn palette(ctx: &Context) -> Palette {
    let contrast = ctx
        .data(|data| data.get_temp::<Contrast>(contrast_id()))
        .unwrap_or_default();
    Palette::for_theme(ctx.theme(), contrast)
}

/// Registers Inter (regular and semibold) as the UI font, ahead of egui's
/// built-in fallbacks. Call once when the app starts; rebuilding the font atlas
/// every frame is expensive.
pub fn install_fonts(ctx: &Context) {
    let mut fonts = FontDefinitions::default();
    fonts.font_data.insert(
        INTER_REGULAR.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../../../assets/fonts/Inter-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        INTER_SEMIBOLD.to_owned(),
        Arc::new(FontData::from_static(include_bytes!(
            "../../../assets/fonts/Inter-SemiBold.ttf"
        ))),
    );

    let fallbacks = fonts
        .families
        .get(&FontFamily::Proportional)
        .cloned()
        .unwrap_or_default();
    let mut regular = vec![INTER_REGULAR.to_owned()];
    regular.extend(fallbacks.iter().cloned());
    let mut semibold = vec![INTER_SEMIBOLD.to_owned()];
    semibold.extend(fallbacks);

    fonts.families.insert(FontFamily::Proportional, regular);
    fonts
        .families
        .insert(FontFamily::Name(SEMIBOLD.into()), semibold);
    ctx.set_fonts(fonts);
    ctx.data_mut(|data| data.insert_temp(fonts_id(), true));
}

/// A semibold font at `size` points.
#[must_use]
pub fn semibold(size: f32) -> FontId {
    FontId::new(size, FontFamily::Name(SEMIBOLD.into()))
}

/// The text style for page titles.
#[must_use]
pub fn title_style() -> TextStyle {
    TextStyle::Name(TITLE.into())
}

/// The text style for large numbers.
#[must_use]
pub fn display_style() -> TextStyle {
    TextStyle::Name(DISPLAY.into())
}

fn fonts_id() -> egui::Id {
    egui::Id::unique("smokeforge-theme-fonts")
}

fn contrast_id() -> egui::Id {
    egui::Id::unique("smokeforge-theme-contrast")
}

fn apply_style(style: &mut egui::Style) {
    let semibold_family = FontFamily::Name(SEMIBOLD.into());
    style.text_styles = [
        (
            TextStyle::Small,
            FontId::new(12.0, FontFamily::Proportional),
        ),
        (TextStyle::Body, FontId::new(14.0, FontFamily::Proportional)),
        (
            TextStyle::Button,
            FontId::new(14.0, FontFamily::Proportional),
        ),
        (
            TextStyle::Monospace,
            FontId::new(13.0, FontFamily::Monospace),
        ),
        (
            TextStyle::Heading,
            FontId::new(15.0, semibold_family.clone()),
        ),
        (title_style(), FontId::new(22.0, semibold_family.clone())),
        (display_style(), FontId::new(28.0, semibold_family)),
    ]
    .into();

    let spacing = &mut style.spacing;
    spacing.item_spacing = egui::vec2(space::S, space::S);
    spacing.button_padding = egui::vec2(space::M, 6.0);
    spacing.interact_size = egui::vec2(size::CONTROL, size::CONTROL);
    spacing.icon_width = 16.0;
    spacing.icon_width_inner = 10.0;
    spacing.menu_margin = Margin::same(6);
    spacing.window_margin = Margin::same(16);
    spacing.scroll = egui::style::ScrollStyle::floating();

    style.interaction.tooltip_delay = 0.4;
    style.animation_time = ANIMATION_SECONDS;
}

fn visuals(palette: &Palette, theme: Theme) -> Visuals {
    let mut visuals = match theme {
        Theme::Light => Visuals::light(),
        Theme::Dark => Visuals::dark(),
    };
    let border = Stroke::new(1.0, palette.border);

    visuals.override_text_color = Some(palette.text);
    visuals.weak_text_color = Some(palette.text_secondary);
    visuals.hyperlink_color = palette.accent_text;
    visuals.panel_fill = palette.surface;
    visuals.window_fill = palette.surface_raised;
    visuals.window_stroke = border;
    visuals.faint_bg_color = palette.hover;
    visuals.extreme_bg_color = palette.field;
    visuals.text_edit_bg_color = Some(palette.field);
    visuals.code_bg_color = palette.field;
    visuals.warn_fg_color = palette.warning;
    visuals.error_fg_color = palette.danger;
    visuals.window_corner_radius = CornerRadius::same(radius::LG);
    visuals.menu_corner_radius = CornerRadius::same(radius::MD);
    visuals.window_shadow = Shadow {
        offset: [0, 8],
        blur: 24,
        spread: 0,
        color: palette.shadow,
    };
    visuals.popup_shadow = Shadow {
        offset: [0, 4],
        blur: 12,
        spread: 0,
        color: palette.shadow,
    };
    visuals.selection.bg_fill = palette.selected;
    visuals.selection.stroke = Stroke::new(1.0, palette.accent_text);
    visuals.text_cursor.stroke = Stroke::new(1.5, palette.accent_text);
    visuals.slider_trailing_fill = true;
    visuals.striped = false;

    let widgets = &mut visuals.widgets;
    for state in [
        &mut widgets.noninteractive,
        &mut widgets.inactive,
        &mut widgets.hovered,
        &mut widgets.active,
        &mut widgets.open,
    ] {
        state.corner_radius = CornerRadius::same(radius::MD);
        state.fg_stroke = Stroke::new(1.0, palette.text);
        state.expansion = 0.0;
    }
    widgets.noninteractive.bg_fill = palette.surface;
    widgets.noninteractive.weak_bg_fill = palette.surface;
    widgets.noninteractive.bg_stroke = border;
    widgets.noninteractive.fg_stroke = Stroke::new(1.0, palette.text_secondary);

    widgets.inactive.bg_fill = palette.field;
    widgets.inactive.weak_bg_fill = palette.field;
    widgets.inactive.bg_stroke = Stroke::NONE;

    widgets.hovered.bg_fill = palette.hover;
    widgets.hovered.weak_bg_fill = palette.hover;
    widgets.hovered.bg_stroke = Stroke::new(1.0, palette.border_strong);

    widgets.active.bg_fill = palette.pressed;
    widgets.active.weak_bg_fill = palette.pressed;
    widgets.active.bg_stroke = Stroke::new(1.0, palette.focus);

    widgets.open.bg_fill = palette.hover;
    widgets.open.weak_bg_fill = palette.hover;
    widgets.open.bg_stroke = Stroke::new(1.0, palette.border_strong);

    visuals
}

/// WCAG relative luminance of an opaque colour.
#[cfg(test)]
fn luminance(color: Color32) -> f64 {
    fn channel(value: u8) -> f64 {
        let c = f64::from(value) / 255.0;
        if c <= 0.039_28 {
            c / 12.92
        } else {
            ((c + 0.055) / 1.055).powf(2.4)
        }
    }
    0.2126 * channel(color.r()) + 0.7152 * channel(color.g()) + 0.0722 * channel(color.b())
}

/// WCAG contrast ratio between two opaque colours (1.0 to 21.0).
#[cfg(test)]
pub(crate) fn contrast_ratio(a: Color32, b: Color32) -> f64 {
    let (light, dark) = {
        let (la, lb) = (luminance(a), luminance(b));
        if la >= lb { (la, lb) } else { (lb, la) }
    };
    (light + 0.05) / (dark + 0.05)
}

#[cfg(test)]
mod tests {
    use super::*;

    const AA_TEXT: f64 = 4.5;
    const AA_LARGE_OR_UI: f64 = 3.0;

    fn every_palette() -> Vec<(String, Palette)> {
        let mut palettes = Vec::new();
        for theme in [Theme::Light, Theme::Dark] {
            for contrast in [Contrast::Standard, Contrast::High] {
                palettes.push((
                    format!("{theme:?}/{contrast:?}"),
                    Palette::for_theme(theme, contrast),
                ));
            }
        }
        palettes
    }

    fn assert_contrast(name: &str, what: &str, fg: Color32, bg: Color32, minimum: f64) {
        let ratio = contrast_ratio(fg, bg);
        assert!(
            ratio >= minimum,
            "{name}: {what} {fg:?} on {bg:?} is {ratio:.2}:1, needs {minimum}:1"
        );
    }

    #[test]
    fn body_and_secondary_text_meet_wcag_aa_on_every_surface() {
        for (name, p) in every_palette() {
            for (surface_name, bg) in [
                ("canvas", p.canvas),
                ("surface", p.surface),
                ("raised", p.surface_raised),
                ("field", p.field),
                ("hover", p.hover),
                ("selected", p.selected),
            ] {
                assert_contrast(
                    &name,
                    &format!("text on {surface_name}"),
                    p.text,
                    bg,
                    AA_TEXT,
                );
                assert_contrast(
                    &name,
                    &format!("secondary text on {surface_name}"),
                    p.text_secondary,
                    bg,
                    AA_TEXT,
                );
                assert_contrast(
                    &name,
                    &format!("tertiary text on {surface_name}"),
                    p.text_tertiary,
                    bg,
                    AA_LARGE_OR_UI,
                );
            }
        }
    }

    #[test]
    fn tertiary_text_stays_dimmer_than_secondary() {
        for (name, p) in every_palette() {
            assert!(
                contrast_ratio(p.text_tertiary, p.surface)
                    < contrast_ratio(p.text_secondary, p.surface),
                "{name}: tertiary must read dimmer than secondary"
            );
        }
    }

    #[test]
    fn accent_and_status_colours_are_readable() {
        for (name, p) in every_palette() {
            assert_contrast(&name, "text on accent", p.on_accent, p.accent, AA_TEXT);
            assert_contrast(
                &name,
                "text on accent hover",
                p.on_accent,
                p.accent_hover,
                AA_TEXT,
            );
            assert_contrast(
                &name,
                "text on danger",
                Color32::WHITE,
                p.danger_fill,
                AA_TEXT,
            );
            for (what, fg) in [
                ("accent text", p.accent_text),
                ("success", p.success),
                ("danger", p.danger),
            ] {
                assert_contrast(&name, what, fg, p.surface, AA_TEXT);
            }
            assert_contrast(&name, "warning", p.warning, p.surface, AA_LARGE_OR_UI);
            assert_contrast(
                &name,
                "accent text on selection",
                p.accent_text,
                p.selected,
                AA_TEXT,
            );
        }
    }

    #[test]
    fn focus_ring_and_strong_borders_are_visible() {
        for (name, p) in every_palette() {
            for bg in [p.canvas, p.surface, p.field] {
                assert_contrast(&name, "focus ring", p.focus, bg, AA_LARGE_OR_UI);
            }
        }
    }

    #[test]
    fn high_contrast_strengthens_text_and_borders() {
        for theme in [Theme::Light, Theme::Dark] {
            let standard = Palette::for_theme(theme, Contrast::Standard);
            let high = Palette::for_theme(theme, Contrast::High);
            assert!(
                contrast_ratio(high.text, high.surface)
                    > contrast_ratio(standard.text, standard.surface)
            );
            assert!(
                contrast_ratio(high.border, high.surface)
                    > contrast_ratio(standard.border, standard.surface)
            );
            assert_contrast(
                &format!("{theme:?}/High"),
                "border",
                high.border,
                high.surface,
                AA_LARGE_OR_UI,
            );
        }
    }

    #[test]
    fn theme_preference_round_trips_through_settings() {
        for preference in [
            ThemePreference::Light,
            ThemePreference::Dark,
            ThemePreference::System,
        ] {
            assert_eq!(
                ThemePreference::from_setting(preference.as_setting()),
                preference
            );
        }
        assert_eq!(
            ThemePreference::from_setting("unknown"),
            ThemePreference::Dark
        );
    }

    #[test]
    fn apply_installs_both_themes_and_the_preference() {
        let ctx = Context::default();
        apply(
            &ctx,
            ThemeSettings {
                preference: ThemePreference::Light,
                contrast: Contrast::Standard,
            },
        );
        assert_eq!(ctx.theme(), Theme::Light);
        assert_eq!(palette(&ctx), Palette::light());
        assert_eq!(
            ctx.style_of(Theme::Dark).visuals.panel_fill,
            Palette::dark().surface
        );
        assert_eq!(
            ctx.style_of(Theme::Light).visuals.panel_fill,
            Palette::light().surface
        );

        apply(
            &ctx,
            ThemeSettings {
                preference: ThemePreference::Dark,
                contrast: Contrast::High,
            },
        );
        assert_eq!(ctx.theme(), Theme::Dark);
        assert_eq!(
            palette(&ctx),
            Palette::for_theme(Theme::Dark, Contrast::High)
        );
    }

    #[test]
    fn inter_is_the_first_proportional_font() {
        let ctx = Context::default();
        install_fonts(&ctx);
        // Fonts are applied at the start of the next pass.
        ctx.run_ui(egui::RawInput::default(), |_| {})
            .textures_delta
            .clear();
        ctx.fonts(|fonts| {
            let families = fonts.definitions().families.clone();
            assert_eq!(
                families
                    .get(&FontFamily::Proportional)
                    .and_then(|list| list.first()),
                Some(&INTER_REGULAR.to_owned())
            );
            assert_eq!(
                families
                    .get(&FontFamily::Name(SEMIBOLD.into()))
                    .and_then(|list| list.first()),
                Some(&INTER_SEMIBOLD.to_owned())
            );
        });
    }
}
