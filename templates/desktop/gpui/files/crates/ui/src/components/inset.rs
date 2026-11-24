//! Boxes inside dialogs and tours: a quiet [`Inset::new`] for secondary details
//! and an accent [`Inset::callout`] for the one thing on a page worth remembering.
//! On the workspace canvas use `SectionCard` instead.

use gpui_kit::base::v_flex;
use gpui_kit::component::ActiveTheme;
use gpui_kit::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Emphasis {
    Quiet,
    Accent,
}

/// A muted, outlined box, or an accent callout.
#[derive(IntoElement)]
pub struct Inset {
    emphasis: Emphasis,
    children: Vec<AnyElement>,
}

impl Inset {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            emphasis: Emphasis::Quiet,
            children: Vec::new(),
        }
    }

    /// The accent treatment. Use at most one per page.
    #[must_use]
    pub const fn callout() -> Self {
        Self {
            emphasis: Emphasis::Accent,
            children: Vec::new(),
        }
    }

    #[must_use]
    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.children.push(child.into_any_element());
        self
    }
}

impl Default for Inset {
    fn default() -> Self {
        Self::new()
    }
}

impl RenderOnce for Inset {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let (fill, border) = match self.emphasis {
            Emphasis::Quiet => (theme.muted, theme.border),
            Emphasis::Accent => (theme.list_active, theme.primary),
        };
        v_flex()
            .w_full()
            .gap_2()
            .p_3p5()
            .rounded(theme.radius_lg)
            .bg(fill)
            .border_1()
            .border_color(border)
            .children(self.children)
    }
}
