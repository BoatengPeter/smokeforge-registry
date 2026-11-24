//! Home: counts, status chart, and recent records.

use egui::{RichText, Ui};

use super::status_tone;
use crate::components::{Badge, Button, EmptyState, ListRow, PageHeader, StatCard, section};
use crate::domain::LibraryStatus;
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{Action, DesktopApp, Pane, t};

const RECENT_LIMIT: usize = 5;
/// Height of a status breakdown bar.
const BAR_HEIGHT: f32 = 6.0;

impl DesktopApp {
    #[allow(clippy::too_many_lines)]
    pub(crate) fn dashboard_view(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let mut action = None;
        let items = match self.records("") {
            Ok(items) => items,
            Err(error) => {
                PageHeader::new(&t("dashboard.title"))
                    .subtitle(&t("dashboard.subtitle"))
                    .show(ui, |_| {});
                if let Some(action) = super::database_error(ui, &error, &self.paths.logs) {
                    self.dispatch(action);
                }
                return;
            }
        };
        let count =
            |status: LibraryStatus| items.iter().filter(|item| item.status == status).count();
        let (drafts, active, archived) = (
            count(LibraryStatus::Draft),
            count(LibraryStatus::Active),
            count(LibraryStatus::Archived),
        );

        PageHeader::new(&t("dashboard.title"))
            .subtitle(&t("dashboard.subtitle"))
            .show(ui, |ui| {
                if ui
                    .add(Button::primary(&t("library.new")).icon(Icon::Plus))
                    .clicked()
                {
                    action = Some(Action::NewRecord);
                }
            });

        let stats = [
            (Icon::Library, t("dashboard.records"), items.len()),
            (Icon::Check, t("dashboard.active"), active),
            (Icon::Terminal, t("dashboard.drafts"), drafts),
            (Icon::Database, t("dashboard.archived"), archived),
        ];
        ui.columns(stats.len(), |columns| {
            for (column, (icon, label, value)) in columns.iter_mut().zip(&stats) {
                StatCard::new(*icon, label, &value.to_string()).show(column);
            }
        });
        ui.add_space(space::L);

        if items.is_empty() {
            section(ui, &t("dashboard.recent"), |ui| {
                let empty = EmptyState::new(
                    Icon::Library,
                    &t("dashboard.empty_title"),
                    &t("dashboard.empty_body"),
                )
                .action(&t("library.new"))
                .show(ui);
                if empty.is_some_and(|response| response.clicked()) {
                    action = Some(Action::NewRecord);
                }
            });
        } else {
            ui.columns(2, |columns| {
                section(&mut columns[0], &t("dashboard.status"), |ui| {
                    let rows = [
                        (LibraryStatus::Active, active, palette.chart[2]),
                        (LibraryStatus::Draft, drafts, palette.chart[1]),
                        (LibraryStatus::Archived, archived, palette.chart[0]),
                    ];
                    status_breakdown(ui, &rows, items.len());
                });
                section(&mut columns[1], &t("dashboard.recent"), |ui| {
                    ui.spacing_mut().item_spacing.y = 2.0;
                    for item in items.iter().take(RECENT_LIMIT) {
                        let tags = if item.tags.is_empty() {
                            t("library.no_tags")
                        } else {
                            item.tags.join(", ")
                        };
                        let row = ui.push_id(("recent", item.id), |ui| {
                            ui.add(
                                ListRow::new(&item.title)
                                    .subtitle(&tags)
                                    .badge(item.status.label(), status_tone(item.status)),
                            )
                        });
                        if row.inner.clicked() {
                            action = Some(Action::Select(Some(item.id)));
                        }
                    }
                });
            });
        }

        if let Some(action) = action {
            if matches!(action, Action::Select(_)) {
                self.dispatch(Action::Navigate(Pane::Library));
            }
            self.dispatch(action);
        }
    }
}

/// One labelled row per status: name, count, share of the total, and a thin
/// proportional bar. Reads better than a chart for a handful of categories.
fn status_breakdown(ui: &mut Ui, rows: &[(LibraryStatus, usize, egui::Color32)], total: usize) {
    let palette = theme::palette(ui.ctx());
    for (status, count, color) in rows {
        // Counts are small; precision loss is irrelevant for a display ratio.
        #[expect(clippy::cast_precision_loss, reason = "display-only ratio")]
        let share = if total == 0 {
            0.0
        } else {
            *count as f32 / total as f32
        };
        egui::Sides::new().show(
            ui,
            |ui| {
                ui.add(Badge::new(status.label(), status_tone(*status)));
            },
            |ui| {
                ui.label(
                    RichText::new(format!("{count} · {:.0}%", share * 100.0))
                        .color(palette.text_secondary),
                );
            },
        );
        let (track, _) = ui.allocate_exact_size(
            egui::vec2(ui.available_width(), BAR_HEIGHT),
            egui::Sense::hover(),
        );
        ui.painter()
            .rect_filled(track, BAR_HEIGHT / 2.0, palette.field);
        if share > 0.0 {
            let mut fill = track;
            fill.set_width((track.width() * share).max(BAR_HEIGHT));
            ui.painter().rect_filled(fill, BAR_HEIGHT / 2.0, *color);
        }
        ui.add_space(space::M);
    }
}
