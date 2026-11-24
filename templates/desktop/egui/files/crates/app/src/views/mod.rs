//! One module per page. Each view reads prepared state from [`DesktopApp`],
//! draws with [`crate::components`], and dispatches at most one [`Action`]
//! after it has finished drawing.
//!
//! [`DesktopApp`]: crate::DesktopApp
//! [`Action`]: crate::Action

mod about;
mod account;
mod activity;
mod dashboard;
mod library;
mod settings;

use egui::{RichText, Ui};

use crate::components::{StatusAction, StatusPage, Tone};
use crate::domain::LibraryStatus;
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{Action, system, t};

/// The badge tone for a record status. Only "Active" carries meaning worth a
/// colour; drafts and archived records stay neutral so the list stays calm.
pub(crate) const fn status_tone(status: LibraryStatus) -> Tone {
    match status {
        LibraryStatus::Active => Tone::Success,
        LibraryStatus::Draft | LibraryStatus::Archived => Tone::Neutral,
    }
}

/// A settings-style row: label and help text on the left, a control on the right.
pub(crate) fn setting_row(ui: &mut Ui, label: &str, help: &str, control: impl FnOnce(&mut Ui)) {
    let palette = theme::palette(ui.ctx());
    egui::Sides::new().height(36.0).show(
        ui,
        |ui| {
            ui.vertical(|ui| {
                ui.spacing_mut().item_spacing.y = 2.0;
                ui.label(label);
                ui.label(RichText::new(help).small().color(palette.text_secondary));
            });
        },
        control,
    );
    ui.add_space(space::XS);
}

/// The "local database can't be read" state, shared by every page that lists
/// records. Returns the action to dispatch.
pub(crate) fn database_error(ui: &mut Ui, error: &str, logs: &std::path::Path) -> Option<Action> {
    match StatusPage::new(
        Icon::Database,
        Tone::Danger,
        &t("state.database_title"),
        &t("state.database_body"),
    )
    .details(error)
    .primary(&t("state.retry"))
    .secondary(&t("state.open_logs"))
    .show(ui)
    {
        Some(StatusAction::Primary) => Some(Action::ReopenDatabase),
        Some(StatusAction::Secondary) => {
            let _ = system::open_folder(logs);
            None
        }
        None => None,
    }
}

/// A key–value line for details panels.
pub(crate) fn detail_row(ui: &mut Ui, key: &str, value: &str) {
    let palette = theme::palette(ui.ctx());
    ui.horizontal(|ui| {
        ui.add_sized(
            [140.0, 20.0],
            egui::Label::new(RichText::new(key).color(palette.text_secondary)),
        );
        ui.add(egui::Label::new(RichText::new(value).monospace()).selectable(true));
    });
}
