//! A floating, draggable multi-step window: first-run tours, import flows.
//!
//! The window floats over the live app without a backdrop, so the user can drag
//! it aside (by its header) and watch a setting take effect. It has a fixed
//! size, so pages of different lengths don't make it jump; long pages scroll.
//!
//! The caller owns which step is showing and draws the page; the window draws
//! the chrome and reports what the user asked for as a [`WizardEvent`].
//! Keyboard: Escape closes; Enter or → goes forward and ← back when no control
//! has focus (a focused button takes Enter itself).

use egui::{
    Align, Context, CursorIcon, Frame, Id, ImageSource, Key, Label, Layout, Margin, Order, Pos2,
    Rect, RichText, Role, Sense, Stroke, Ui, UiBuilder, WidgetInfo, vec2,
};

use super::{Badge, Button, HoverTone, IconButton, Tone};
use crate::icons::Icon;
use crate::theme::{self, radius, space};

const WIDTH: f32 = 580.0;
const HEIGHT: f32 = 620.0;
const HEADER: f32 = 56.0;
const FOOTER: f32 = 64.0;
/// Space kept between the window and the edges of the app window.
const SCREEN_MARGIN: f32 = 16.0;
const LOGO: f32 = 24.0;

/// What the user asked the wizard to do this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WizardEvent {
    Back,
    Next,
    Close,
}

/// Translated strings for the chrome. Pass the label for the current step's
/// forward button as `next` ("Next", "Not now", "Get started").
#[derive(Clone, Copy, Debug)]
pub struct WizardText<'a> {
    pub back: &'a str,
    pub next: &'a str,
    /// Accessible name and tooltip of the × button ("Skip the tour").
    pub close: &'a str,
    /// Hint in the header ("Drag to move").
    pub drag_hint: &'a str,
    /// Screen-reader text for the step dots ("Step 1 of 3").
    pub progress: &'a str,
}

/// The wizard window. Call [`WizardWindow::show`] every frame while it is open.
#[must_use = "call `show` every frame while the wizard is open"]
pub struct WizardWindow<'a> {
    id: Id,
    title: &'a str,
    badge: Option<&'a str>,
    logo: Option<ImageSource<'a>>,
    step: usize,
    steps: usize,
    text: WizardText<'a>,
}

impl<'a> WizardWindow<'a> {
    /// `step` is zero-based; `steps` is the total.
    pub const fn new(
        id: Id,
        title: &'a str,
        step: usize,
        steps: usize,
        text: WizardText<'a>,
    ) -> Self {
        Self {
            id,
            title,
            badge: None,
            logo: None,
            step,
            steps,
            text,
        }
    }

    /// A small outlined tag after the title ("Beta", a version).
    pub const fn badge(mut self, badge: &'a str) -> Self {
        self.badge = Some(badge);
        self
    }

    /// The app logo before the title.
    pub fn logo(mut self, logo: ImageSource<'a>) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Draws the window with `add_page` as the scrolling body.
    pub fn show(self, ctx: &Context, add_page: impl FnOnce(&mut Ui)) -> Option<WizardEvent> {
        let palette = theme::palette(ctx);
        let screen = ctx.content_rect();
        let size = vec2(
            WIDTH.min(screen.width() - SCREEN_MARGIN * 2.0),
            HEIGHT.min(screen.height() - SCREEN_MARGIN * 2.0),
        )
        .max(vec2(280.0, HEADER + FOOTER + 80.0));
        let pos_id = self.id.with("position");
        let stored: Option<Pos2> = ctx.data(|data| data.get_temp(pos_id));
        let mut pos = clamp_position(
            stored.unwrap_or_else(|| screen.center() - size / 2.0),
            size,
            screen,
        );

        let mut event = keyboard_event(ctx, self.step);
        let shadow = ctx.global_style().visuals.window_shadow;
        egui::Area::new(self.id)
            .order(Order::Foreground)
            .fixed_pos(pos)
            .show(ctx, |ui| {
                Frame::new()
                    .fill(palette.surface_raised)
                    .stroke(Stroke::new(1.0, palette.border))
                    .corner_radius(radius::LG)
                    .shadow(shadow)
                    .show(ui, |ui| {
                        ui.set_min_size(size);
                        ui.set_max_size(size);
                        let spacing = ui.spacing().item_spacing;
                        ui.spacing_mut().item_spacing = vec2(0.0, 0.0);

                        let (header, _) =
                            ui.allocate_exact_size(vec2(size.x, HEADER), Sense::hover());
                        let drag = ui.interact(header, self.id.with("drag"), Sense::drag());
                        if drag.dragged() {
                            pos += drag.drag_delta();
                            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
                        } else if drag.hovered() {
                            ui.ctx().set_cursor_icon(CursorIcon::Grab);
                        }
                        if self.header(ui, header) {
                            event = Some(WizardEvent::Close);
                        }
                        hairline(ui, header.bottom(), header.x_range());

                        let body_height = size.y - HEADER - FOOTER;
                        let (body, _) =
                            ui.allocate_exact_size(vec2(size.x, body_height), Sense::hover());
                        ui.scope_builder(UiBuilder::new().max_rect(body), |ui| {
                            ui.spacing_mut().item_spacing = spacing;
                            egui::ScrollArea::vertical()
                                .id_salt(self.id.with("body"))
                                .auto_shrink([false, false])
                                .show(ui, |ui| {
                                    Frame::new()
                                        .inner_margin(Margin {
                                            left: 24,
                                            right: 24,
                                            top: 22,
                                            bottom: 20,
                                        })
                                        .show(ui, |ui| {
                                            ui.set_width(ui.available_width());
                                            add_page(ui);
                                        });
                                });
                        });

                        let (footer, _) =
                            ui.allocate_exact_size(vec2(size.x, FOOTER), Sense::hover());
                        hairline(ui, footer.top(), footer.x_range());
                        if let Some(footer_event) = self.footer(ui, footer) {
                            event = Some(footer_event);
                        }
                    });
            });

        ctx.data_mut(|data| data.insert_temp(pos_id, pos));
        event
    }

    /// Logo, title, badge, drag hint, close button. Returns whether × was clicked.
    fn header(&self, ui: &mut Ui, rect: Rect) -> bool {
        let palette = theme::palette(ui.ctx());
        let inner = rect.shrink2(vec2(space::XL - 4.0, 0.0));
        let mut close = false;
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                ui.spacing_mut().item_spacing.x = space::M - 2.0;
                if let Some(logo) = self.logo.clone() {
                    ui.add(
                        egui::Image::new(logo)
                            .fit_to_exact_size(vec2(LOGO, LOGO))
                            .corner_radius(radius::MD),
                    );
                }
                // Not selectable: a press on the title must drag the window.
                ui.add(
                    Label::new(RichText::new(self.title).font(theme::semibold(15.0)))
                        .selectable(false),
                );
                if let Some(badge) = self.badge {
                    ui.add(Badge::new(badge, Tone::Neutral));
                }
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    close = ui
                        .add(
                            IconButton::new(Icon::Close, self.text.close)
                                .hover_tone(HoverTone::Neutral),
                        )
                        .clicked();
                    ui.add(
                        Label::new(
                            RichText::new(self.text.drag_hint)
                                .small()
                                .color(palette.text_tertiary),
                        )
                        .selectable(false),
                    );
                });
            },
        );
        close
    }

    /// Step dots on the left, Back and Next on the right.
    fn footer(&self, ui: &mut Ui, rect: Rect) -> Option<WizardEvent> {
        let mut event = None;
        let inner = rect.shrink2(vec2(space::XL - 4.0, 0.0));
        ui.scope_builder(
            UiBuilder::new()
                .max_rect(inner)
                .layout(Layout::left_to_right(Align::Center)),
            |ui| {
                step_dots(ui, self.step, self.steps, self.text.progress);
                ui.with_layout(Layout::right_to_left(Align::Center), |ui| {
                    ui.spacing_mut().item_spacing.x = space::S;
                    if ui
                        .add(Button::primary(self.text.next).trailing_icon(Icon::ArrowRight))
                        .clicked()
                    {
                        event = Some(WizardEvent::Next);
                    }
                    if self.step > 0 && ui.add(Button::ghost(self.text.back)).clicked() {
                        event = Some(WizardEvent::Back);
                    }
                });
            },
        );
        event
    }
}

/// Escape, Enter, and the arrow keys. Enter and arrows only act when no
/// control has focus, so they never fight a focused button or text field.
fn keyboard_event(ctx: &Context, step: usize) -> Option<WizardEvent> {
    if ctx.input_mut(|input| input.consume_key(egui::Modifiers::NONE, Key::Escape)) {
        return Some(WizardEvent::Close);
    }
    if ctx.memory(|memory| memory.focused().is_some()) {
        return None;
    }
    ctx.input_mut(|input| {
        if input.consume_key(egui::Modifiers::NONE, Key::Enter)
            || input.consume_key(egui::Modifiers::NONE, Key::ArrowRight)
        {
            Some(WizardEvent::Next)
        } else if step > 0 && input.consume_key(egui::Modifiers::NONE, Key::ArrowLeft) {
            Some(WizardEvent::Back)
        } else {
            None
        }
    })
}

/// Keeps the whole window on screen.
fn clamp_position(pos: Pos2, size: egui::Vec2, screen: Rect) -> Pos2 {
    let min = screen.min + vec2(SCREEN_MARGIN, SCREEN_MARGIN);
    let max = (screen.max - size - vec2(SCREEN_MARGIN, SCREEN_MARGIN)).max(min);
    pos.clamp(min, max)
}

fn hairline(ui: &Ui, y: f32, x: egui::Rangef) {
    let palette = theme::palette(ui.ctx());
    ui.painter().hline(x, y, Stroke::new(1.0, palette.border));
}

/// A pill for the current step and dots for the rest.
fn step_dots(ui: &mut Ui, step: usize, steps: usize, progress: &str) {
    const DOT: f32 = 6.0;
    const ACTIVE: f32 = 18.0;
    const GAP: f32 = 5.0;
    let palette = theme::palette(ui.ctx());
    #[expect(clippy::cast_precision_loss, reason = "a handful of steps")]
    let width = ACTIVE + (DOT + GAP) * steps.saturating_sub(1) as f32;
    let (rect, response) = ui.allocate_exact_size(vec2(width, DOT), Sense::hover());
    response.widget_info(|| WidgetInfo::labeled(Role::Label, true, progress));
    let mut x = rect.left();
    for index in 0..steps {
        let (w, color) = if index == step {
            (ACTIVE, palette.text)
        } else {
            (DOT, palette.border_strong)
        };
        let dot = Rect::from_min_size(egui::pos2(x, rect.top()), vec2(w, DOT));
        ui.painter().rect_filled(dot, DOT / 2.0, color);
        x += w + GAP;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn window_stays_on_screen() {
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(1000.0, 700.0));
        let size = vec2(580.0, 620.0);
        let clamped = clamp_position(egui::pos2(900.0, -50.0), size, screen);
        assert_eq!(clamped, egui::pos2(1000.0 - 580.0 - 16.0, 16.0));
    }

    #[test]
    fn tiny_screens_pin_the_window_to_the_corner() {
        let screen = Rect::from_min_size(Pos2::ZERO, vec2(300.0, 300.0));
        let clamped = clamp_position(egui::pos2(100.0, 100.0), vec2(580.0, 620.0), screen);
        assert_eq!(clamped, egui::pos2(16.0, 16.0));
    }
}
