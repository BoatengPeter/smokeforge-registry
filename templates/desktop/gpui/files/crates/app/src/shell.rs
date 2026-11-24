use super::*;

use gpui_kit::component::kbd::Kbd;
use gpui_kit::component::{Icon, IconName};

impl Pane {
    const fn icon(self) -> IconName {
        match self {
            Self::Dashboard => IconName::LayoutDashboard,
            Self::Library => IconName::BookOpen,
            Self::Activity => IconName::ChartPie,
            Self::Settings => IconName::Settings,
            Self::Account => IconName::CircleUser,
            Self::About => IconName::Info,
        }
    }
}

impl AppView {
    /// Applies the theme the settings ask for when it changed.
    fn sync_theme(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        if self.applied_theme == self.settings.theme {
            return;
        }
        match self.settings.theme.as_str() {
            "light" => Theme::change(ThemeMode::Light, Some(window), cx),
            "system" => Theme::sync_system_appearance(Some(window), cx),
            _ => Theme::change(ThemeMode::Dark, Some(window), cx),
        }
        self.applied_theme.clone_from(&self.settings.theme);
    }

    /// Flips between light and dark and remembers the choice.
    fn toggle_theme(&mut self, cx: &mut Context<Self>) {
        let next = if cx.theme().mode.is_dark() {
            "light"
        } else {
            "dark"
        };
        next.clone_into(&mut self.settings.theme);
        let _ = self.settings_store.save(&self.settings);
        cx.notify();
    }

    fn title_bar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let (theme_icon, theme_label) = if cx.theme().mode.is_dark() {
            (IconName::Sun, t("shell.light_theme"))
        } else {
            (IconName::Moon, t("shell.dark_theme"))
        };
        let shortcut = Kbd::global_binding_for_action(&ToggleCommand, window);
        TitleBar::new().child(
            h_flex()
                .w_full()
                .pr_2()
                .justify_between()
                .child(
                    h_flex()
                        .gap_2()
                        .child(img(self.logo.clone()).size_4())
                        .child(div().text_sm().font_semibold().child(t("app.name"))),
                )
                .child(
                    h_flex()
                        .gap_1()
                        .child(
                            Button::new("open-command")
                                .ghost()
                                .small()
                                .icon(IconName::Search)
                                .label(t("shell.search"))
                                .when_some(shortcut, |button, kbd| button.child(kbd))
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
                                .small()
                                .icon(theme_icon)
                                .tooltip(theme_label)
                                .on_click(cx.listener(|this, _, _, cx| this.toggle_theme(cx))),
                        ),
                ),
        )
    }

    fn sidebar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        Sidebar::new("main-sidebar")
            .w(rems(15.))
            .header(
                SidebarHeader::new().child(
                    h_flex()
                        .gap_2()
                        .child(img(self.logo.clone()).size_5())
                        .child(div().text_sm().font_semibold().child(t("app.name"))),
                ),
            )
            .child(
                SidebarGroup::new(t("sidebar.workspace")).child(SidebarMenu::new().children(
                    Pane::available(self.api_mode).into_iter().map(|pane| {
                        SidebarMenuItem::new(pane.label())
                            .icon(Icon::new(pane.icon()))
                            .active(self.active == pane)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active = pane;
                                cx.notify();
                            }))
                    }),
                )),
            )
            .footer(
                SidebarFooter::new().child(
                    div()
                        .text_xs()
                        .text_color(cx.theme().sidebar_foreground)
                        .child(t("sidebar.footer")),
                ),
            )
    }

    fn status_bar(&self, cx: &mut Context<Self>) -> impl IntoElement {
        StatusBar::new()
            .left(
                h_flex()
                    .gap_1p5()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(div().size_2().rounded_full().bg(cx.theme().success))
                    .child(t("status.local")),
            )
            .right(
                div()
                    .text_xs()
                    .text_color(cx.theme().muted_foreground)
                    .child(t("status.version")),
            )
    }
}

impl Render for AppView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        self.sync_theme(window, cx);
        if self
            .focus_requested
            .swap(false, std::sync::atomic::Ordering::Relaxed)
        {
            window.activate_window();
        }

        let entity = cx.entity();
        let panes = Pane::available(self.api_mode);
        let command = self.show_command.then(|| {
            Command::new(&self.command_state)
                .items(panes.iter().copied().map(|pane| {
                    CommandItem::new()
                        .icon(Icon::new(pane.icon()))
                        .label(pane.label())
                }))
                .on_confirm(move |path, _, cx| {
                    if let Some(pane) = panes.get(path.row).copied() {
                        entity.update(cx, |this, cx| {
                            this.active = pane;
                            this.show_command = false;
                            cx.notify();
                        });
                    }
                })
                .w_112()
        });

        v_flex()
            .relative()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.title_bar(window, cx))
            .child(
                h_flex().flex_1().min_h_0().child(self.sidebar(cx)).child(
                    div()
                        .id("workspace")
                        .flex_1()
                        .min_w_0()
                        .h_full()
                        .overflow_y_scroll()
                        .child(
                            v_flex()
                                .w_full()
                                .max_w(rems(72.))
                                .mx_auto()
                                .px_8()
                                .py_6()
                                .gap_6()
                                .child(self.pane_body(cx)),
                        ),
                ),
            )
            .child(self.status_bar(cx))
            .when_some(command, |this, command| {
                this.child(
                    h_flex()
                        .absolute()
                        .top_16()
                        .left_0()
                        .right_0()
                        .justify_center()
                        .child(command),
                )
            })
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
            .on_action(cx.listener(|this, _: &ToggleTheme, _, cx| this.toggle_theme(cx)))
    }
}
