//! An icon tile beside a short title and explanation, for tours and intros.

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::{ActiveTheme, Icon, IconName};
use gpui_kit::*;

/// "What this app does" row: icon tile, semibold title, wrapped body.
#[derive(IntoElement)]
pub struct FeatureRow {
    icon: IconName,
    title: SharedString,
    body: SharedString,
}

impl FeatureRow {
    #[must_use]
    pub fn new(
        icon: IconName,
        title: impl Into<SharedString>,
        body: impl Into<SharedString>,
    ) -> Self {
        Self {
            icon,
            title: title.into(),
            body: body.into(),
        }
    }
}

impl RenderOnce for FeatureRow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        h_flex()
            .w_full()
            .gap_3()
            .items_start()
            .child(
                div()
                    .flex_none()
                    .size_8()
                    .rounded(theme.radius_lg)
                    .bg(theme.muted)
                    .border_1()
                    .border_color(theme.border)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(self.icon).text_color(theme.primary)),
            )
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .gap_1()
                    .child(div().text_sm().font_semibold().child(self.title))
                    .child(
                        div()
                            .text_sm()
                            .text_color(theme.muted_foreground)
                            .child(self.body),
                    ),
            )
    }
}
