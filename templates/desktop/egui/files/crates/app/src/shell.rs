//! The app frame: full-height sidebar, title bar, status bar, workspace tabs,
//! and the "Go to…" window.

use egui::{
    CentralPanel, Color32, Frame, Margin, Panel, RichText, ScrollArea, Sense, Stroke, Ui,
    UiBuilder, Visuals, vec2,
};
use egui_tiles::{Behavior, TabState, TileId, Tiles, Tree, UiResponse};

use crate::components::{
    ConfirmChoice, ConfirmDialog, HoverTone, IconButton, Kbd, NavDensity, NavItem, PickerRow,
    PickerWindow, RowLead, picker_group, picker_search,
};
use crate::go_to::{self, GoToItem};
use crate::icons::Icon;
use crate::theme::{self, Palette, size, space};
use crate::{Action, DesktopApp, Pane, t};

/// Left inset that keeps the title bar clear of the macOS traffic lights.
const MACOS_TRAFFIC_LIGHTS: f32 = 78.0;
/// Side of the sidebar's icon buttons (search, collapse, section "+").
const NAV_BUTTON: f32 = 26.0;
/// The "Go to…" list scrolls past this height.
const GO_TO_LIST_HEIGHT: f32 = 320.0;
const TAB_BAR_HEIGHT: f32 = 38.0;
/// Widest a page's content grows before it centres in the workspace.
const CONTENT_MAX_WIDTH: f32 = 1200.0;
/// Minimum space between page content and the workspace edges.
const PAGE_GUTTER: f32 = 28.0;

impl DesktopApp {
    pub(crate) fn shell(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let bar = Frame::new().fill(palette.surface);
        let sidebar_width = if self.settings.sidebar_open {
            size::SIDEBAR
        } else {
            size::SIDEBAR_COLLAPSED
        };

        // The sidebar runs the full window height and its header is the
        // window's top-left corner; the title and status bars span the
        // content only. Panel order sets this: left first.
        Panel::left("sidebar")
            .exact_size(sidebar_width)
            .resizable(false)
            .frame(bar.inner_margin(Margin::symmetric(8, 0)))
            .show(ui, |ui| self.sidebar(ui));
        Panel::top("title-bar")
            .exact_size(size::TITLE_BAR)
            .resizable(false)
            .frame(bar.inner_margin(Margin::symmetric(8, 0)))
            .show(ui, |ui| self.title_bar(ui));
        Panel::bottom("status-bar")
            .exact_size(size::STATUS_BAR)
            .resizable(false)
            .frame(bar.inner_margin(Margin::symmetric(12, 0)))
            .show(ui, |ui| self.status_bar(ui));
        CentralPanel::no_frame()
            .frame(Frame::new().fill(palette.canvas))
            .show(ui, |ui| self.workspace(ui, palette));

        self.go_to_window(ui);
        self.confirmation_dialog(ui);
        self.welcome_window(ui);
        self.toasts.show(ui.ctx());
    }

    /// Theme toggle and window controls over the content area. The whole bar
    /// drags the window.
    fn title_bar(&mut self, ui: &mut Ui) {
        window_drag(ui, "title-drag", ui.max_rect());
        let mut action = None;
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.spacing_mut().item_spacing.x = 2.0;
            if !cfg!(target_os = "macos") {
                window_controls(ui);
                ui.add_space(space::S);
            }
            let (icon, label) = match ui.ctx().theme() {
                egui::Theme::Dark => (Icon::Sun, t("shell.light_theme")),
                egui::Theme::Light => (Icon::Moon, t("shell.dark_theme")),
            };
            if ui.add(IconButton::new(icon, &label)).clicked() {
                action = Some(Action::ToggleTheme);
            }
        });
        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    /// Header (logo, name, search, collapse), the workspace pages, and a
    /// footer with Settings and About. Collapsed, it is an icon rail.
    fn sidebar(&mut self, ui: &mut Ui) {
        let density = if self.settings.sidebar_open {
            NavDensity::Expanded
        } else {
            NavDensity::Collapsed
        };
        let mut action = self.sidebar_header(ui, density);
        ui.add_space(space::S);
        ui.spacing_mut().item_spacing.y = 2.0;

        if density == NavDensity::Expanded {
            if let Some(section_action) = sidebar_section(ui, &t("shell.workspace")) {
                action = Some(section_action);
            }
        } else if cfg!(target_os = "macos") {
            // On the macOS rail the traffic lights fill the header row.
            if ui
                .add(IconButton::new(Icon::PanelLeft, &t("shell.expand")).side(NAV_BUTTON))
                .clicked()
            {
                action = Some(Action::ToggleSidebar);
            }
        }
        let (main, footer): (Vec<Pane>, Vec<Pane>) = Pane::all(self.api_mode)
            .into_iter()
            .partition(|pane| !pane.in_sidebar_footer());
        for pane in main {
            if self.nav_item(ui, pane, density).clicked() {
                action = Some(Action::Navigate(pane));
            }
        }

        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            ui.add_space(space::S);
            // Bottom-up: the last pane is drawn first, at the bottom.
            for pane in footer.into_iter().rev() {
                if self.nav_item(ui, pane, density).clicked() {
                    action = Some(Action::Navigate(pane));
                }
            }
            ui.add_space(space::XS);
            ui.separator();
        });
        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    fn nav_item(&self, ui: &mut Ui, pane: Pane, density: NavDensity) -> egui::Response {
        let label = t(pane.title_key());
        ui.add(
            NavItem::new(pane.icon(), &label)
                .selected(pane == self.pane)
                .density(density),
        )
    }

    /// The sidebar's top row, as tall as the title bar so they read as one
    /// band. It drags the window like the title bar.
    fn sidebar_header(&self, ui: &mut Ui, density: NavDensity) -> Option<Action> {
        let (header, _) =
            ui.allocate_exact_size(vec2(ui.available_width(), size::TITLE_BAR), Sense::hover());
        window_drag(ui, "sidebar-drag", header);
        let mut action = None;
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(header)
                .layout(egui::Layout::left_to_right(egui::Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = space::XS;
                if density == NavDensity::Collapsed {
                    if !cfg!(target_os = "macos") {
                        ui.add_space((header.width() - NAV_BUTTON) / 2.0);
                        if ui
                            .add(
                                IconButton::new(Icon::PanelLeft, &t("shell.expand"))
                                    .side(NAV_BUTTON),
                            )
                            .clicked()
                        {
                            action = Some(Action::ToggleSidebar);
                        }
                    }
                    return;
                }
                ui.add_space(if cfg!(target_os = "macos") {
                    MACOS_TRAFFIC_LIGHTS - 8.0
                } else {
                    6.0
                });
                ui.add(
                    egui::Image::new(egui::include_image!("../../../assets/logo.png"))
                        .fit_to_exact_size(vec2(20.0, 20.0))
                        .corner_radius(5),
                );
                ui.add_space(space::XS);
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = space::XS;
                    if ui
                        .add(
                            IconButton::new(Icon::PanelLeft, &t("shell.collapse")).side(NAV_BUTTON),
                        )
                        .clicked()
                    {
                        action = Some(Action::ToggleSidebar);
                    }
                    if ui
                        .add(IconButton::new(Icon::Search, &t("shell.search")).side(NAV_BUTTON))
                        .clicked()
                    {
                        action = Some(Action::OpenPalette);
                    }
                    ui.add(
                        egui::Label::new(RichText::new(&self.app_name).font(theme::semibold(14.0)))
                            .selectable(false)
                            .truncate(),
                    );
                });
            },
        );
        action
    }

    fn status_bar(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let mut action = None;
        ui.horizontal_centered(|ui| {
            ui.spacing_mut().item_spacing.x = 6.0;
            let (dot, _) = ui.allocate_exact_size(vec2(8.0, 8.0), Sense::hover());
            ui.painter()
                .circle_filled(dot.center(), 3.5, palette.success);
            ui.label(
                RichText::new(t("status.local"))
                    .small()
                    .color(palette.text_secondary),
            );
            if !self.sync_status.is_empty() {
                ui.label(
                    RichText::new(&self.sync_status)
                        .small()
                        .color(palette.text_tertiary),
                );
            }
            if self.last_deleted.is_some()
                && ui.link(RichText::new(t("status.undo")).small()).clicked()
            {
                action = Some(Action::UndoDelete);
            }
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.label(
                    RichText::new(concat!("v", env!("CARGO_PKG_VERSION")))
                        .small()
                        .color(palette.text_tertiary),
                );
                ui.add_space(space::S);
                ui.label(
                    RichText::new(t("status.commands"))
                        .small()
                        .color(palette.text_tertiary),
                );
                ui.add(Kbd(&ui.ctx().format_shortcut(&crate::shortcuts::PALETTE)));
            });
        });
        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    fn workspace(&mut self, ui: &mut Ui, palette: Palette) {
        let mut tree =
            std::mem::replace(&mut self.tree, Tree::empty(egui::Id::unique("workspace")));
        let mut behavior = PaneBehavior { app: self, palette };
        tree.ui(&mut behavior, ui);
        self.tree = tree;
    }

    /// The "Go to…" window (sidebar search, Ctrl/Cmd+K): pages, matching
    /// records, and actions. Rows come from `go_to::items`.
    fn go_to_window(&mut self, ui: &mut Ui) {
        if !self.palette_open {
            return;
        }
        let records = if self.palette_query.trim().is_empty() {
            Vec::new()
        } else {
            self.records(&self.palette_query)
                .map(|items| {
                    items
                        .into_iter()
                        .map(|item| (item.id, item.title))
                        .collect()
                })
                .unwrap_or_default()
        };
        let rows = go_to::items(
            &self.palette_query,
            &Pane::all(self.api_mode),
            records,
            |pane| t(pane.title_key()),
            |command| t(command.label_key()),
        );
        ui.input(|input| {
            if input.key_pressed(egui::Key::ArrowDown) {
                self.palette_selection =
                    go_to::step_selection(self.palette_selection, rows.len(), true);
            }
            if input.key_pressed(egui::Key::ArrowUp) {
                self.palette_selection =
                    go_to::step_selection(self.palette_selection, rows.len(), false);
            }
        });
        self.palette_selection = self.palette_selection.min(rows.len().saturating_sub(1));
        let enter = ui.input(|input| input.key_pressed(egui::Key::Enter));

        let title = t("go_to.title");
        let mut chosen = None;
        let response = PickerWindow::new(egui::Id::unique("go-to"), &title, &t("go_to.escape"))
            .show(ui.ctx(), |ui| {
                let before = self.palette_query.clone();
                let search =
                    picker_search(ui, &title, &mut self.palette_query, &t("go_to.placeholder"));
                if self.palette_needs_focus {
                    search.request_focus();
                    self.palette_needs_focus = false;
                }
                if before != self.palette_query {
                    self.palette_selection = 0;
                }
                if rows.is_empty() {
                    let palette = theme::palette(ui.ctx());
                    ui.label(RichText::new(t("go_to.empty")).color(palette.text_secondary));
                }
                ScrollArea::vertical()
                    .max_height(GO_TO_LIST_HEIGHT)
                    .auto_shrink([false, true])
                    .show(ui, |ui| {
                        ui.spacing_mut().item_spacing.y = 2.0;
                        let mut group = None;
                        for (index, row) in rows.iter().enumerate() {
                            let heading = match row {
                                GoToItem::Page(_) => "go_to.pages",
                                GoToItem::Record { .. } => "go_to.records",
                                GoToItem::Action(_) => "go_to.actions",
                            };
                            if group != Some(heading) {
                                picker_group(ui, &t(heading));
                                group = Some(heading);
                            }
                            let selected = index == self.palette_selection;
                            let clicked = ui
                                .push_id(row.key(), |ui| go_to_row(ui, row, selected))
                                .inner
                                .clicked();
                            if clicked || (selected && enter) {
                                chosen = Some(row.clone());
                            }
                        }
                    });
            });

        if chosen.is_some() || response.close {
            self.dispatch(Action::ClosePalette);
            self.palette_query.clear();
        }
        match chosen {
            Some(GoToItem::Page(pane)) => self.dispatch(Action::Navigate(pane)),
            Some(GoToItem::Record { id, .. }) => {
                self.dispatch(Action::Navigate(Pane::Library));
                self.dispatch(Action::Select(Some(id)));
            }
            Some(GoToItem::Action(command)) => self.dispatch(Action::RunCommand(command)),
            None => {}
        }
    }
}

/// One "Go to…" row: pages and records get their icon, actions a "+".
fn go_to_row(ui: &mut Ui, row: &GoToItem, selected: bool) -> egui::Response {
    match row {
        GoToItem::Page(pane) => {
            let label = t(pane.title_key());
            ui.add(PickerRow::new(RowLead::Icon(pane.icon()), &label).selected(selected))
        }
        GoToItem::Record { title, .. } => {
            let hint = t("go_to.record_hint");
            ui.add(
                PickerRow::new(RowLead::Icon(Icon::Library), title)
                    .hint(&hint)
                    .selected(selected),
            )
        }
        GoToItem::Action(command) => {
            let label = t(command.label_key());
            ui.add(PickerRow::new(RowLead::Plus, &label).selected(selected))
        }
    }
}

/// A sidebar section label with a "+" (New record) at its trailing edge.
fn sidebar_section(ui: &mut Ui, label: &str) -> Option<Action> {
    let palette = theme::palette(ui.ctx());
    let mut action = None;
    ui.horizontal(|ui| {
        ui.add_space(space::S + 2.0);
        ui.label(RichText::new(label).color(palette.text_tertiary));
        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            if ui
                .add(IconButton::new(Icon::Plus, &t("shell.new_record")).side(NAV_BUTTON))
                .clicked()
            {
                action = Some(Action::NewRecord);
            }
        });
    });
    action
}

/// Makes `rect` drag the window; a double click maximizes. Call before
/// adding the controls inside `rect` so they keep their clicks.
fn window_drag(ui: &Ui, id_salt: &str, rect: egui::Rect) {
    let drag = ui.interact(rect, ui.scope_id().with(id_salt), Sense::click_and_drag());
    if drag.drag_started() {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
    }
    if drag.double_clicked() {
        let maximized = ui.input(|input| input.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
}

impl DesktopApp {
    fn confirmation_dialog(&mut self, ui: &mut Ui) {
        let Some(confirmation) = self.confirmation else {
            return;
        };
        let (title, body, confirm) = match confirmation {
            crate::Confirmation::ClearLocalData => (
                t("state.clear_title"),
                t("state.clear_body"),
                t("state.clear_confirm"),
            ),
        };
        let choice = ConfirmDialog::destructive(
            egui::Id::unique("confirmation"),
            &title,
            &body,
            &confirm,
            &t("common.cancel"),
        )
        .show(ui.ctx());
        match choice {
            Some(ConfirmChoice::Confirm) => self.dispatch(Action::Confirm),
            Some(ConfirmChoice::Cancel) => self.dispatch(Action::CancelConfirmation),
            None => {}
        }
    }
}

fn window_controls(ui: &mut Ui) {
    let close = t("shell.close");
    let maximize = t("shell.maximize");
    let minimize = t("shell.minimize");
    if ui
        .add(IconButton::new(Icon::Close, &close).hover_tone(HoverTone::Danger))
        .clicked()
    {
        ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
    }
    if ui.add(IconButton::new(Icon::Maximize, &maximize)).clicked() {
        let maximized = ui.input(|input| input.viewport().maximized.unwrap_or(false));
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
    }
    if ui.add(IconButton::new(Icon::Minimize, &minimize)).clicked() {
        ui.ctx()
            .send_viewport_cmd(egui::ViewportCommand::Minimized(true));
    }
}

struct PaneBehavior<'a> {
    app: &'a mut DesktopApp,
    palette: Palette,
}

impl Behavior<Pane> for PaneBehavior<'_> {
    fn tab_title_for_pane(&mut self, pane: &Pane) -> egui::WidgetText {
        t(pane.title_key()).into()
    }

    fn pane_ui(&mut self, ui: &mut Ui, _tile_id: TileId, pane: &mut Pane) -> UiResponse {
        ScrollArea::vertical()
            .id_salt(("pane", *pane))
            .auto_shrink(false)
            .show(ui, |ui| {
                let available = ui.available_width();
                let width = (available - 2.0 * PAGE_GUTTER).clamp(0.0, CONTENT_MAX_WIDTH);
                let left = ((available - width) / 2.0).max(0.0);
                ui.horizontal(|ui| {
                    ui.add_space(left);
                    ui.vertical(|ui| {
                        ui.set_width(width);
                        ui.add_space(space::XL);
                        match pane {
                            Pane::Dashboard => self.app.dashboard_view(ui),
                            Pane::Library => self.app.library_view(ui),
                            Pane::Activity => self.app.activity_view(ui),
                            Pane::Settings => self.app.settings_view(ui),
                            Pane::Account => self.app.account_view(ui),
                            Pane::About => self.app.about_view(ui),
                        }
                        ui.add_space(space::XXL);
                    });
                });
            });
        UiResponse::None
    }

    fn on_tab_button(
        &mut self,
        tiles: &mut Tiles<Pane>,
        tile_id: TileId,
        button_response: egui::Response,
    ) -> egui::Response {
        if button_response.clicked()
            && let Some(egui_tiles::Tile::Pane(pane)) = tiles.get(tile_id)
        {
            self.app.pane = *pane;
        }
        button_response
    }

    fn tab_bar_height(&self, _style: &egui::Style) -> f32 {
        TAB_BAR_HEIGHT
    }

    fn gap_width(&self, _style: &egui::Style) -> f32 {
        1.0
    }

    fn tab_bar_color(&self, _visuals: &Visuals) -> Color32 {
        self.palette.surface
    }

    fn tab_bg_color(
        &self,
        _visuals: &Visuals,
        _tiles: &Tiles<Pane>,
        _tile_id: TileId,
        state: &TabState,
    ) -> Color32 {
        if state.active {
            self.palette.canvas
        } else {
            Color32::TRANSPARENT
        }
    }

    fn tab_outline_stroke(
        &self,
        _visuals: &Visuals,
        _tiles: &Tiles<Pane>,
        _tile_id: TileId,
        state: &TabState,
    ) -> Stroke {
        if state.active {
            Stroke::new(1.0, self.palette.border)
        } else {
            Stroke::NONE
        }
    }

    fn tab_bar_hline_stroke(&self, _visuals: &Visuals) -> Stroke {
        Stroke::new(1.0, self.palette.border)
    }

    fn tab_text_color(
        &self,
        _visuals: &Visuals,
        _tiles: &Tiles<Pane>,
        _tile_id: TileId,
        state: &TabState,
    ) -> Color32 {
        if state.active {
            self.palette.text
        } else {
            self.palette.text_secondary
        }
    }

    fn resize_stroke(&self, _style: &egui::Style, state: egui_tiles::ResizeState) -> Stroke {
        match state {
            egui_tiles::ResizeState::Idle => Stroke::new(1.0, self.palette.border),
            egui_tiles::ResizeState::Hovering | egui_tiles::ResizeState::Dragging => {
                Stroke::new(2.0, self.palette.focus)
            }
        }
    }

    fn dragged_overlay_color(&self, _visuals: &Visuals) -> Color32 {
        self.palette.selected.gamma_multiply(0.6)
    }
}
