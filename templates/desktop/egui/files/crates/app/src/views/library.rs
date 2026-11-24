//! Library: searchable record list and the record editor.

use egui::{RichText, Ui};

use super::status_tone;
use crate::components::{
    Button, EmptyState, ListRow, PageHeader, StatusAction, StatusPage, Tone, card, search_field,
    text_area, text_field,
};
use crate::icons::Icon;
use crate::theme::{self, space};
use crate::{Action, DesktopApp, Editor, t};

const ROW_HEIGHT: f32 = 52.0;
const LIST_SHARE: f32 = 0.42;
const LIST_MIN_WIDTH: f32 = 260.0;
const LIST_HEIGHT: f32 = 460.0;

impl DesktopApp {
    pub(crate) fn library_view(&mut self, ui: &mut Ui) {
        let mut action = None;
        PageHeader::new(&t("library.title"))
            .subtitle(&t("library.subtitle"))
            .show(ui, |ui| {
                if ui
                    .add(Button::primary(&t("library.new")).icon(Icon::Plus))
                    .clicked()
                {
                    action = Some(Action::NewRecord);
                }
            });

        let list_width = (ui.available_width() * LIST_SHARE).max(LIST_MIN_WIDTH);
        ui.horizontal_top(|ui| {
            ui.spacing_mut().item_spacing.x = space::L;
            ui.vertical(|ui| {
                ui.set_width(list_width);
                if let Some(next) = self.record_list(ui) {
                    action = Some(next);
                }
            });
            ui.vertical(|ui| {
                ui.set_width(ui.available_width());
                if let Some(next) = self.record_editor(ui) {
                    action = Some(next);
                }
            });
        });

        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    fn record_list(&mut self, ui: &mut Ui) -> Option<Action> {
        let mut action = None;
        let items = match self.records(&self.search) {
            Ok(items) => items,
            Err(error) => {
                card(ui, |ui| {
                    action = super::database_error(ui, &error, &self.paths.logs);
                });
                return action;
            }
        };
        let selected = self.editor.record();
        card(ui, |ui| {
            ui.set_width(ui.available_width());
            let mut search = self.search.clone();
            search_field(ui, &t("library.search"), &mut search, &t("library.search"));
            if search != self.search {
                action = Some(Action::SearchChanged(search));
            }
            ui.add_space(space::S);
            if items.is_empty() {
                EmptyState::new(
                    Icon::Search,
                    &t("library.no_matches"),
                    &t("library.no_matches_body"),
                )
                .show(ui);
                return;
            }
            ui.spacing_mut().item_spacing.y = 2.0;
            egui::ScrollArea::vertical()
                .id_salt("library-records")
                .max_height(LIST_HEIGHT)
                .auto_shrink([false, true])
                .show_rows(ui, ROW_HEIGHT + 2.0, items.len(), |ui, range| {
                    for item in &items[range] {
                        let tags = if item.tags.is_empty() {
                            t("library.no_tags")
                        } else {
                            item.tags.join(", ")
                        };
                        let row = ui.push_id(("record", item.id), |ui| {
                            ui.add(
                                ListRow::new(&item.title)
                                    .subtitle(&tags)
                                    .badge(item.status.label(), status_tone(item.status))
                                    .selected(selected == Some(item.id)),
                            )
                        });
                        if row.inner.clicked() {
                            action = Some(Action::Select(Some(item.id)));
                        }
                    }
                });
        });
        action
    }

    fn record_editor(&mut self, ui: &mut Ui) -> Option<Action> {
        let palette = theme::palette(ui.ctx());
        let mut action = None;
        card(ui, |ui| {
            ui.set_width(ui.available_width());
            if let Editor::Missing(_) = self.editor {
                let page = StatusPage::new(
                    Icon::Search,
                    Tone::Neutral,
                    &t("state.missing_title"),
                    &t("state.missing_body"),
                )
                .primary(&t("state.back_to_records"))
                .show(ui);
                if page == Some(StatusAction::Primary) {
                    action = Some(Action::Select(None));
                }
                return;
            }
            if self.editor == Editor::Closed {
                let empty = EmptyState::new(
                    Icon::Library,
                    &t("library.select"),
                    &t("library.select_body"),
                )
                .action(&t("library.new"))
                .show(ui);
                if empty.is_some_and(|response| response.clicked()) {
                    action = Some(Action::NewRecord);
                }
                return;
            }

            let heading = match self.editor {
                Editor::New => t("library.new_title"),
                _ if self.draft.title.trim().is_empty() => t("library.untitled"),
                _ => self.draft.title.clone(),
            };
            ui.label(RichText::new(heading).heading());
            ui.add_space(space::M);
            if let Some(error) = &self.form_error {
                ui.label(RichText::new(error).color(palette.danger));
                ui.add_space(space::S);
            }
            text_field(ui, &t("library.title_field"), &mut self.draft.title, "");
            ui.add_space(space::S);
            text_area(
                ui,
                &t("library.description_field"),
                &mut self.draft.description,
                4,
            );
            ui.add_space(space::S);
            let mut tags = self.draft.tags.join(", ");
            if text_field(
                ui,
                &t("library.tags_field"),
                &mut tags,
                &t("library.tags_hint"),
            )
            .changed()
            {
                self.draft.tags = tags
                    .split(',')
                    .map(str::trim)
                    .filter(|tag| !tag.is_empty())
                    .map(str::to_owned)
                    .collect();
            }
            ui.add_space(space::L);
            let exists = self.editor.record().is_some();
            let (delete, save) = egui::Sides::new().show(
                ui,
                |ui| {
                    exists
                        && ui
                            .add(Button::danger(&t("library.delete")).icon(Icon::Trash))
                            .clicked()
                },
                |ui| {
                    ui.add(Button::primary(&t("library.save")).icon(Icon::Check))
                        .clicked()
                },
            );
            if delete {
                action = Some(Action::DeleteRecord);
            } else if save {
                action = Some(Action::SaveDraft);
            }
        });
        action
    }
}
