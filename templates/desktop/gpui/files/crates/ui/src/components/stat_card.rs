//! A metric card: icon, label, and a large value.

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::{ActiveTheme, Icon, IconName, Sizable as _};
use gpui_kit::*;

/// One number on the dashboard.
#[derive(IntoElement)]
pub struct StatCard {
    icon: IconName,
    label: SharedString,
    value: SharedString,
}

impl StatCard {
    #[must_use]
    pub fn new(
        icon: IconName,
        label: impl Into<SharedString>,
        value: impl Into<SharedString>,
    ) -> Self {
        Self {
            icon,
            label: label.into(),
            value: value.into(),
        }
    }
}

impl RenderOnce for StatCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .flex_1()
            .min_w_40()
            .gap_1()
            .p_4()
            .rounded(cx.theme().radius_lg)
            .bg(cx.theme().group_box)
            .border_1()
            .border_color(cx.theme().border)
            .child(
                h_flex()
                    .gap_2()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(Icon::new(self.icon).small())
                    .child(self.label),
            )
            .child(div().text_2xl().font_semibold().child(self.value))
    }
}
