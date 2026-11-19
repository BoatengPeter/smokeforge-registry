//! GPUI application shell.

#![allow(
    clippy::wildcard_imports,
    reason = "GPUI Kit exports the framework prelude."
)]

use gpui_kit::*;

pub struct AppView;

impl AppView {
    #[must_use]
    pub fn new() -> Self {
        Self
    }
}

impl Default for AppView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().p_8().child("Smokeforge GPUI")
    }
}
