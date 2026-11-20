//! Compact metric card.

use gpui_kit::base::{StyledExt, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

#[derive(IntoElement)]
pub struct StatCard {
    label: SharedString,
    value: SharedString,
}

impl StatCard {
    #[must_use]
    pub fn new(label: impl Into<SharedString>, value: impl Into<SharedString>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
        }
    }
}

impl RenderOnce for StatCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w(px(220.))
            .p_5()
            .gap_1()
            .rounded_lg()
            .bg(cx.theme().secondary)
            .border_1()
            .border_color(cx.theme().border)
            .child(
                div()
                    .text_sm()
                    .text_color(cx.theme().muted_foreground)
                    .child(self.label),
            )
            .child(div().text_3xl().font_semibold().child(self.value))
    }
}
