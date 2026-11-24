//! A − value + control for small numeric settings such as interface size.
//!
//! GPUI Kit's `Stepper` is a step-progress indicator, and its number input is
//! a text field; this is the compact pill used in Settings and the welcome tour.

use gpui_kit::base::h_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

/// Decrease button, the formatted value, increase button. The caller builds
/// the two buttons (ghost, small, `IconName::Minus` / `IconName::Plus`, with a
/// tooltip naming what they do) so it owns their listeners.
#[derive(IntoElement)]
pub struct ValueStepper {
    value: SharedString,
    decrease: AnyElement,
    increase: AnyElement,
}

impl ValueStepper {
    #[must_use]
    pub fn new(
        value: impl Into<SharedString>,
        decrease: impl IntoElement,
        increase: impl IntoElement,
    ) -> Self {
        Self {
            value: value.into(),
            decrease: decrease.into_any_element(),
            increase: increase.into_any_element(),
        }
    }
}

impl RenderOnce for ValueStepper {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        h_flex()
            .p_0p5()
            .gap_0p5()
            .items_center()
            .rounded(theme.radius)
            .border_1()
            .border_color(theme.border)
            .bg(theme.muted)
            .child(self.decrease)
            .child(
                div()
                    .w(rems(3.5))
                    .flex()
                    .justify_center()
                    .text_sm()
                    .child(self.value),
            )
            .child(self.increase)
    }
}
