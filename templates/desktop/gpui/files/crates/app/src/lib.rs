//! GPUI application shell.

#![allow(
    clippy::wildcard_imports,
    reason = "GPUI Kit exports the framework prelude."
)]

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::TitleBar;
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_kit::component::status_bar::StatusBar;
use gpui_kit::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pane {
    Dashboard,
    Library,
    Activity,
    Settings,
    Account,
    About,
}

impl Pane {
    const ALL: [Self; 6] = [
        Self::Dashboard,
        Self::Library,
        Self::Activity,
        Self::Settings,
        Self::Account,
        Self::About,
    ];

    const fn label(self) -> &'static str {
        match self {
            Self::Dashboard => "Home",
            Self::Library => "Library",
            Self::Activity => "Activity",
            Self::Settings => "Settings",
            Self::Account => "Account",
            Self::About => "About",
        }
    }
}

pub struct AppView {
    active: Pane,
}

impl AppView {
    #[must_use]
    pub fn new() -> Self {
        Self {
            active: Pane::Dashboard,
        }
    }
}

impl Default for AppView {
    fn default() -> Self {
        Self::new()
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let sidebar = Sidebar::new("main-sidebar")
            .w(px(212.))
            .header(SidebarHeader::new().child("Smokeforge"))
            .child(
                SidebarGroup::new("Workspace").child(SidebarMenu::new().children(Pane::ALL.map(
                    |pane| {
                        SidebarMenuItem::new(pane.label())
                            .active(self.active == pane)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active = pane;
                                cx.notify();
                            }))
                    },
                ))),
            )
            .footer(SidebarFooter::new().child("Local · Rust · GPUI"));

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(TitleBar::new().child("Smokeforge"))
            .child(
                h_flex().flex_1().min_h_0().child(sidebar).child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .p_6()
                        .gap_2()
                        .child(div().text_2xl().font_semibold().child(self.active.label()))
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child("Local-first desktop workspace."),
                        ),
                ),
            )
            .child(
                StatusBar::new()
                    .left("Local")
                    .right("GPUI Kit 0.10 · pinned 0.3.12"),
            )
    }
}
