//! Empty and zero-result state.

use gpui_kit::base::{StyledExt, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

#[derive(IntoElement)]
pub struct EmptyState {
    title: SharedString,
    body: Option<SharedString>,
}

impl EmptyState {
    #[must_use]
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            body: None,
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<SharedString>) -> Self {
        self.body = Some(body.into());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_2()
            .p_6()
            .rounded_lg()
            .bg(cx.theme().secondary)
            .border_1()
            .border_color(cx.theme().border)
            .child(div().font_semibold().child(self.title))
            .when_some(self.body, |this, body| {
                this.child(div().text_color(cx.theme().muted_foreground).child(body))
            })
    }
}
