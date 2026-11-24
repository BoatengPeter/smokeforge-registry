//! Shared UI primitives.
//!
//! Every component here is presentational: it takes prepared values, draws with
//! [`crate::theme`] tokens and [`crate::icons`], and returns a [`egui::Response`]
//! (or an inner value) for the caller to act on. None of them read the database,
//! call the API, or dispatch actions.

mod badge;
mod button;
mod confirm_dialog;
mod empty_state;
mod field;
mod icon_button;
mod kbd;
mod list_row;
mod nav_item;
mod notice;
mod page_header;
mod segmented;
mod stat;
mod status_page;
mod surface;
mod switch;

pub use badge::{Badge, Tone};
pub use button::{Button, ButtonKind};
pub use confirm_dialog::{ConfirmChoice, ConfirmDialog};
pub use empty_state::EmptyState;
pub use field::{password_field, search_field, text_area, text_field};
pub use icon_button::{HoverTone, IconButton};
pub use kbd::Kbd;
pub use list_row::ListRow;
pub use nav_item::{NavDensity, NavItem};
pub use notice::Notice;
pub use page_header::PageHeader;
pub use segmented::Segmented;
pub use stat::StatCard;
pub use status_page::{StatusAction, StatusPage};
pub use surface::{card, section};
pub use switch::Switch;

use std::sync::Arc;

use egui::text::{LayoutJob, TextWrapping};
use egui::{Color32, FontId, Galley, Rect, Response, Stroke, StrokeKind, Ui};

/// Lays out `text` on one line, cut with an ellipsis at `max_width`.
pub(crate) fn single_line(
    ui: &Ui,
    text: &str,
    font: FontId,
    color: Color32,
    max_width: f32,
) -> Arc<Galley> {
    let mut job = LayoutJob::simple_singleline(text.to_owned(), font, color);
    job.wrap = TextWrapping::truncate_at_width(max_width.max(0.0));
    ui.painter().layout_job(job)
}

/// Draws the keyboard focus ring around `rect` when `response` has focus.
pub(crate) fn focus_ring(ui: &Ui, response: &Response, rect: Rect, radius: u8) {
    if response.has_focus() {
        let palette = crate::theme::palette(ui.ctx());
        ui.painter().rect_stroke(
            rect.expand(2.0),
            radius.saturating_add(2),
            Stroke::new(2.0, palette.focus),
            StrokeKind::Outside,
        );
    }
}
