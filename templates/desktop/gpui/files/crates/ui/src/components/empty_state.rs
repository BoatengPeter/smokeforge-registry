//! Empty, zero-result, and nothing-selected states that name the next action.

use gpui_kit::component::empty::{
    Empty, EmptyContent, EmptyDescription, EmptyHeader, EmptyMedia, EmptyMediaVariant, EmptyTitle,
};
use gpui_kit::component::{Icon, IconName};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// A thin wrapper over GPUI Kit's `Empty` with the app's standard layout:
/// icon, title, one explanatory sentence, and an optional action.
#[derive(IntoElement)]
pub struct EmptyState {
    icon: IconName,
    title: SharedString,
    body: Option<SharedString>,
    action: Option<AnyElement>,
}

impl EmptyState {
    #[must_use]
    pub fn new(icon: IconName, title: impl Into<SharedString>) -> Self {
        Self {
            icon,
            title: title.into(),
            body: None,
            action: None,
        }
    }

    #[must_use]
    pub fn body(mut self, body: impl Into<SharedString>) -> Self {
        self.body = Some(body.into());
        self
    }

    /// Adds the next useful action, usually a primary button.
    #[must_use]
    pub fn action(mut self, action: impl IntoElement) -> Self {
        self.action = Some(action.into_any_element());
        self
    }
}

impl RenderOnce for EmptyState {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        let header = EmptyHeader::new()
            .media(
                EmptyMedia::new()
                    .with_variant(EmptyMediaVariant::Icon)
                    .child(Icon::new(self.icon)),
            )
            .title(EmptyTitle::new().child(self.title))
            .when_some(self.body, |header, body| {
                header.description(EmptyDescription::new().child(body))
            });
        Empty::new()
            .header(header)
            .when_some(self.action, |empty, action| {
                empty.content(EmptyContent::new().child(action))
            })
    }
}
