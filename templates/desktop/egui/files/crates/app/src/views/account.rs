//! Account: API sign-in, session, and manual sync.

use egui::{RichText, Ui};

use crate::components::{
    Badge, Button, Notice, PageHeader, Tone, card, password_field, text_field,
};
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{AuthState, DesktopApp, t};

const FORM_WIDTH: f32 = 420.0;

/// What the account form asked for this frame.
enum AccountAction {
    SignIn,
    SignOut,
    Sync,
}

impl DesktopApp {
    pub(crate) fn account_view(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let mut action = None;
        PageHeader::new(&t("account.title"))
            .subtitle(&t("account.subtitle"))
            .show(ui, |_| {});

        ui.allocate_ui(egui::vec2(FORM_WIDTH, 0.0), |ui| {
            card(ui, |ui| {
                ui.set_width(ui.available_width());
                match &self.auth_state {
                    AuthState::SignedIn(email) => {
                        if let Some(error) = &self.sync_error {
                            let _ = Notice::new(Tone::Warning, &t("state.sync_failed_title"))
                                .body(error)
                                .show(ui);
                            ui.add_space(space::S);
                        }
                        ui.add(Badge::new(&t("account.signed_in"), Tone::Success));
                        ui.add_space(space::S);
                        ui.label(RichText::new(email).heading());
                        if !self.sync_status.is_empty() {
                            ui.label(
                                RichText::new(&self.sync_status).color(palette.text_secondary),
                            );
                        }
                        ui.add_space(space::M);
                        ui.horizontal(|ui| {
                            if self.sync_rx.is_some() {
                                ui.spinner();
                                ui.label(t("state.syncing"));
                            } else if ui
                                .add(Button::primary(&t("account.sync_now")).icon(Icon::Sync))
                                .clicked()
                            {
                                action = Some(AccountAction::Sync);
                            }
                            if ui.add(Button::ghost(&t("account.sign_out"))).clicked() {
                                action = Some(AccountAction::SignOut);
                            }
                        });
                    }
                    AuthState::Loading => {
                        ui.horizontal(|ui| {
                            ui.spinner();
                            ui.label(t("common.loading"));
                        });
                    }
                    AuthState::SignedOut | AuthState::Failed(_) => {
                        if let AuthState::Failed(error) = &self.auth_state {
                            let _ = Notice::new(Tone::Danger, &t("state.sign_in_failed_title"))
                                .body(error)
                                .show(ui);
                            ui.add_space(space::S);
                        }
                        text_field(
                            ui,
                            &t("account.email"),
                            &mut self.auth_email,
                            "you@example.com",
                        );
                        ui.add_space(space::S);
                        password_field(ui, &t("account.password"), &mut self.auth_password, "");
                        ui.add_space(space::M);
                        if ui.add(Button::primary(&t("account.sign_in"))).clicked() {
                            action = Some(AccountAction::SignIn);
                        }
                    }
                }
            });
        });

        match action {
            Some(AccountAction::SignIn) => self.start_login(ui.ctx()),
            Some(AccountAction::Sync) => self.start_sync(),
            Some(AccountAction::SignOut) => {
                self.access_token = None;
                self.tokens.delete().ok();
                self.auth_state = AuthState::SignedOut;
            }
            None => {}
        }
    }
}
