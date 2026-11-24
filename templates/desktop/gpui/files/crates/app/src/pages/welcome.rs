//! The first-run welcome tour: a floating, draggable window over the live app.
//!
//! It opens on first launch (`settings.onboarded == false`) and again from
//! Settings › Help or the `ShowWelcome` action. Closing it any way marks the
//! tour as seen. The chrome is the ui crate's `WizardWindow`; this module owns
//! the tour's transitions and draws its pages from components. To add a page:
//! add a `WelcomePage` variant (in `crate::welcome`), its `welcome.*` locale
//! keys, and a match arm in `welcome_page`. See `docs/STATES.md`.

use gpui_kit::component::kbd::Kbd;

use super::*;
use crate::welcome::{WelcomeOutcome, WelcomePage};

impl AppView {
    /// Opens the tour on its first page, centred, with keyboard focus.
    pub(crate) fn show_welcome(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        self.welcome = Some(WelcomePage::Intro);
        self.welcome_drag.reset();
        window.focus(&self.welcome_focus, cx);
        cx.notify();
    }

    fn welcome_go(&mut self, page: WelcomePage, cx: &mut Context<Self>) {
        self.welcome = Some(page);
        cx.notify();
    }

    /// Closes the tour and remembers that it was seen.
    pub(crate) fn close_welcome(&mut self, outcome: WelcomeOutcome, cx: &mut Context<Self>) {
        self.welcome = None;
        self.welcome_drag.end();
        self.settings.onboarded = true;
        let _ = self.settings_store.save(&self.settings);
        tracing::info!(?outcome, "welcome tour closed");
        cx.notify();
    }

    /// Forward from `page`: the next page, or finish on the last one.
    fn welcome_forward(&mut self, page: WelcomePage, cx: &mut Context<Self>) {
        match page.next() {
            Some(next) => self.welcome_go(next, cx),
            None => self.close_welcome(WelcomeOutcome::Finished, cx),
        }
    }

    /// Escape skips; Enter and → go forward and ← back while the tour itself
    /// (not one of its buttons) has focus.
    fn welcome_key(&mut self, event: &KeyDownEvent, window: &mut Window, cx: &mut Context<Self>) {
        let Some(page) = self.welcome else {
            return;
        };
        let key = event.keystroke.key.as_str();
        if key == "escape" {
            self.close_welcome(WelcomeOutcome::Skipped, cx);
            cx.stop_propagation();
            return;
        }
        if !self.welcome_focus.is_focused(window) || event.keystroke.modifiers.modified() {
            return;
        }
        match key {
            "enter" | "right" => self.welcome_forward(page, cx),
            "left" => {
                if let Some(previous) = page.previous() {
                    self.welcome_go(previous, cx);
                }
            }
            _ => return,
        }
        cx.stop_propagation();
    }

    /// The tour window, when it is open. The shell adds it as the last child
    /// of its root and forwards mouse moves to `welcome_drag`.
    pub(crate) fn welcome_window(
        &mut self,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<AnyElement> {
        let page = self.welcome?;
        let size = wizard_size(window.viewport_size(), window.rem_size());
        let progress = rust_i18n::t!(
            "welcome.progress",
            step = page.index() + 1,
            total = WelcomePage::ALL.len()
        );
        let next_label = if page.next().is_some() {
            t("welcome.next")
        } else {
            t("welcome.finish")
        };

        let wizard = WizardWindow::new(
            "welcome-tour",
            rust_i18n::t!("welcome.window_title", app = t("app.name")).to_string(),
            page.index(),
            WelcomePage::ALL.len(),
            size,
        )
        .badge(format!("v{}", env!("CARGO_PKG_VERSION")))
        .logo(self.logo.clone())
        .labels(t("welcome.drag_hint"), progress.to_string())
        .offset(self.welcome_drag.offset())
        .on_drag_start(cx.listener(|this, event: &MouseDownEvent, _, cx| {
            this.welcome_drag.start(event.position);
            cx.notify();
        }))
        .close(
            Button::new("welcome-close")
                .ghost()
                .small()
                .icon(IconName::Close)
                .tooltip(t("welcome.skip"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.close_welcome(WelcomeOutcome::Skipped, cx);
                })),
        )
        .when_some(page.previous(), |wizard, previous| {
            wizard.back(
                Button::new("welcome-back")
                    .ghost()
                    .label(t("welcome.back"))
                    .on_click(cx.listener(move |this, _, _, cx| this.welcome_go(previous, cx))),
            )
        })
        .next(
            Button::new("welcome-next")
                .primary()
                .label(next_label)
                .child(gpui_kit::component::Icon::new(IconName::ArrowRight))
                .on_click(cx.listener(move |this, _, _, cx| this.welcome_forward(page, cx))),
        )
        .body(self.welcome_page(page, window, cx));

        Some(
            div()
                .id("welcome-layer")
                .absolute()
                .inset_0()
                .track_focus(&self.welcome_focus)
                .on_key_down(cx.listener(Self::welcome_key))
                .child(wizard)
                .into_any_element(),
        )
    }

    fn welcome_page(
        &mut self,
        page: WelcomePage,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        match page {
            WelcomePage::Intro => self.intro_page(cx),
            WelcomePage::Shortcuts => shortcuts_page(window, cx),
            WelcomePage::Ready => self.ready_page(cx),
        }
    }

    fn intro_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        v_flex()
            .gap_5()
            .child(page_title(
                rust_i18n::t!("welcome.intro_title", app = t("app.name")).to_string(),
                t("welcome.intro_body"),
                cx,
            ))
            .child(FeatureRow::new(
                IconName::HardDrive,
                t("welcome.local_title"),
                t("welcome.local_body"),
            ))
            .child(FeatureRow::new(
                IconName::Search,
                t("welcome.find_title"),
                t("welcome.find_body"),
            ))
            .child(FeatureRow::new(
                IconName::RefreshCw,
                t("welcome.sync_title"),
                t("welcome.sync_body"),
            ))
            .child(div().h_px().bg(cx.theme().border))
            .child(labelled_row(
                t("welcome.appearance"),
                self.theme_choice("welcome-theme", cx),
                cx,
            ))
            .child(labelled_row(
                t("welcome.size"),
                self.size_stepper("welcome-size", cx),
                cx,
            ))
            .into_any_element()
    }

    fn ready_page(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let data = self.paths.data.clone();
        v_flex()
            .gap_5()
            .child(page_title(
                t("welcome.ready_title"),
                t("welcome.ready_body"),
                cx,
            ))
            .child(
                Inset::new()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .child(t("welcome.data_title")),
                    )
                    .child(muted(t("welcome.data_body"), cx))
                    .child(
                        div()
                            .text_sm()
                            .font_family(cx.theme().mono_font_family.clone())
                            .child(data.display().to_string()),
                    )
                    .child(
                        h_flex().child(
                            Button::new("welcome-open-data")
                                .outline()
                                .small()
                                .icon(IconName::FolderOpen)
                                .label(t("welcome.open_folder"))
                                .on_click(move |_, _, cx| cx.open_with_system(&data)),
                        ),
                    ),
            )
            .child(
                Inset::new()
                    .child(
                        div()
                            .text_sm()
                            .font_semibold()
                            .child(t("welcome.first_title")),
                    )
                    .child(muted(t("welcome.first_body"), cx))
                    .child(
                        h_flex().child(
                            Button::new("welcome-new-record")
                                .outline()
                                .small()
                                .icon(IconName::Plus)
                                .label(t("library.new"))
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_welcome(WelcomeOutcome::Finished, cx);
                                    this.active = Pane::Library;
                                    cx.notify();
                                })),
                        ),
                    ),
            )
            .child(muted(t("welcome.reopen"), cx))
            .into_any_element()
    }
}

fn shortcuts_page(window: &mut Window, cx: &mut Context<AppView>) -> AnyElement {
    let binding = |action: &dyn gpui_kit::Action| Kbd::global_binding_for_action(action, window);
    let rows = [
        (binding(&ShowLibrary), t("welcome.key_library")),
        (binding(&ShowSettings), t("welcome.key_settings")),
        (binding(&ZoomIn), t("welcome.key_zoom_in")),
        (binding(&ZoomOut), t("welcome.key_zoom_out")),
        (binding(&ToggleTheme), t("welcome.key_theme")),
    ];
    v_flex()
        .gap_5()
        .child(page_title(
            t("welcome.keys_title"),
            t("welcome.keys_body"),
            cx,
        ))
        .child(
            Inset::callout().child(
                h_flex()
                    .gap_3()
                    .items_start()
                    .children(binding(&ToggleCommand))
                    .child(
                        v_flex()
                            .gap_1()
                            .child(
                                div()
                                    .text_sm()
                                    .font_semibold()
                                    .child(t("welcome.palette_title")),
                            )
                            .child(muted(t("welcome.palette_body"), cx)),
                    ),
            ),
        )
        .child(
            v_flex()
                .gap_2()
                .children(rows.into_iter().filter_map(|(keys, label)| {
                    // An unbound action has nothing to teach; leave it out.
                    keys.map(|keys| {
                        h_flex()
                            .gap_3()
                            .text_sm()
                            .child(div().w(rems(9.)).flex_none().child(keys))
                            .child(label)
                    })
                }))
                .child(
                    h_flex()
                        .gap_3()
                        .text_sm()
                        .child(div().w(rems(9.)).flex_none().child(Kbd::new(
                            gpui_kit::Keystroke::parse("escape").expect("escape is a valid key"),
                        )))
                        .child(t("welcome.key_escape")),
                ),
        )
        .into_any_element()
}

/// Page heading plus a paragraph under it.
fn page_title(title: String, body: String, cx: &App) -> impl IntoElement {
    v_flex()
        .gap_2()
        .child(div().text_xl().font_semibold().child(title))
        .child(muted(body, cx))
}

fn muted(text: String, cx: &App) -> impl IntoElement {
    div()
        .text_sm()
        .text_color(cx.theme().muted_foreground)
        .child(text)
}

/// A fixed-width label column and a control, like the Appearance/Size rows.
fn labelled_row(label: String, control: impl IntoElement, cx: &App) -> impl IntoElement {
    h_flex()
        .gap_3()
        .items_center()
        .child(
            div()
                .w(rems(6.))
                .flex_none()
                .text_sm()
                .text_color(cx.theme().muted_foreground)
                .child(label),
        )
        .child(control)
}
