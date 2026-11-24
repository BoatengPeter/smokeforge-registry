//! Small status pills. Always text plus colour, never colour alone.

use egui::{Color32, Response, Role, Sense, TextStyle, Ui, Widget, WidgetInfo, vec2};

use crate::theme::{self, Palette, radius};

/// Strength of the tinted badge background over the surface.
const TINT: f32 = 0.14;

/// The meaning a badge conveys.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Tone {
    #[default]
    Neutral,
    Accent,
    Success,
    Warning,
    Danger,
}

impl Tone {
    /// Text colour for this tone.
    #[must_use]
    pub const fn color(self, palette: &Palette) -> Color32 {
        match self {
            Self::Neutral => palette.text_secondary,
            Self::Accent => palette.accent_text,
            Self::Success => palette.success,
            Self::Warning => palette.warning,
            Self::Danger => palette.danger,
        }
    }

    /// Background tint for this tone.
    #[must_use]
    pub fn fill(self, palette: &Palette) -> Color32 {
        match self {
            Self::Neutral => palette.field,
            _ => self.color(palette).gamma_multiply(TINT),
        }
    }
}

/// A status pill such as "Draft" or "Synced".
#[must_use = "add the badge to a Ui with `ui.add(...)`"]
#[derive(Clone, Debug)]
pub struct Badge<'a> {
    label: &'a str,
    tone: Tone,
}

impl<'a> Badge<'a> {
    pub const fn new(label: &'a str, tone: Tone) -> Self {
        Self { label, tone }
    }
}

impl Widget for Badge<'_> {
    fn ui(self, ui: &mut Ui) -> Response {
        let palette = theme::palette(ui.ctx());
        let color = self.tone.color(&palette);
        let font = TextStyle::Small.resolve(ui.style());
        let galley = ui
            .painter()
            .layout_no_wrap(self.label.to_owned(), font, color);
        let size = vec2(galley.size().x + 12.0, 20.0);
        let (rect, response) = ui.allocate_exact_size(size, Sense::hover());
        response.widget_info(|| WidgetInfo::labeled(Role::Label, ui.is_enabled(), self.label));
        if ui.is_rect_visible(rect) {
            ui.painter()
                .rect_filled(rect, radius::SM, self.tone.fill(&palette));
            let pos = rect.center() - galley.size() / 2.0;
            ui.painter().galley(pos, galley, color);
        }
        response
    }
}

#[cfg(test)]
mod tests {
    use egui::Theme;

    use super::*;
    use crate::theme::{Contrast, contrast_ratio};

    /// `fill` (premultiplied) painted over the opaque `background`.
    fn over(fill: Color32, background: Color32) -> Color32 {
        let keep = 1.0 - f32::from(fill.a()) / 255.0;
        let mix = |top: u8, bottom: u8| {
            let value = (f32::from(top) + f32::from(bottom) * keep)
                .round()
                .min(255.0);
            // `value` is clamped to 0..=255 above.
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_sign_loss,
                reason = "clamped"
            )]
            let byte = value as u8;
            byte
        };
        Color32::from_rgb(
            mix(fill.r(), background.r()),
            mix(fill.g(), background.g()),
            mix(fill.b(), background.b()),
        )
    }

    #[test]
    fn badge_text_is_readable_on_its_tint_in_every_theme() {
        for theme in [Theme::Light, Theme::Dark] {
            for contrast in [Contrast::Standard, Contrast::High] {
                let palette = Palette::for_theme(theme, contrast);
                for tone in [Tone::Neutral, Tone::Accent, Tone::Success, Tone::Danger] {
                    let background = over(tone.fill(&palette), palette.surface);
                    let ratio = contrast_ratio(tone.color(&palette), background);
                    assert!(
                        ratio >= 4.5,
                        "{theme:?}/{contrast:?} {tone:?}: {ratio:.2}:1"
                    );
                }
                let warning = over(Tone::Warning.fill(&palette), palette.surface);
                let ratio = contrast_ratio(Tone::Warning.color(&palette), warning);
                assert!(ratio >= 3.0, "{theme:?}/{contrast:?} warning: {ratio:.2}:1");
            }
        }
    }
}
