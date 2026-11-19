//! GPUI application shell.

#![allow(
    clippy::wildcard_imports,
    reason = "GPUI Kit exports the framework prelude."
)]

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::component::TitleBar;
use gpui_kit::component::button::{Button, ButtonVariants as _};
use gpui_kit::component::command::{Command, CommandItem, CommandState};
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_kit::component::status_bar::StatusBar;
use gpui_kit::component::{Sizable as _, Theme, ThemeMode};
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

rust_i18n::i18n!("locales");

gpui_kit::actions!(
    smokeforge,
    [
        ShowDashboard,
        ShowLibrary,
        ShowActivity,
        ShowSettings,
        ShowAccount,
        ShowAbout,
        ToggleCommand,
        ToggleTheme
    ]
);

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

    const fn title_key(self) -> &'static str {
        match self {
            Self::Dashboard => "nav.dashboard",
            Self::Library => "nav.library",
            Self::Activity => "nav.activity",
            Self::Settings => "nav.settings",
            Self::Account => "nav.account",
            Self::About => "nav.about",
        }
    }

    fn label(self) -> String {
        t(self.title_key())
    }
}

pub struct AppView {
    active: Pane,
    show_command: bool,
    command_state: Entity<CommandState>,
}

impl AppView {
    pub fn new(window: &mut Window, cx: &mut Context<Self>) -> Self {
        Self {
            active: Pane::Dashboard,
            show_command: false,
            command_state: cx.new(|cx| CommandState::new(window, cx)),
        }
    }
}

impl Render for AppView {
    #[expect(
        clippy::too_many_lines,
        reason = "Shell composition is kept in one render."
    )]
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let entity = cx.entity();
        let panes = Pane::ALL;
        let command = self.show_command.then(|| {
            Command::new(&self.command_state)
                .items(
                    panes
                        .iter()
                        .copied()
                        .map(|pane| CommandItem::new().label(pane.label())),
                )
                .on_confirm(move |path, _, cx| {
                    if let Some(pane) = panes.get(path.row).copied() {
                        entity.update(cx, |this, cx| {
                            this.active = pane;
                            this.show_command = false;
                            cx.notify();
                        });
                    }
                })
                .w(px(420.))
        });

        let sidebar =
            Sidebar::new("main-sidebar")
                .w(px(212.))
                .header(SidebarHeader::new().child(t("app.name")))
                .child(SidebarGroup::new(t("sidebar.workspace")).child(
                    SidebarMenu::new().children(Pane::ALL.map(|pane| {
                        SidebarMenuItem::new(pane.label())
                            .active(self.active == pane)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active = pane;
                                cx.notify();
                            }))
                    })),
                ))
                .footer(SidebarFooter::new().child(t("sidebar.footer")));

        v_flex()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(
                TitleBar::new().child(t("app.name")).child(
                    h_flex()
                        .gap_2()
                        .child(
                            Button::new("open-command")
                                .ghost()
                                .xsmall()
                                .label("Commands")
                                .on_click(cx.listener(|this, _, window, cx| {
                                    this.show_command = true;
                                    this.command_state
                                        .update(cx, |state, cx| state.focus(window, cx));
                                    cx.notify();
                                })),
                        )
                        .child(
                            Button::new("toggle-theme")
                                .ghost()
                                .xsmall()
                                .label("Theme")
                                .on_click(cx.listener(|_, _, window, cx| {
                                    let mode = Theme::global(cx).mode;
                                    Theme::change(
                                        if mode == ThemeMode::Dark {
                                            ThemeMode::Light
                                        } else {
                                            ThemeMode::Dark
                                        },
                                        Some(window),
                                        cx,
                                    );
                                })),
                        ),
                ),
            )
            .child(
                h_flex().flex_1().min_h_0().child(sidebar).child(
                    v_flex()
                        .flex_1()
                        .min_w_0()
                        .p_6()
                        .gap_2()
                        .when_some(command, ParentElement::child)
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
                    .left(t("status.local"))
                    .right(t("status.version")),
            )
            .on_action(cx.listener(|this, _: &ShowDashboard, _, cx| {
                this.active = Pane::Dashboard;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ShowLibrary, _, cx| {
                this.active = Pane::Library;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ShowActivity, _, cx| {
                this.active = Pane::Activity;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ShowSettings, _, cx| {
                this.active = Pane::Settings;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ShowAccount, _, cx| {
                this.active = Pane::Account;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ShowAbout, _, cx| {
                this.active = Pane::About;
                cx.notify();
            }))
            .on_action(cx.listener(|this, _: &ToggleCommand, window, cx| {
                this.show_command = !this.show_command;
                if this.show_command {
                    this.command_state
                        .update(cx, |state, cx| state.focus(window, cx));
                }
                cx.notify();
            }))
            .on_action(cx.listener(|_this, _: &ToggleTheme, window, cx| {
                let mode = Theme::global(cx).mode;
                Theme::change(
                    if mode == ThemeMode::Dark {
                        ThemeMode::Light
                    } else {
                        ThemeMode::Dark
                    },
                    Some(window),
                    cx,
                );
                cx.notify();
            }))
    }
}

fn t(key: &str) -> String {
    rust_i18n::t!(key).to_string()
}
