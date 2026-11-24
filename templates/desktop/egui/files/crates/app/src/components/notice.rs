//! Inline banners for states that leave the page usable: offline, sync
//! failed, keyring unavailable, read-only data.

use egui::{CornerRadius, Frame, Margin, Response, RichText, Ui, vec2};

use super::{Button, Tone};
use crate::icons::{self, Icon};
use crate::theme::{self, radius, size, space};

/// A tinted banner with an icon, a short title, one sentence, and an
/// optional action. Tone carries meaning; the icon and text repeat it so the
/// state never depends on colour alone.
#[must_use = "call `show` to draw the notice"]
#[derive(Clone, Debug)]
pub struct Notice<'a> {
    tone: Tone,
    title: &'a str,
    body: Option<&'a str>,
    action: Option<&'a str>,
}

impl<'a> Notice<'a> {
    pub const fn new(tone: Tone, title: &'a str) -> Self {
        Self {
            tone,
            title,
            body: None,
            action: None,
        }
    }

    pub const fn body(mut self, body: &'a str) -> Self {
        self.body = Some(body);
        self
    }

    /// Adds a ghost button such as "Retry" or "Sign in".
    pub const fn action(mut self, label: &'a str) -> Self {
        self.action = Some(label);
        self
    }

    /// Draws the notice; returns the action button's response when there is one.
    pub fn show(self, ui: &mut Ui) -> Option<Response> {
        let palette = theme::palette(ui.ctx());
        let color = self.tone.color(&palette);
        let icon = match self.tone {
            super::Tone::Success => Icon::Check,
            super::Tone::Danger => Icon::Close,
            super::Tone::Neutral | super::Tone::Accent | super::Tone::Warning => Icon::About,
        };
        Frame::new()
            .fill(self.tone.fill(&palette))
            .corner_radius(CornerRadius::same(radius::LG))
            .inner_margin(Margin::symmetric(14, 10))
            .show(ui, |ui| {
                ui.set_width(ui.available_width());
                egui::Sides::new()
                    .show(
                        ui,
                        |ui| {
                            ui.horizontal_top(|ui| {
                                let (rect, _) = ui.allocate_exact_size(
                                    vec2(size::ICON, size::ICON),
                                    egui::Sense::hover(),
                                );
                                icons::paint(ui.painter(), rect, icon, color);
                                ui.vertical(|ui| {
                                    ui.spacing_mut().item_spacing.y = 2.0;
                                    ui.label(
                                        RichText::new(self.title)
                                            .font(theme::semibold(14.0))
                                            .color(color),
                                    );
                                    if let Some(body) = self.body {
                                        ui.label(RichText::new(body).color(palette.text_secondary));
                                    }
                                });
                            });
                        },
                        |ui| self.action.map(|label| ui.add(Button::ghost(label))),
                    )
                    .1
            })
            .inner
            .map(|response| {
                ui.add_space(space::XS);
                response
            })
    }
}
