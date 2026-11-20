use super::*;

impl Render for AppView {
    #[expect(
        clippy::too_many_lines,
        reason = "Shell composition is kept in one render."
    )]
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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

        let sidebar = Sidebar::new("main-sidebar")
            .w(px(212.))
            .header(SidebarHeader::new().child(t("app.name")))
            .child(
                SidebarGroup::new(t("sidebar.workspace")).child(SidebarMenu::new().children(
                    Pane::available(self.api_mode).into_iter().map(|pane| {
                        SidebarMenuItem::new(pane.label())
                            .active(self.active == pane)
                            .on_click(cx.listener(move |this, _, _, cx| {
                                this.active = pane;
                                cx.notify();
                            }))
                    }),
                )),
            )
            .footer(SidebarFooter::new().child(t("sidebar.footer")));

        v_flex()
            .relative()
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
                        .child(div().text_2xl().font_semibold().child(self.active.label()))
                        .child(
                            div()
                                .text_color(cx.theme().muted_foreground)
                                .child(self.subtitle(cx)),
                        )
                        .child(self.pane_body(cx)),
                ),
            )
            .child(
                StatusBar::new()
                    .left(t("status.local"))
                    .right(t("status.version")),
            )
            .when_some(command, |this, command| {
                this.child(
                    div()
                        .absolute()
                        .top(px(54.))
                        .left(px(240.))
                        .right(px(24.))
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
