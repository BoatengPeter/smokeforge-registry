//! The Smokeforge light and dark themes, the Inter UI font, and layout tokens.
//!
//! Colours live in `themes/smokeforge.json` in GPUI Kit's theme format, with the
//! same values as the egui template's palette (`DESIGN.md` lists them). Feature
//! code reads colours through `cx.theme()` and sizes through GPUI's rem-based
//! helpers (`p_4()`, `gap_3()`, `text_sm()`); it never writes a raw colour. The
//! theme's base font size (16 px) is the rem, so it also acts as the app zoom.
//!
//! Call [`install`] once after `gpui_kit::init`, then switch modes with
//! `Theme::change` as usual: it now loads these themes instead of GPUI Kit's
//! defaults.

use std::borrow::Cow;

use anyhow::{Context as _, Result};
use gpui_kit::App;
use gpui_kit::component::{Theme, ThemeRegistry};

/// The theme set: one light and one dark theme.
pub const THEMES_JSON: &str = include_str!("../themes/smokeforge.json");
/// Name of the light theme in [`THEMES_JSON`].
pub const LIGHT_THEME: &str = "Smokeforge Light";
/// Name of the dark theme in [`THEMES_JSON`].
pub const DARK_THEME: &str = "Smokeforge Dark";

const INTER_REGULAR: &[u8] = include_bytes!("../../../assets/fonts/Inter-Regular.ttf");
const INTER_SEMIBOLD: &[u8] = include_bytes!("../../../assets/fonts/Inter-SemiBold.ttf");

/// Registers the Inter font and the Smokeforge themes, then re-applies the
/// current mode so the new colours, font, and radii take effect.
///
/// # Errors
///
/// Returns an error when the embedded font cannot be loaded or the theme JSON
/// does not parse. Both are build-time assets, so an error means they were
/// edited incorrectly.
pub fn install(cx: &mut App) -> Result<()> {
    cx.text_system()
        .add_fonts(vec![
            Cow::Borrowed(INTER_REGULAR),
            Cow::Borrowed(INTER_SEMIBOLD),
        ])
        .context("load the embedded Inter font")?;

    ThemeRegistry::global_mut(cx)
        .load_themes_from_str(THEMES_JSON)
        .context("parse themes/smokeforge.json")?;
    let registry = ThemeRegistry::global(cx);
    let light = registry
        .themes()
        .get(LIGHT_THEME)
        .cloned()
        .with_context(|| format!("{LIGHT_THEME} is missing from themes/smokeforge.json"))?;
    let dark = registry
        .themes()
        .get(DARK_THEME)
        .cloned()
        .with_context(|| format!("{DARK_THEME} is missing from themes/smokeforge.json"))?;

    Theme::update(cx, |theme| {
        theme.light_theme = light;
        theme.dark_theme = dark;
    });
    let mode = Theme::global(cx).mode;
    Theme::change(mode, None, cx);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A colour from the theme file: `#rrggbb` or `#rrggbbaa`.
    fn rgb(value: &str) -> (f64, f64, f64) {
        let hex = value.trim_start_matches('#');
        assert!(hex.len() == 6 || hex.len() == 8, "{value} is not #rrggbb");
        let channel = |index: usize| {
            let byte = u8::from_str_radix(&hex[index..index + 2], 16)
                .unwrap_or_else(|error| panic!("{value}: {error}"));
            f64::from(byte) / 255.0
        };
        (channel(0), channel(2), channel(4))
    }

    fn luminance(value: &str) -> f64 {
        let linear = |c: f64| {
            if c <= 0.039_28 {
                c / 12.92
            } else {
                ((c + 0.055) / 1.055).powf(2.4)
            }
        };
        let (r, g, b) = rgb(value);
        0.2126 * linear(r) + 0.7152 * linear(g) + 0.0722 * linear(b)
    }

    fn contrast(a: &str, b: &str) -> f64 {
        let (la, lb) = (luminance(a), luminance(b));
        (la.max(lb) + 0.05) / (la.min(lb) + 0.05)
    }

    fn themes() -> Vec<serde_json::Value> {
        let set: serde_json::Value =
            serde_json::from_str(THEMES_JSON).expect("themes/smokeforge.json is valid JSON");
        set["themes"].as_array().expect("a themes array").clone()
    }

    fn color<'a>(theme: &'a serde_json::Value, key: &str) -> &'a str {
        theme["colors"][key]
            .as_str()
            .unwrap_or_else(|| panic!("{} has no colour {key}", theme["name"]))
    }

    #[test]
    fn the_file_defines_one_light_and_one_dark_theme_with_inter() {
        let themes = themes();
        let names: Vec<_> = themes.iter().map(|theme| theme["name"].as_str()).collect();
        assert_eq!(names, vec![Some(LIGHT_THEME), Some(DARK_THEME)]);
        for theme in &themes {
            assert_eq!(theme["font.family"], "Inter", "{}", theme["name"]);
        }
    }

    #[test]
    fn text_pairs_meet_wcag_aa() {
        let pairs = [
            ("foreground", "background", 4.5),
            ("foreground", "sidebar.background", 4.5),
            ("foreground", "popover.background", 4.5),
            ("muted.foreground", "background", 4.5),
            ("muted.foreground", "muted.background", 4.5),
            ("muted.foreground", "group_box.background", 4.5),
            ("sidebar.foreground", "sidebar.background", 4.5),
            (
                "sidebar.accent.foreground",
                "sidebar.accent.background",
                4.5,
            ),
            ("accent.foreground", "accent.background", 4.5),
            ("primary.foreground", "primary.background", 4.5),
            ("primary.foreground", "primary.hover.background", 4.5),
            ("danger.foreground", "danger.background", 4.5),
            ("secondary.foreground", "secondary.background", 4.5),
            ("tab.active.foreground", "tab.active.background", 4.5),
            ("tab.foreground", "tab.background", 4.5),
            ("link", "background", 4.5),
            ("ring", "background", 3.0),
            ("ring", "group_box.background", 3.0),
        ];
        for theme in themes() {
            for (fg, bg, minimum) in pairs {
                let ratio = contrast(color(&theme, fg), color(&theme, bg));
                assert!(
                    ratio >= minimum,
                    "{}: {fg} on {bg} is {ratio:.2}:1, needs {minimum}:1",
                    theme["name"]
                );
            }
        }
    }
}
