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
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::component::sidebar::{
    Sidebar, SidebarFooter, SidebarGroup, SidebarHeader, SidebarMenu, SidebarMenuItem,
};
use gpui_kit::component::status_bar::StatusBar;
use gpui_kit::component::table::{Table, TableBody, TableCell, TableHead, TableHeader, TableRow};
use gpui_kit::component::{Sizable as _, Theme, ThemeMode};
use gpui_kit::prelude::FluentBuilder as _;
pub mod credentials;
pub mod logging;
pub mod sync;

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
    fn available(api_mode: bool) -> Vec<Self> {
        let mut panes = vec![
            Self::Dashboard,
            Self::Library,
            Self::Activity,
            Self::Settings,
        ];
        if api_mode {
            panes.push(Self::Account);
        }
        panes.push(Self::About);
        panes
    }

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
    paths: {{.crate_name}}_storage::Paths,
    settings: {{.crate_name}}_storage::Settings,
    _settings_store: {{.crate_name}}_storage::SettingsStore,
    _logging_guard: logging::LoggingGuard,
    tokens: credentials::TokenStore,
    api_mode: bool,
    api_base: String,
    email_input: Entity<InputState>,
    password_input: Entity<InputState>,
    auth_status: String,
    access_token: Option<String>,
    auth_task: Option<Task<()>>,
    sync_task: Option<Task<()>>,
    sync_status: String,
    focus_requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    database: Option<{{.crate_name}}_storage::Database>,
    title_input: Entity<InputState>,
    tags_input: Entity<InputState>,
}

impl AppView {
    pub fn new(
        window: &mut Window,
        cx: &mut Context<Self>,
        focus_requested: std::sync::Arc<std::sync::atomic::AtomicBool>,
    ) -> Self {
        let paths = {{.crate_name}}_storage::Paths::discover("Smokeforge").unwrap_or_else(|_| {
            {{.crate_name}}_storage::Paths {
                data: std::env::temp_dir().join("smokeforge"),
                config: std::env::temp_dir().join("smokeforge"),
                logs: std::env::temp_dir().join("smokeforge-logs"),
            }
        });
        let settings_store = {{.crate_name}}_storage::SettingsStore::new(&paths.config);
        let settings = settings_store.load().unwrap_or_default();
        let logging_guard = logging::init(&paths.logs, &settings.log_level);
        let database = {{.crate_name}}_storage::Database::open(&paths.data.join("local.sqlite"))
            .ok()
            .inspect(|database| {
                let _ = database.seed_if_empty();
            });
        let title_input = cx.new(|cx| InputState::new(window, cx).placeholder("Title"));
        let tags_input =
            cx.new(|cx| InputState::new(window, cx).placeholder("tags, comma separated"));
        let api_mode = std::env::var("SMOKEFORGE_API_MODE")
            .is_ok_and(|value| matches!(value.as_str(), "api" | "1" | "true"));
        let api_base = std::env::var("SMOKEFORGE_API_URL")
            .unwrap_or_else(|_| "http://localhost:8080".to_owned());
        let email_input = cx.new(|cx| InputState::new(window, cx).placeholder("Email"));
        let password_input = cx.new(|cx| {
            InputState::new(window, cx)
                .placeholder("Password")
                .masked(true)
        });
        Self {
            active: Pane::Dashboard,
            show_command: false,
            command_state: cx.new(|cx| CommandState::new(window, cx)),
            paths,
            settings,
            _settings_store: settings_store,
            _logging_guard: logging_guard,
            database,
            title_input,
            tags_input,
            tokens: credentials::TokenStore::new("smokeforge-gpui", "auth"),
            api_mode,
            api_base,
            email_input,
            password_input,
            auth_status: String::new(),
            access_token: None,
            auth_task: None,
            sync_task: None,
            sync_status: String::new(),
            focus_requested,
        }
    }
}

impl AppView {
    fn subtitle(&self, _cx: &mut Context<Self>) -> String {
        match self.active {
            Pane::Settings => "Preferences, local data, and diagnostics.".to_owned(),
            Pane::Account => {
                if self.tokens.warning().is_some() {
                    "API account · keyring unavailable; tokens will stay in memory.".to_owned()
                } else {
                    "API account session.".to_owned()
                }
            }
            _ => "Local-first desktop workspace.".to_owned(),
        }
    }

    fn conflict_rows(&self, cx: &mut Context<Self>) -> Vec<impl IntoElement> {
        let conflicts = self
            .database
            .as_ref()
            .and_then(|database| database.conflicts().ok())
            .unwrap_or_default();
        conflicts
            .into_iter()
            .map(|(item_id, _, _)| {
                let keep = item_id.clone();
                let remote = item_id.clone();
                h_flex()
                    .gap_2()
                    .child(format!("Conflict: {item_id}"))
                    .child(
                        Button::new(format!("keep-{item_id}"))
                            .xsmall()
                            .label("Keep local")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(database) = &this.database {
                                    let _ = database.resolve_conflict_local(&keep);
                                }
                                cx.notify();
                            })),
                    )
                    .child(
                        Button::new(format!("server-{item_id}"))
                            .xsmall()
                            .label("Use server")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(database) = &this.database {
                                    let _ = database.resolve_conflict_remote(&remote);
                                }
                                cx.notify();
                            })),
                    )
            })
            .collect()
    }

    #[expect(
        clippy::too_many_lines,
        reason = "Pane composition intentionally lives together."
    )]
    fn pane_body(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        match self.active {
            Pane::Settings => v_flex()
                .gap_3()
                .child(format!("Theme: {}", self.settings.theme))
                .child(format!("Language: {}", self.settings.language))
                .child(format!("Log level: {}", self.settings.log_level))
                .child(format!("Data: {}", self.paths.data.display()))
                .child(format!("Logs: {}", self.paths.logs.display()))
                .child(format!("Config: {}", self.paths.config.display()))
                .into_any_element(),
            Pane::Account => v_flex()
                .gap_2()
                .when_some(self.access_token.as_ref(), |this, token| {
                    this.child(format!(
                        "Signed in · token {}",
                        &token[..token.len().min(8)]
                    ))
                    .child(
                        Button::new("sync-now")
                            .label("Sync now")
                            .on_click(cx.listener(|this, _, _, cx| {
                                let Some(token) = this.access_token.clone() else {
                                    return;
                                };
                                let base = this.api_base.clone();
                                let path = this.paths.data.join("local.sqlite");
                                "Syncing…".clone_into(&mut this.sync_status);
                                this.sync_task = Some(cx.spawn(async move |weak, cx| {
                                    let report = cx
                                        .background_spawn(
                                            async move { sync::run(&path, &base, token) },
                                        )
                                        .await;
                                    weak.update(cx, |this, cx| {
                                        this.sync_status = format!(
                                            "Pushed {} · conflicts {} · retried {}",
                                            report.pushed, report.conflicts, report.retried
                                        );
                                        cx.notify();
                                    })
                                    .ok();
                                }));
                                cx.notify();
                            })),
                    )
                })
                .when(self.access_token.is_none(), |this| {
                    this.child(Input::new(&self.email_input))
                        .child(Input::new(&self.password_input))
                        .child(Button::new("sign-in").primary().label("Sign in").on_click(
                            cx.listener(|this, _, _, cx| {
                                let email = this.email_input.read(cx).value().to_string();
                                let password = this.password_input.read(cx).value().to_string();
                                if email.is_empty() || password.is_empty() {
                                    "Email and password are required."
                                        .clone_into(&mut this.auth_status);
                                    cx.notify();
                                    return;
                                }
                                let base = this.api_base.clone();
                                "Signing in…".clone_into(&mut this.auth_status);
                                this.auth_task = Some(cx.spawn(async move |weak, cx| {
                                    let result = cx
                                        .background_spawn(async move {
                                            let runtime =
                                                tokio::runtime::Builder::new_current_thread()
                                                    .enable_all()
                                                    .build()
                                                    .map_err(|error| error.to_string())?;
                                            runtime.block_on(async move {
                                                let mut client = api_client::ApiClient::new(&base)
                                                    .map_err(|error| error.to_string())?;
                                                client
                                                    .login(&email, &password)
                                                    .await
                                                    .map_err(|error| error.to_string())
                                            })
                                        })
                                        .await;
                                    weak.update(cx, |this, cx| {
                                        match result {
                                            Ok(session) => {
                                                this.tokens.set(&session.refresh_token).ok();
                                                this.access_token = Some(session.access_token);
                                                "Signed in.".clone_into(&mut this.auth_status);
                                            }
                                            Err(error) => {
                                                this.auth_status = error;
                                            }
                                        }
                                        cx.notify();
                                    })
                                    .ok();
                                }));
                                cx.notify();
                            }),
                        ))
                })
                .when(!self.auth_status.is_empty(), |this| {
                    this.child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child(self.auth_status.clone()),
                    )
                })
                .when(!self.sync_status.is_empty(), |this| {
                    this.child(
                        div()
                            .text_color(cx.theme().muted_foreground)
                            .child(self.sync_status.clone()),
                    )
                })
                .children(self.conflict_rows(cx))
                .when_some(self.tokens.warning().map(str::to_owned), |this, warning| {
                    this.child(div().text_color(cx.theme().warning).child(warning))
                })
                .into_any_element(),
            Pane::Library => self.library_body(cx).into_any_element(),
            _ => div()
                .text_color(cx.theme().muted_foreground)
                .child("No records yet.")
                .into_any_element(),
        }
    }

    fn library_body(&mut self, cx: &mut Context<Self>) -> impl IntoElement {
        let items = self
            .database
            .as_ref()
            .and_then(|database| database.list("", None).ok())
            .unwrap_or_default();
        let rows = items.into_iter().map(|item| {
            let id = item.id;
            TableRow::new()
                .id(id.to_string())
                .child(TableCell::new().child(item.title))
                .child(TableCell::new().child(item.status.label()))
                .child(
                    TableCell::new().child(
                        Button::new(format!("delete-{id}"))
                            .ghost()
                            .xsmall()
                            .label("Delete")
                            .on_click(cx.listener(move |this, _, _, cx| {
                                if let Some(database) = &this.database {
                                    let _ = database.soft_delete(id);
                                }
                                cx.notify();
                            })),
                    ),
                )
        });

        v_flex()
            .gap_3()
            .child(Input::new(&self.title_input))
            .child(Input::new(&self.tags_input))
            .child(
                Button::new("create-item")
                    .primary()
                    .label("Create record")
                    .on_click(cx.listener(|this, _, window, cx| {
                        let title = this.title_input.read(cx).value().to_string();
                        if title.trim().is_empty() {
                            return;
                        }
                        let tags = this
                            .tags_input
                            .read(cx)
                            .value()
                            .split(',')
                            .map(str::trim)
                            .filter(|tag| !tag.is_empty())
                            .map(str::to_owned)
                            .collect();
                        if let Some(database) = &this.database {
                            let _ = database.create(&{{.crate_name}}_core::LibraryDraft {
                                title,
                                description: String::new(),
                                status: {{.crate_name}}_core::LibraryStatus::Draft,
                                tags,
                            });
                        }
                        this.title_input =
                            cx.new(|cx| InputState::new(window, cx).placeholder("Title"));
                        this.tags_input = cx.new(|cx| {
                            InputState::new(window, cx).placeholder("tags, comma separated")
                        });
                        cx.notify();
                    })),
            )
            .child(
                Table::new()
                    .child(
                        TableHeader::new().child(
                            TableRow::new()
                                .child(TableHead::new().child("Title"))
                                .child(TableHead::new().w(px(100.)).child("Status"))
                                .child(TableHead::new().w(px(90.)).child("")),
                        ),
                    )
                    .child(TableBody::new().children(rows)),
            )
            .into_any_element()
    }
}

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
