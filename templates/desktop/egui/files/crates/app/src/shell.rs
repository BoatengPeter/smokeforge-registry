//! The app frame: title bar, sidebar, status bar, workspace tabs, and the
//! command palette.

use egui::{
    CentralPanel, Color32, Frame, Margin, Panel, RichText, Role, ScrollArea, Sense, Stroke,
    StrokeKind, TextStyle, Ui, Visuals, WidgetInfo, vec2,
};
use egui_tiles::{Behavior, TabState, TileId, Tiles, Tree, UiResponse};

use crate::components::{
    ConfirmChoice, ConfirmDialog, HoverTone, IconButton, Kbd, ListRow, NavDensity, NavItem,
    search_field,
};
use crate::icons::{self, Icon};
use crate::theme::{self, Palette, radius, size, space};
use crate::{Action, DesktopApp, Pane, commands, t};

/// Left inset that keeps the title bar clear of the macOS traffic lights.
const MACOS_TRAFFIC_LIGHTS: f32 = 78.0;
const SEARCH_PILL_WIDTH: f32 = 340.0;
const PALETTE_WIDTH: f32 = 520.0;
const TAB_BAR_HEIGHT: f32 = 38.0;
/// Widest a page's content grows before it centres in the workspace.
const CONTENT_MAX_WIDTH: f32 = 1200.0;
/// Minimum space between page content and the workspace edges.
const PAGE_GUTTER: f32 = 28.0;

impl DesktopApp {
    pub(crate) fn shell(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let bar = Frame::new().fill(palette.surface);

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
        let sidebar_width = if self.settings.sidebar_open {
            size::SIDEBAR
        } else {
            size::SIDEBAR_COLLAPSED
        };
        Panel::left("sidebar")
            .exact_size(sidebar_width)
            .resizable(false)
            .frame(bar.inner_margin(Margin::same(10)))
            .show(ui, |ui| self.sidebar(ui));
        CentralPanel::no_frame()
            .frame(Frame::new().fill(palette.canvas))
            .show(ui, |ui| self.workspace(ui, palette));

        self.command_palette(ui);
        self.confirmation_dialog(ui);
        self.welcome_window(ui);
        self.toasts.show(ui.ctx());
    }

    fn title_bar(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let bar = ui.max_rect();
        let drag = ui.interact(
            bar,
            ui.scope_id().with("title-drag"),
            Sense::click_and_drag(),
        );
        if drag.drag_started() {
            ui.ctx().send_viewport_cmd(egui::ViewportCommand::StartDrag);
        }
        if drag.double_clicked() {
            let maximized = ui.input(|input| input.viewport().maximized.unwrap_or(false));
            ui.ctx()
                .send_viewport_cmd(egui::ViewportCommand::Maximized(!maximized));
        }

        let mut action = None;
        ui.horizontal_centered(|ui| {
            if cfg!(target_os = "macos") {
                ui.add_space(MACOS_TRAFFIC_LIGHTS);
            } else {
                ui.add_space(space::XS);
            }
            ui.add(
                egui::Image::new(egui::include_image!("../../../assets/logo.png"))
                    .fit_to_exact_size(vec2(18.0, 18.0))
                    .tint(palette.text),
            );
            ui.label(RichText::new(&self.app_name).font(theme::semibold(14.0)));

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
        });

        let pill = egui::Rect::from_center_size(
            bar.center(),
            vec2(
                SEARCH_PILL_WIDTH.min(bar.width() * 0.4),
                size::CONTROL - 2.0,
            ),
        );
        if search_pill(ui, pill, &palette).clicked() {
            action = Some(Action::OpenPalette);
        }
        if let Some(action) = action {
            self.dispatch(action);
        }
    }

    fn sidebar(&mut self, ui: &mut Ui) {
        let palette = theme::palette(ui.ctx());
        let density = if self.settings.sidebar_open {
            NavDensity::Expanded
        } else {
            NavDensity::Collapsed
        };
        let mut action = None;
        ui.spacing_mut().item_spacing.y = 2.0;
        if density == NavDensity::Expanded {
            ui.add_space(space::XS);
            ui.label(
                RichText::new(t("shell.workspace").to_uppercase())
                    .small()
                    .color(palette.text_tertiary),
            );
            ui.add_space(space::XS);
        }
        for pane in Pane::all(self.api_mode) {
            let label = t(pane.title_key());
            let item = NavItem::new(pane.icon(), &label)
                .selected(pane == self.pane)
                .density(density);
            if ui.add(item).clicked() {
                action = Some(Action::Navigate(pane));
            }
        }
        ui.with_layout(egui::Layout::bottom_up(egui::Align::Min), |ui| {
            let label = match density {
                NavDensity::Expanded => t("shell.collapse"),
                NavDensity::Collapsed => t("shell.expand"),
            };
            if ui.add(IconButton::new(Icon::PanelLeft, &label)).clicked() {
                action = Some(Action::ToggleSidebar);
            }
        });
        if let Some(action) = action {
            self.dispatch(action);
        }
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
                ui.add(Kbd(shortcut_label()));
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

    fn command_palette(&mut self, ui: &mut Ui) {
        if !self.palette_open {
            return;
        }
        let commands =
            commands::filter_commands(&self.palette_query, |command| t(command.label_key()));
        if !commands.is_empty() {
            let last = commands.len() - 1;
            ui.input(|input| {
                if input.key_pressed(egui::Key::ArrowDown) {
                    self.palette_selection = (self.palette_selection + 1).min(last);
                }
                if input.key_pressed(egui::Key::ArrowUp) {
                    self.palette_selection = self.palette_selection.saturating_sub(1);
                }
            });
            self.palette_selection = self.palette_selection.min(last);
        }
        let enter = ui.input(|input| input.key_pressed(egui::Key::Enter));

        let mut chosen = None;
        let modal = egui::Modal::new(egui::Id::unique("command-palette"))
            .backdrop_color(Color32::from_black_alpha(70))
            .show(ui.ctx(), |ui| {
                ui.set_width(PALETTE_WIDTH);
                let query = self.palette_query.clone();
                let search = search_field(
                    ui,
                    &t("command.title"),
                    &mut self.palette_query,
                    &t("command.placeholder"),
                );
                if self.palette_needs_focus {
                    search.request_focus();
                    self.palette_needs_focus = false;
                }
                if query != self.palette_query {
                    self.palette_selection = 0;
                }
                ui.add_space(space::S);
                if commands.is_empty() {
                    let palette = theme::palette(ui.ctx());
                    ui.label(RichText::new(t("command.empty")).color(palette.text_secondary));
                }
                ui.spacing_mut().item_spacing.y = 2.0;
                for (index, command) in commands.iter().enumerate() {
                    let label = t(command.label_key());
                    let selected = index == self.palette_selection;
                    let row = ui.push_id(command.label_key(), |ui| {
                        ui.add(ListRow::new(&label).selected(selected))
                    });
                    if row.inner.clicked() || (selected && enter) {
                        chosen = Some(*command);
                    }
                }
            });

        if chosen.is_some() || modal.should_close() {
            self.dispatch(Action::ClosePalette);
            self.palette_query.clear();
        }
        if let Some(command) = chosen {
            self.dispatch(Action::RunCommand(command));
        }
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

/// The centred "search or run a command" button in the title bar.
fn search_pill(ui: &mut Ui, rect: egui::Rect, palette: &Palette) -> egui::Response {
    let label = t("shell.search");
    let response = ui.interact(rect, ui.scope_id().with("search-pill"), Sense::click());
    response.widget_info(|| WidgetInfo::labeled(Role::Button, true, &label));
    let fill = if response.hovered() {
        palette.hover
    } else {
        palette.field
    };
    ui.painter().rect(
        rect,
        radius::MD,
        fill,
        Stroke::new(1.0, palette.border),
        StrokeKind::Inside,
    );
    let icon_rect = egui::Rect::from_min_size(
        egui::pos2(rect.left() + space::S + 2.0, rect.center().y - 7.0),
        vec2(14.0, 14.0),
    );
    icons::paint(ui.painter(), icon_rect, Icon::Search, palette.text_tertiary);
    let font = TextStyle::Body.resolve(ui.style());
    let text = crate::components::single_line(
        ui,
        &label,
        font,
        palette.text_tertiary,
        rect.width() - 80.0,
    );
    let text_pos = egui::pos2(
        icon_rect.right() + space::S,
        rect.center().y - text.size().y / 2.0,
    );
    ui.painter().galley(text_pos, text, palette.text_tertiary);
    let chip = egui::Rect::from_min_size(
        egui::pos2(rect.right() - 54.0, rect.center().y - 9.0),
        vec2(48.0, 18.0),
    );
    ui.put(chip, Kbd(shortcut_label()));
    crate::components::focus_ring(ui, &response, rect, radius::MD);
    response
}

/// Minimize, maximize, and close for client-decorated windows.
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

const fn shortcut_label() -> &'static str {
    if cfg!(target_os = "macos") {
        "⌘ K"
    } else {
        "Ctrl K"
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
