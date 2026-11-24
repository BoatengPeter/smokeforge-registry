//! The first-run welcome tour: a floating, draggable window over the live app.
//!
//! It opens on first launch (`settings.onboarded == false`) and again from
//! Settings › Help. Closing it any way marks the tour as seen. The window
//! chrome is [`WizardWindow`]; this module only draws the pages. To add a page:
//! add a [`WelcomePage`] variant (in `crate::welcome`), its locale keys under `welcome.`,
//! and a match arm in `DesktopApp::welcome_page`. See `docs/STATES.md`.

use egui::{RichText, Ui};

use crate::components::{
    Button, FeatureRow, Kbd, Segmented, Stepper, StepperFormat, WizardEvent, WizardText,
    WizardWindow, callout, inset, shortcut_row,
};
use crate::icons::Icon;
use crate::theme::{self, ThemePreference, space, zoom};
use crate::welcome::{WelcomeOutcome, WelcomePage};
use crate::{Action, DesktopApp, shortcuts, system, t};

/// Width of the label column in the appearance/size rows.
const LABEL_WIDTH: f32 = 96.0;

impl DesktopApp {
    /// Draws the welcome tour when it is open and dispatches what it asks for.
    pub(crate) fn welcome_window(&mut self, ui: &mut Ui) {
        let Some(page) = self.welcome else {
            return;
        };
        let title = rust_i18n::t!("welcome.window_title", app = &self.app_name).to_string();
        let next = if page.next().is_some() {
            t("welcome.next")
        } else {
            t("welcome.finish")
        };
        let progress = rust_i18n::t!(
            "welcome.progress",
            step = page.index() + 1,
            total = WelcomePage::ALL.len()
        )
        .to_string();
        let text = WizardText {
            back: &t("welcome.back"),
            next: &next,
            close: &t("welcome.skip"),
            drag_hint: &t("welcome.drag_hint"),
            progress: &progress,
        };
        let version = format!("v{}", env!("CARGO_PKG_VERSION"));

        let mut actions = Vec::new();
        let event = WizardWindow::new(
            egui::Id::unique("welcome-tour"),
            &title,
            page.index(),
            WelcomePage::ALL.len(),
            text,
        )
        .badge(&version)
        .logo(egui::include_image!("../../../../assets/logo.png"))
        .show(ui.ctx(), |ui| self.welcome_page(ui, page, &mut actions));

        match event {
            Some(WizardEvent::Close) => {
                actions.push(Action::CloseWelcome(WelcomeOutcome::Skipped));
            }
            Some(WizardEvent::Back) => {
                if let Some(previous) = page.previous() {
                    actions.push(Action::WelcomeGo(previous));
                }
            }
            Some(WizardEvent::Next) => actions.push(match page.next() {
                Some(next) => Action::WelcomeGo(next),
                None => Action::CloseWelcome(WelcomeOutcome::Finished),
            }),
            None => {}
        }
        for action in actions {
            self.dispatch(action);
        }
    }

    fn welcome_page(&self, ui: &mut Ui, page: WelcomePage, actions: &mut Vec<Action>) {
        ui.spacing_mut().item_spacing.y = space::L;
        match page {
            WelcomePage::Intro => self.intro_page(ui, actions),
            WelcomePage::Shortcuts => shortcuts_page(ui),
            WelcomePage::Ready => self.ready_page(ui, actions),
        }
    }

    fn intro_page(&self, ui: &mut Ui, actions: &mut Vec<Action>) {
        page_title(
            ui,
            &rust_i18n::t!("welcome.intro_title", app = &self.app_name),
            &t("welcome.intro_body"),
        );
        FeatureRow::new(
            Icon::Database,
            &t("welcome.local_title"),
            &t("welcome.local_body"),
        )
        .show(ui);
        FeatureRow::new(
            Icon::Search,
            &t("welcome.find_title"),
            &t("welcome.find_body"),
        )
        .show(ui);
        FeatureRow::new(
            Icon::Sync,
            &t("welcome.sync_title"),
            &t("welcome.sync_body"),
        )
        .show(ui);
        ui.separator();

        labelled_row(ui, &t("welcome.appearance"), |ui| {
            let mut preference = ThemePreference::from_setting(&self.settings.theme);
            let options = [
                (ThemePreference::System, t("settings.system")),
                (ThemePreference::Light, t("settings.light")),
                (ThemePreference::Dark, t("settings.dark")),
            ];
            if Segmented::new("welcome-theme", &mut preference, &options)
                .show(ui)
                .changed()
            {
                actions.push(Action::SetTheme(preference));
            }
        });
        labelled_row(ui, &t("welcome.size"), |ui| {
            let mut zoom = self.settings.zoom;
            if Stepper::new(
                "welcome-size",
                &mut zoom,
                zoom::RANGE,
                zoom::STEP,
                &t("settings.size_smaller"),
                &t("settings.size_larger"),
            )
            .format(StepperFormat::Percent)
            .show(ui)
            .changed()
            {
                actions.push(Action::SetZoom(zoom));
            }
        });
    }

    fn ready_page(&self, ui: &mut Ui, actions: &mut Vec<Action>) {
        let palette = theme::palette(ui.ctx());
        page_title(ui, &t("welcome.ready_title"), &t("welcome.ready_body"));
        inset(ui, |ui| {
            ui.label(RichText::new(t("welcome.data_title")).font(theme::semibold(14.0)));
            ui.label(RichText::new(t("welcome.data_body")).color(palette.text_secondary));
            ui.label(
                RichText::new(self.paths.data.display().to_string())
                    .monospace()
                    .color(palette.text),
            );
            if ui
                .add(Button::secondary(&t("welcome.open_folder")).icon(Icon::Database))
                .clicked()
                && let Err(error) = system::open_folder(&self.paths.data)
            {
                tracing::warn!(%error, "could not open the data folder");
            }
        });
        inset(ui, |ui| {
            ui.label(RichText::new(t("welcome.first_title")).font(theme::semibold(14.0)));
            ui.label(RichText::new(t("welcome.first_body")).color(palette.text_secondary));
            if ui
                .add(Button::secondary(&t("library.new")).icon(Icon::Plus))
                .clicked()
            {
                actions.push(Action::CloseWelcome(WelcomeOutcome::Finished));
                actions.push(Action::NewRecord);
            }
        });
        ui.label(RichText::new(t("welcome.reopen")).color(palette.text_secondary));
    }
}

fn shortcuts_page(ui: &mut Ui) {
    let palette = theme::palette(ui.ctx());
    let ctx = ui.ctx().clone();
    page_title(ui, &t("welcome.keys_title"), &t("welcome.keys_body"));

    // The one shortcut worth remembering, highlighted.
    callout(ui, |ui| {
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = space::M;
            ui.add(Kbd(&ctx.format_shortcut(&shortcuts::PALETTE)));
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = space::XS;
                ui.label(RichText::new(t("welcome.palette_title")).font(theme::semibold(14.0)));
                ui.label(RichText::new(t("welcome.palette_body")).color(palette.text_secondary));
            });
        });
    });

    let zoom_keys = format!(
        "{} / {}",
        ctx.format_shortcut(&egui::gui_zoom::kb_shortcuts::ZOOM_IN),
        ctx.format_shortcut(&egui::gui_zoom::kb_shortcuts::ZOOM_OUT)
    );
    let rows = [
        (
            ctx.format_shortcut(&shortcuts::NEW_RECORD),
            t("welcome.key_new"),
        ),
        (
            ctx.format_shortcut(&shortcuts::SIDEBAR),
            t("welcome.key_sidebar"),
        ),
        (zoom_keys, t("welcome.key_zoom")),
        ("Esc".to_owned(), t("welcome.key_escape")),
    ];
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = space::S;
        for (keys, label) in &rows {
            shortcut_row(ui, keys, label);
        }
    });
}

/// Page heading plus a wrapped paragraph under it.
fn page_title(ui: &mut Ui, title: &str, body: &str) {
    let palette = theme::palette(ui.ctx());
    ui.vertical(|ui| {
        ui.spacing_mut().item_spacing.y = space::S;
        ui.label(RichText::new(title).text_style(theme::title_style()));
        ui.label(RichText::new(body).color(palette.text_secondary));
    });
}

/// A fixed-width label column and a control, like the Appearance/Size rows.
fn labelled_row(ui: &mut Ui, label: &str, control: impl FnOnce(&mut Ui)) {
    let palette = theme::palette(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_sized(
            [LABEL_WIDTH, theme::size::CONTROL],
            egui::Label::new(RichText::new(label).color(palette.text_secondary)),
        );
        control(ui);
    });
}
