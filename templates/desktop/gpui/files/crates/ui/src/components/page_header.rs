//! Page title and supporting text.

use gpui_kit::base::{StyledExt, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

pub struct PageHeader {
    title: SharedString,
    subtitle: Option<SharedString>,
}

impl PageHeader {
    #[must_use]
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            subtitle: None,
        }
    }

    #[must_use]
    pub fn subtitle(mut self, subtitle: impl Into<SharedString>) -> Self {
        self.subtitle = Some(subtitle.into());
        self
    }
}

impl RenderOnce for PageHeader {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_1()
            .child(div().text_2xl().font_semibold().child(self.title))
            .when_some(self.subtitle, |this, subtitle| {
                this.child(
                    div()
                        .text_color(cx.theme().muted_foreground)
                        .child(subtitle),
                )
            })
    }
}
