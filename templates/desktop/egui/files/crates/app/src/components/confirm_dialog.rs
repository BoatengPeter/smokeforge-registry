//! A modal that confirms a destructive or irreversible action.
//!
//! Copy names the object and the verb: title `Delete "Roadmap"?`, confirm
//! button `Delete`, never `Are you sure?` / `OK`. Reversible actions (soft
//! delete with Undo) do not need a confirmation.

use egui::{Color32, Context, Id, RichText};

use super::{Button, ButtonKind};
use crate::theme::{self, space};

const WIDTH: f32 = 400.0;

/// The user's answer.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConfirmChoice {
    Confirm,
    Cancel,
}

/// A themed confirmation modal. Escape and clicking outside cancel.
#[must_use = "call `show` every frame while the confirmation is pending"]
#[derive(Clone, Debug)]
pub struct ConfirmDialog<'a> {
    id: Id,
    title: &'a str,
    body: &'a str,
    confirm: &'a str,
    cancel: &'a str,
    kind: ButtonKind,
}

impl<'a> ConfirmDialog<'a> {
    /// A destructive confirmation; the confirm button is red.
    pub const fn destructive(
        id: Id,
        title: &'a str,
        body: &'a str,
        confirm: &'a str,
        cancel: &'a str,
    ) -> Self {
        Self {
            id,
            title,
            body,
            confirm,
            cancel,
            kind: ButtonKind::Danger,
        }
    }

    /// Draws the modal; returns the choice in the frame it is made.
    #[must_use]
    pub fn show(self, ctx: &Context) -> Option<ConfirmChoice> {
        let palette = theme::palette(ctx);
        let mut choice = None;
        let modal = egui::Modal::new(self.id)
            .backdrop_color(Color32::from_black_alpha(90))
            .show(ctx, |ui| {
                ui.set_width(WIDTH);
                ui.label(RichText::new(self.title).heading());
                ui.add_space(space::XS);
                ui.label(RichText::new(self.body).color(palette.text_secondary));
                ui.add_space(space::L);
                egui::Sides::new().show(
                    ui,
                    |_| {},
                    |ui| {
                        // Right-to-left: the confirm button sits at the trailing edge.
                        if ui.add(Button::new(self.kind, self.confirm)).clicked() {
                            choice = Some(ConfirmChoice::Confirm);
                        }
                        let cancel = ui.add(Button::secondary(self.cancel));
                        if cancel.clicked() {
                            choice = Some(ConfirmChoice::Cancel);
                        }
                    },
                );
            });
        if choice.is_none() && modal.should_close() {
            choice = Some(ConfirmChoice::Cancel);
        }
        choice
    }
}
