//! Full-area states: errors, not found, offline, permission denied.
//!
//! Use a status page when the area cannot show its normal content at all. For
//! a problem that leaves the page usable, use GPUI Kit's `Alert` instead.

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::{ActiveTheme, Icon, IconName, Sizable as _};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// How serious a status page is. Sets the icon colour only; the title and
/// icon always state the problem too.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum StatusTone {
    #[default]
    Neutral,
    Danger,
    Warning,
}

/// A centred state with an icon, title, explanation, optional technical
/// details, and actions.
#[derive(IntoElement)]
pub struct StatusPage {
    icon: IconName,
    tone: StatusTone,
    title: SharedString,
    body: SharedString,
    details: Option<SharedString>,
    actions: Vec<AnyElement>,
}

impl StatusPage {
    #[must_use]
    pub fn new(
        icon: IconName,
        tone: StatusTone,
        title: impl Into<SharedString>,
        body: impl Into<SharedString>,
    ) -> Self {
        Self {
            icon,
            tone,
            title: title.into(),
            body: body.into(),
            details: None,
            actions: Vec::new(),
        }
    }

    /// Selectable technical text (an error chain, a path) for bug reports.
    #[must_use]
    pub fn details(mut self, details: impl Into<SharedString>) -> Self {
        self.details = Some(details.into());
        self
    }

    /// Adds a recovery action. Pass the primary action last.
    #[must_use]
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.actions.push(action.into_any_element());
        self
    }
}

impl RenderOnce for StatusPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let icon_color = match self.tone {
            StatusTone::Neutral => theme.muted_foreground,
            StatusTone::Danger => theme.danger,
            StatusTone::Warning => theme.warning,
        };
        v_flex()
            .w_full()
            .items_center()
            .gap_2()
            .py_12()
            .child(
                div()
                    .size_12()
                    .rounded_full()
                    .bg(theme.muted)
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(Icon::new(self.icon).large().text_color(icon_color)),
            )
            .child(div().pt_2().text_xl().font_semibold().child(self.title))
            .child(
                div()
                    .max_w_96()
                    .text_sm()
                    .text_center()
                    .text_color(theme.muted_foreground)
                    .child(self.body),
            )
            .when_some(self.details, |this, details| {
                this.child(
                    div()
                        .mt_2()
                        .max_w_112()
                        .w_full()
                        .p_3()
                        .rounded(theme.radius)
                        .bg(theme.muted)
                        .border_1()
                        .border_color(theme.border)
                        .text_xs()
                        .font_family(theme.mono_font_family.clone())
                        .child(details),
                )
            })
            .when(!self.actions.is_empty(), |this| {
                this.child(h_flex().pt_3().gap_2().children(self.actions))
            })
    }
}
