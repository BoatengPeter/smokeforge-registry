use super::*;

use gpui_kit::component::{Collapsible as _, Icon, IconName};

use crate::ui::PickerFrame;

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
        // Theme::change resets the font size, which is the interface size.
        self.applied_zoom = None;
    }

    /// Applies the interface size from settings (the theme's font size is the
    /// rem every GPUI Kit size is measured in).
    fn sync_zoom(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let zoom = ui::theme::zoom::clamp(self.settings.zoom);
        if self.applied_zoom != Some(zoom) {
            ui::theme::zoom::apply(zoom, cx);
            self.applied_zoom = Some(zoom);
            // Root sets the rem at the start of a frame; draw one more.
            window.refresh();
        }
    }

    /// Sets and saves the interface size.
    pub(crate) fn set_zoom(&mut self, zoom: f32, cx: &mut Context<Self>) {
        self.settings.zoom = ui::theme::zoom::clamp(zoom);
        let _ = self.settings_store.save(&self.settings);
        cx.notify();
    }

    fn step_zoom(&mut self, delta: f32, cx: &mut Context<Self>) {
        self.set_zoom(ui::theme::zoom::step(self.settings.zoom, delta), cx);
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

    /// The theme toggle over the content area. GPUI Kit's `TitleBar` moves the
    /// window and draws the window controls.
    fn title_bar(cx: &mut Context<Self>) -> impl IntoElement {
        let (theme_icon, theme_label) = if cx.theme().mode.is_dark() {
            (IconName::Sun, t("shell.light_theme"))
        } else {
            (IconName::Moon, t("shell.dark_theme"))
        };
        TitleBar::new().child(
            h_flex().w_full().pr_2().justify_end().child(
                Button::new("toggle-theme")
                    .ghost()
                    .small()
                    .icon(theme_icon)
                    .tooltip(theme_label)
                    .on_click(cx.listener(|this, _, _, cx| this.toggle_theme(cx))),
            ),
        )
    }

    /// Full-height sidebar: header (logo, name, search, collapse), the
    /// workspace pages with a "+" (New record), and Settings / About in the
    /// footer. Collapsed, GPUI Kit draws it as an icon rail.
    fn sidebar(&self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let collapsed = !self.settings.sidebar_open;
        let (main, footer): (Vec<Pane>, Vec<Pane>) = Pane::available(self.api_mode)
            .into_iter()
            .partition(|pane| !pane.in_sidebar_footer());
        let footer_menu = SidebarMenu::new()
            .children(footer.into_iter().map(|pane| self.nav_item(pane, cx)))
            .collapsed(collapsed)
            .render("sidebar-footer-menu", window, cx)
            .into_any_element();

        Sidebar::new("main-sidebar")
            .w(rems(15.))
            .collapsed(collapsed)
            .header(SidebarHeader::new().child(self.sidebar_header(collapsed, cx)))
            .child(
                SidebarGroup::new(t("sidebar.workspace"))
                    .suffix(|_, _| {
                        Button::new("sidebar-new-record")
                            .ghost()
                            .xsmall()
                            .icon(IconName::Plus)
                            .tooltip(t("shell.new_record"))
                            .on_click(|_, window, cx| {
                                window.dispatch_action(Box::new(NewRecord), cx);
                            })
                    })
                    .child(
                        SidebarMenu::new()
                            .children(main.into_iter().map(|pane| self.nav_item(pane, cx))),
                    ),
            )
            .footer(SidebarFooter::new().child(footer_menu))
    }

    fn nav_item(&self, pane: Pane, cx: &mut Context<Self>) -> SidebarMenuItem {
        SidebarMenuItem::new(pane.label())
            .icon(Icon::new(pane.icon()))
            .active(self.active == pane)
            .on_click(cx.listener(move |this, _, _, cx| {
                this.active = pane;
                cx.notify();
            }))
    }

    /// Logo and name (which drag the window), then search and collapse.
    fn sidebar_header(&self, collapsed: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let toggle_label = if collapsed {
            t("shell.expand")
        } else {
            t("shell.collapse")
        };
        let toggle = Button::new("sidebar-toggle")
            .ghost()
            .small()
            .icon(IconName::PanelLeft)
            .tooltip(toggle_label)
            .on_click(cx.listener(|this, _, _, cx| this.toggle_sidebar(cx)));
        if collapsed {
            return h_flex().w_full().justify_center().child(toggle);
        }
        h_flex()
            .w_full()
            .gap_1()
            .items_center()
            .child(
                h_flex()
                    .id("sidebar-drag")
                    .flex_1()
                    .min_w_0()
                    .gap_2()
                    .items_center()
                    .when(cfg!(target_os = "macos"), |this| this.pl(rems(4.5)))
                    .window_control_area(WindowControlArea::Drag)
                    .on_mouse_down(MouseButton::Left, |_, window, _| window.start_window_move())
                    .child(img(self.logo.clone()).size_5().rounded_md().flex_none())
                    .child(
                        div()
                            .min_w_0()
                            .truncate()
                            .text_sm()
                            .font_semibold()
                            .child(t("app.name")),
                    ),
            )
            .child(
                Button::new("sidebar-search")
                    .ghost()
                    .small()
                    .icon(IconName::Search)
                    .tooltip(t("shell.search"))
                    .on_click(cx.listener(|this, _, window, cx| this.open_go_to(window, cx))),
            )
            .child(toggle)
    }

    fn toggle_sidebar(&mut self, cx: &mut Context<Self>) {
        self.settings.sidebar_open = !self.settings.sidebar_open;
        let _ = self.settings_store.save(&self.settings);
        cx.notify();
    }

    fn open_go_to(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.show_command = true;
        self.command_state
            .update(cx, |state, cx| state.focus(window, cx));
        cx.notify();
    }

    fn close_go_to(&mut self, cx: &mut Context<Self>) {
        self.show_command = false;
        cx.notify();
    }

    /// Opens Library and puts the cursor in the new-record form.
    fn new_record(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.active = Pane::Library;
        let focus = self.title_input.read(cx).focus_handle(cx);
        window.focus(&focus, cx);
        cx.notify();
    }

    /// "Go to…" (sidebar search, Ctrl/Cmd+K): pages and actions in a GPUI Kit
    /// `Command` inside the ui crate's `PickerFrame`. Rows dispatch their
    /// actions; confirming or cancelling closes it.
    fn go_to(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        if !self.show_command {
            return None;
        }
        let entity = cx.entity();
        let close = move |_: &mut Window, cx: &mut App| {
            entity.update(cx, Self::close_go_to);
        };
        let pages = CommandGroup::new().label(t("go_to.pages")).items(
            Pane::available(self.api_mode).into_iter().map(|pane| {
                CommandItem::new()
                    .icon(Icon::new(pane.icon()))
                    .label(pane.label())
                    .action(go_to::page_action(pane))
            }),
        );
        let actions =
            CommandGroup::new()
                .label(t("go_to.actions"))
                .items(go_to::ACTIONS.iter().map(|row| {
                    CommandItem::new()
                        .icon(Icon::new(row.icon.clone()))
                        .label(t(row.label_key))
                        .action((row.action)())
                }));
        let on_cancel = close.clone();
        let command = Command::new(&self.command_state)
            .bordered(false)
            .placeholder(t("go_to.placeholder"))
            .group(pages)
            .group(actions)
            .max_h(rems(20.))
            .on_confirm(move |_, window, cx| close(window, cx))
            .on_cancel(move |window, cx| on_cancel(window, cx));
        Some(
            PickerFrame::new(t("go_to.title"))
                .escape(
                    Button::new("go-to-escape")
                        .ghost()
                        .xsmall()
                        .label(t("go_to.escape"))
                        .on_click(cx.listener(|this, _, _, cx| this.close_go_to(cx))),
                )
                .body(command)
                .into_any_element(),
        )
    }

    fn status_bar(cx: &mut Context<Self>) -> impl IntoElement {
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
        self.sync_zoom(window, cx);
        if self
            .focus_requested
            .swap(false, std::sync::atomic::Ordering::Relaxed)
        {
            window.activate_window();
        }

        let welcome = self.welcome_window(window, cx);
        let go_to = self.go_to(cx);

        // The sidebar runs the full window height; the title and status bars
        // span the content column only.
        h_flex()
            .relative()
            .size_full()
            .bg(cx.theme().background)
            .text_color(cx.theme().foreground)
            .child(self.sidebar(window, cx))
            .child(
                v_flex()
                    .flex_1()
                    .min_w_0()
                    .h_full()
                    .child(Self::title_bar(cx))
                    .child(
                        div()
                            .id("workspace")
                            .flex_1()
                            .min_h_0()
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
                    )
                    .child(Self::status_bar(cx)),
            )
            .when_some(go_to, gpui_kit::ParentElement::child)
            .when_some(welcome, gpui_kit::ParentElement::child)
            .map(|root| Self::window_handlers(root, cx))
    }
}

impl AppView {
    /// Workspace actions (navigation, theme, size, tour) and the pointer
    /// tracking that drags the welcome tour, attached to the root element.
    fn window_handlers(root: Div, cx: &mut Context<Self>) -> Div {
        root.on_action(cx.listener(|this, _: &ShowDashboard, _, cx| {
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
            if this.show_command {
                this.close_go_to(cx);
            } else {
                this.open_go_to(window, cx);
            }
        }))
        .on_action(cx.listener(|this, _: &ToggleSidebar, _, cx| this.toggle_sidebar(cx)))
        .on_action(cx.listener(|this, _: &NewRecord, window, cx| this.new_record(window, cx)))
        // The tour is dragged by its header but follows the pointer
        // anywhere in the window, so moves and releases are tracked here.
        .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, window, cx| {
            let size = ui::wizard_size(window.viewport_size(), window.rem_size());
            if this
                .welcome_drag
                .drag_to(event.position, window.viewport_size(), size)
            {
                cx.notify();
            }
        }))
        .on_mouse_up(
            MouseButton::Left,
            cx.listener(|this, _: &MouseUpEvent, _, cx| {
                if this.welcome_drag.end() {
                    cx.notify();
                }
            }),
        )
        .on_action(cx.listener(|this, _: &ToggleTheme, _, cx| this.toggle_theme(cx)))
        .on_action(cx.listener(|this, _: &ZoomIn, _, cx| {
            this.step_zoom(ui::theme::zoom::STEP, cx);
        }))
        .on_action(cx.listener(|this, _: &ZoomOut, _, cx| {
            this.step_zoom(-ui::theme::zoom::STEP, cx);
        }))
        .on_action(cx.listener(|this, _: &ZoomReset, _, cx| {
            this.set_zoom(ui::theme::zoom::DEFAULT, cx);
        }))
        .on_action(cx.listener(|this, _: &ShowWelcome, window, cx| {
            this.show_welcome(window, cx);
        }))
    }
}
