//! A titled card that groups one section of a page.

use gpui_kit::base::{StyledExt, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

/// A grouped region on the workspace canvas: surface fill, hairline border,
/// large radius. Do not nest one inside another.
#[derive(IntoElement)]
pub struct SectionCard {
    title: SharedString,
    children: Vec<AnyElement>,
}

impl SectionCard {
    #[must_use]
    pub fn new(title: impl Into<SharedString>) -> Self {
        Self {
            title: title.into(),
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }

    #[must_use]
    pub fn children(mut self, children: impl IntoIterator<Item = impl IntoElement>) -> Self {
        self.children
            .extend(children.into_iter().map(IntoElement::into_any_element));
        self
    }
}

impl RenderOnce for SectionCard {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        v_flex()
            .w_full()
            .gap_3()
            .p_4()
            .rounded(cx.theme().radius_lg)
            .bg(cx.theme().group_box)
            .border_1()
            .border_color(cx.theme().border)
            .child(div().text_sm().font_semibold().child(self.title))
            .children(self.children)
    }
}
