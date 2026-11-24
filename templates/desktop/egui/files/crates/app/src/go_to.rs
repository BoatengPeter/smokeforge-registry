//! "Go to…": what the search window lists for a query, and the selection.
//!
//! This module is state only; `shell.rs` draws the window with
//! [`crate::components::PickerWindow`]. The list has three groups, in order:
//! pages, records matching the query (only while typing), and "+" actions.
//! Arrow keys move through every row of every group as one list.
//!
//! To add an action: add a [`Command`] variant (with a `command.*` string) and
//! list it in [`Command::ALL`]; it shows up here automatically.

use crate::Pane;
use crate::commands::Command;

/// Records shown at most, so the list stays short while typing.
pub const MAX_RECORDS: usize = 6;

/// One row of the "Go to…" list.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GoToItem {
    Page(Pane),
    Record { id: uuid::Uuid, title: String },
    Action(Command),
}

impl GoToItem {
    /// Stable identity for widget IDs.
    #[must_use]
    pub fn key(&self) -> String {
        match self {
            Self::Page(pane) => format!("page-{pane:?}"),
            Self::Record { id, .. } => format!("record-{id}"),
            Self::Action(command) => format!("action-{command:?}"),
        }
    }
}

/// The rows for `query`, grouped pages → records → actions.
///
/// `pages` are the panes the app shows; `records` are already filtered by the
/// database search; `page_label` and `action_label` give each row its
/// translated text, which is what the query matches.
#[must_use]
pub fn items(
    query: &str,
    pages: &[Pane],
    records: Vec<(uuid::Uuid, String)>,
    page_label: impl Fn(Pane) -> String,
    action_label: impl Fn(Command) -> String,
) -> Vec<GoToItem> {
    let query = query.trim().to_lowercase();
    let matches = |text: &str| query.is_empty() || text.to_lowercase().contains(&query);

    let pages = pages
        .iter()
        .copied()
        .filter(|pane| matches(&page_label(*pane)))
        .map(GoToItem::Page);
    let records = records
        .into_iter()
        .filter(|_| !query.is_empty())
        .take(MAX_RECORDS)
        .map(|(id, title)| GoToItem::Record { id, title });
    let actions = Command::ALL
        .into_iter()
        .filter(|command| !matches!(command, Command::Navigate(_)))
        .filter(|command| matches(&action_label(*command)))
        .map(GoToItem::Action);
    pages.chain(records).chain(actions).collect()
}

/// The selection after pressing ↓ (`forward`) or ↑ in a list of `len` rows;
/// wraps around at both ends.
#[must_use]
pub const fn step_selection(selected: usize, len: usize, forward: bool) -> usize {
    if len == 0 {
        0
    } else if forward {
        (selected + 1) % len
    } else {
        (selected + len - 1) % len
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn page_label(pane: Pane) -> String {
        format!("{pane:?}")
    }

    fn action_label(command: Command) -> String {
        format!("{command:?}")
    }

    #[test]
    fn empty_query_lists_pages_then_actions_and_no_records() {
        let record = (uuid::Uuid::new_v4(), "Roadmap".to_owned());
        let rows = items(
            "",
            &[Pane::Dashboard, Pane::Library],
            vec![record],
            page_label,
            action_label,
        );
        assert_eq!(rows[0], GoToItem::Page(Pane::Dashboard));
        assert_eq!(rows[1], GoToItem::Page(Pane::Library));
        assert!(matches!(rows[2], GoToItem::Action(_)));
        assert!(
            !rows
                .iter()
                .any(|row| matches!(row, GoToItem::Record { .. }))
        );
    }

    #[test]
    fn query_filters_pages_and_actions_and_keeps_records() {
        let id = uuid::Uuid::new_v4();
        let rows = items(
            "LIBR",
            &[Pane::Dashboard, Pane::Library],
            vec![(id, "Library notes".to_owned())],
            page_label,
            action_label,
        );
        assert_eq!(
            rows,
            vec![
                GoToItem::Page(Pane::Library),
                GoToItem::Record {
                    id,
                    title: "Library notes".to_owned()
                },
            ]
        );
    }

    #[test]
    fn records_are_capped() {
        let records = (0..20)
            .map(|index| (uuid::Uuid::new_v4(), format!("Note {index}")))
            .collect();
        let rows = items("note", &[], records, page_label, action_label);
        assert_eq!(rows.len(), MAX_RECORDS);
    }

    #[test]
    fn selection_wraps() {
        assert_eq!(step_selection(2, 3, true), 0);
        assert_eq!(step_selection(0, 3, false), 2);
        assert_eq!(step_selection(0, 0, true), 0);
    }
}
