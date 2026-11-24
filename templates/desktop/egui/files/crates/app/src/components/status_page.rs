//! Full-area states: errors, not found, offline, permission denied.
//!
//! Use a status page when the area cannot show its normal content at all. For
//! a problem that leaves the page usable, use a [`super::Notice`] instead.

use egui::{CornerRadius, Frame, Margin, RichText, ScrollArea, Ui, vec2};

use super::{Button, Tone};
use crate::icons::{self, Icon};
use crate::theme::{self, radius, space};

const BADGE_SIDE: f32 = 48.0;
const MAX_WIDTH: f32 = 440.0;
const DETAILS_HEIGHT: f32 = 120.0;

/// Which of a status page's actions was clicked.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusAction {
    Primary,
    Secondary,
}

/// A centred state with an icon, title, explanation, optional technical
/// details, and up to two actions.
#[must_use = "call `show` to draw the page"]
#[derive(Clone, Debug)]
pub struct StatusPage<'a> {
    icon: Icon,
    tone: Tone,
    title: &'a str,
    body: &'a str,
    details: Option<&'a str>,
    primary: Option<&'a str>,
    secondary: Option<&'a str>,
}

impl<'a> StatusPage<'a> {
    pub const fn new(icon: Icon, tone: Tone, title: &'a str, body: &'a str) -> Self {
        Self {
            icon,
            tone,
            title,
            body,
            details: None,
            primary: None,
            secondary: None,
        }
    }

    /// Selectable technical text (an error chain, a path) for bug reports.
    pub const fn details(mut self, details: &'a str) -> Self {
        self.details = Some(details);
        self
    }

    /// The recommended next step, drawn as the primary button.
    pub const fn primary(mut self, label: &'a str) -> Self {
        self.primary = Some(label);
        self
    }

    /// An alternative step, drawn as a secondary button.
    pub const fn secondary(mut self, label: &'a str) -> Self {
        self.secondary = Some(label);
        self
    }

    pub fn show(self, ui: &mut Ui) -> Option<StatusAction> {
        let palette = theme::palette(ui.ctx());
        let mut clicked = None;
        ui.vertical_centered(|ui| {
            ui.set_max_width(MAX_WIDTH);
            ui.add_space(space::XXL);
            let (rect, _) =
                ui.allocate_exact_size(vec2(BADGE_SIDE, BADGE_SIDE), egui::Sense::hover());
            ui.painter()
                .circle_filled(rect.center(), BADGE_SIDE / 2.0, self.tone.fill(&palette));
            let icon_rect = egui::Rect::from_center_size(rect.center(), vec2(22.0, 22.0));
            icons::paint(
                ui.painter(),
                icon_rect,
                self.icon,
                self.tone.color(&palette),
            );
            ui.add_space(space::M);
            ui.label(RichText::new(self.title).text_style(theme::title_style()));
            ui.add_space(space::XS);
            ui.label(RichText::new(self.body).color(palette.text_secondary));

            if let Some(details) = self.details {
                ui.add_space(space::M);
                Frame::new()
                    .fill(palette.field)
                    .stroke(egui::Stroke::new(1.0, palette.border))
                    .corner_radius(CornerRadius::same(radius::MD))
                    .inner_margin(Margin::same(10))
                    .show(ui, |ui| {
                        ScrollArea::vertical()
                            .id_salt("status-details")
                            .max_height(DETAILS_HEIGHT)
                            .show(ui, |ui| {
                                ui.set_width(ui.available_width());
                                ui.add(
                                    egui::Label::new(RichText::new(details).monospace().small())
                                        .selectable(true)
                                        .wrap(),
                                );
                            });
                    });
            }

            if self.primary.is_some() || self.secondary.is_some() {
                ui.add_space(space::L);
                ui.horizontal(|ui| {
                    let secondary = self.secondary.map(Button::secondary);
                    let primary = self.primary.map(Button::primary);
                    let gap = ui.spacing().item_spacing.x;
                    let buttons = [&secondary, &primary];
                    let row: f32 = buttons
                        .iter()
                        .filter_map(|button| button.as_ref())
                        .map(|button| button.width(ui) + gap)
                        .sum::<f32>()
                        - gap;
                    ui.add_space(((ui.available_width() - row) / 2.0).max(0.0));
                    if let Some(button) = secondary
                        && ui.add(button).clicked()
                    {
                        clicked = Some(StatusAction::Secondary);
                    }
                    if let Some(button) = primary
                        && ui.add(button).clicked()
                    {
                        clicked = Some(StatusAction::Primary);
                    }
                });
            }
            ui.add_space(space::XXL);
        });
        clicked
    }
}
