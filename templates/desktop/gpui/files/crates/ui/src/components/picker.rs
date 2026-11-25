//! The frame of a search-and-pick window near the top of the app ("Go to…").
//!
//! It holds a title row (title and an "Esc" control that closes) above a body,
//! normally a GPUI Kit `Command` with `bordered(false)`. It floats 2.5rem from
//! the top of the window, centred, 40rem wide (Herdr's proportions). The
//! caller builds the Esc control and the body so it owns their listeners.

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

#[derive(IntoElement)]
pub struct PickerFrame {
    title: SharedString,
    escape: Option<AnyElement>,
    body: Option<AnyElement>,
}

impl PickerFrame {
    #[must_use]
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            escape: None,
            body: None,
        }
    }

    /// The "Esc" control in the title row; clicking it must close the picker.
    #[must_use]
    pub fn escape(mut self, escape: impl IntoElement) -> Self {
        self.escape = Some(escape.into_any_element());
        self
    }

    #[must_use]
    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = Some(body.into_any_element());
        self
    }
}

impl RenderOnce for PickerFrame {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        h_flex()
            .absolute()
            .top(rems(2.5))
            .left_0()
            .right_0()
            .justify_center()
            .child(
                v_flex()
                    .occlude()
                    .w(rems(40.))
                    .max_w(relative(0.94))
                    .p(rems(1.125))
                    .gap_2p5()
                    .bg(theme.popover)
                    .text_color(theme.foreground)
                    .border_1()
                    .border_color(theme.border)
                    .rounded(theme.radius_lg)
                    .shadow_xl()
                    .child(
                        h_flex()
                            .items_center()
                            .child(div().flex_1().text_base().font_semibold().child(self.title))
                            .children(self.escape),
                    )
                    .children(self.body),
            )
    }
}
