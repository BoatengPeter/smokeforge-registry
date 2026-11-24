//! A floating, draggable multi-step window: first-run tours, import flows.
//!
//! The window floats over the live app without a backdrop, so the user can drag
//! it aside by its header and watch a setting take effect. It has a fixed size,
//! so pages of different lengths don't make it jump; long pages scroll.
//!
//! Split of responsibilities:
//! - [`WizardWindow`] draws the chrome: header (logo, title, badge, drag hint,
//!   close), scrolling body, footer (step dots, back, next).
//! - [`WindowDrag`] is the drag state. The owning view keeps one, starts it from
//!   [`WizardWindow::on_drag_start`], and feeds it mouse moves and the mouse-up
//!   from its root element (moves outside the header must still drag).
//! - The caller builds the page body and the buttons (with its own listeners),
//!   and handles keys (Escape, Enter, arrows) on its focus handle.

use std::sync::Arc;

use gpui_kit::base::{StyledExt, h_flex, v_flex};
use gpui_kit::component::ActiveTheme;
use gpui_kit::prelude::FluentBuilder as _;
use gpui_kit::*;

/// Window size in rems at 100% interface size.
const WIDTH_REMS: f32 = 52.0;
const HEIGHT_REMS: f32 = 46.0;
/// Space kept between the window and the edges of the app window, in pixels.
const SCREEN_MARGIN: f32 = 16.0;

/// The wizard's size for a viewport: its natural size, shrunk to fit.
#[must_use]
pub fn wizard_size(viewport: Size<Pixels>, rem: Pixels) -> Size<Pixels> {
    let rem = rem.as_f32();
    let width = (WIDTH_REMS * rem).min(viewport.width.as_f32() - SCREEN_MARGIN * 2.0);
    let height = (HEIGHT_REMS * rem).min(viewport.height.as_f32() - SCREEN_MARGIN * 2.0);
    size(px(width.max(240.0)), px(height.max(240.0)))
}

/// Where the user has dragged the window, as an offset from the centre.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WindowDrag {
    offset: Point<Pixels>,
    /// The pointer position relative to the offset while a drag is running.
    grab: Option<Point<Pixels>>,
}

impl WindowDrag {
    #[must_use]
    pub const fn offset(&self) -> Point<Pixels> {
        self.offset
    }

    #[must_use]
    pub const fn is_dragging(&self) -> bool {
        self.grab.is_some()
    }

    /// Starts a drag with the pointer at `pointer`.
    pub fn start(&mut self, pointer: Point<Pixels>) {
        self.grab = Some(pointer - self.offset);
    }

    /// Moves the window under the pointer, keeping it inside `viewport`.
    /// Returns whether anything moved (the caller then calls `cx.notify()`).
    pub fn drag_to(
        &mut self,
        pointer: Point<Pixels>,
        viewport: Size<Pixels>,
        window: Size<Pixels>,
    ) -> bool {
        let Some(grab) = self.grab else {
            return false;
        };
        let next = clamp_offset(pointer - grab, viewport, window);
        let moved = next != self.offset;
        self.offset = next;
        moved
    }

    /// Ends a drag. Returns whether one was running.
    pub fn end(&mut self) -> bool {
        self.grab.take().is_some()
    }

    /// Forgets the position, so the window opens centred next time.
    pub fn reset(&mut self) {
        *self = Self::default();
    }
}

/// Keeps a centred window of size `window` fully inside `viewport`.
fn clamp_offset(
    offset: Point<Pixels>,
    viewport: Size<Pixels>,
    window: Size<Pixels>,
) -> Point<Pixels> {
    let limit_x =
        ((viewport.width.as_f32() - window.width.as_f32()) / 2.0 - SCREEN_MARGIN).max(0.0);
    let limit_y =
        ((viewport.height.as_f32() - window.height.as_f32()) / 2.0 - SCREEN_MARGIN).max(0.0);
    point(
        px(offset.x.as_f32().clamp(-limit_x, limit_x)),
        px(offset.y.as_f32().clamp(-limit_y, limit_y)),
    )
}

type DragStart = Box<dyn Fn(&MouseDownEvent, &mut Window, &mut App) + 'static>;

/// The wizard chrome. Place it as the last child of a `relative()` root.
#[derive(IntoElement)]
pub struct WizardWindow {
    id: ElementId,
    title: SharedString,
    badge: Option<SharedString>,
    logo: Option<Arc<Image>>,
    drag_hint: SharedString,
    progress: SharedString,
    step: usize,
    steps: usize,
    offset: Point<Pixels>,
    size: Size<Pixels>,
    body: Option<AnyElement>,
    close: Option<AnyElement>,
    back: Option<AnyElement>,
    next: Option<AnyElement>,
    on_drag_start: Option<DragStart>,
}

impl WizardWindow {
    /// `step` is zero-based. `progress` is the screen-reader text for the
    /// step dots ("Step 1 of 3"). `size` comes from [`wizard_size`].
    #[must_use]
    pub fn new(
        id: impl Into<ElementId>,
        title: impl Into<SharedString>,
        step: usize,
        steps: usize,
        size: Size<Pixels>,
    ) -> Self {
        Self {
            id: id.into(),
            title: title.into(),
            badge: None,
            logo: None,
            drag_hint: SharedString::default(),
            progress: SharedString::default(),
            step,
            steps,
            offset: Point::default(),
            size,
            body: None,
            close: None,
            back: None,
            next: None,
            on_drag_start: None,
        }
    }

    /// A small outlined tag after the title ("Beta", a version).
    #[must_use]
    pub fn badge(mut self, badge: impl Into<SharedString>) -> Self {
        self.badge = Some(badge.into());
        self
    }

    #[must_use]
    pub fn logo(mut self, logo: Arc<Image>) -> Self {
        self.logo = Some(logo);
        self
    }

    /// Header hint ("Drag to move") and the step-dots description.
    #[must_use]
    pub fn labels(
        mut self,
        drag_hint: impl Into<SharedString>,
        progress: impl Into<SharedString>,
    ) -> Self {
        self.drag_hint = drag_hint.into();
        self.progress = progress.into();
        self
    }

    /// The drag offset from [`WindowDrag::offset`].
    #[must_use]
    pub const fn offset(mut self, offset: Point<Pixels>) -> Self {
        self.offset = offset;
        self
    }

    /// The current page.
    #[must_use]
    pub fn body(mut self, body: impl IntoElement) -> Self {
        self.body = Some(body.into_any_element());
        self
    }

    /// The × button in the header.
    #[must_use]
    pub fn close(mut self, close: impl IntoElement) -> Self {
        self.close = Some(close.into_any_element());
        self
    }

    /// The footer's back button; omit on the first step.
    #[must_use]
    pub fn back(mut self, back: impl IntoElement) -> Self {
        self.back = Some(back.into_any_element());
        self
    }

    /// The footer's primary button ("Next", "Get started").
    #[must_use]
    pub fn next(mut self, next: impl IntoElement) -> Self {
        self.next = Some(next.into_any_element());
        self
    }

    /// Called when the header is pressed: start the owner's [`WindowDrag`].
    #[must_use]
    pub fn on_drag_start(
        mut self,
        handler: impl Fn(&MouseDownEvent, &mut Window, &mut App) + 'static,
    ) -> Self {
        self.on_drag_start = Some(Box::new(handler));
        self
    }
}

impl RenderOnce for WizardWindow {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let theme = cx.theme();
        let header = h_flex()
            .id(self.id.clone())
            .flex_none()
            .h(rems(3.5))
            .pl_5()
            .pr_3()
            .gap_2p5()
            .items_center()
            .border_b_1()
            .border_color(theme.border)
            .cursor_grab()
            .when_some(self.on_drag_start, |this, handler| {
                this.on_mouse_down(MouseButton::Left, handler)
            })
            .when_some(self.logo, |this, logo| {
                this.child(img(logo).size_6().rounded(theme.radius).flex_none())
            })
            .child(div().text_base().font_semibold().child(self.title))
            .when_some(self.badge, |this, badge| {
                this.child(
                    div()
                        .px_1p5()
                        .rounded(theme.radius)
                        .border_1()
                        .border_color(theme.input)
                        .text_xs()
                        .font_semibold()
                        .text_color(theme.muted_foreground)
                        .child(badge),
                )
            })
            .child(div().flex_1())
            .child(
                div()
                    .text_xs()
                    .text_color(theme.muted_foreground)
                    .child(self.drag_hint),
            )
            .children(self.close.map(|close| {
                // A press on the button must not start a drag.
                div()
                    .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                    .child(close)
            }));

        let body = div().relative().flex_1().min_h_0().child(
            div()
                .id("wizard-body")
                .size_full()
                .overflow_y_scroll()
                .child(v_flex().px_6().pt_6().pb_5().gap_5().children(self.body)),
        );

        let footer = h_flex()
            .flex_none()
            .h(rems(4.))
            .px_5()
            .gap_2()
            .items_center()
            .border_t_1()
            .border_color(theme.border)
            .child(step_dots(self.step, self.steps, self.progress, cx))
            .child(div().flex_1())
            .children(self.back)
            .children(self.next);

        // Full-window layer that centres the window; it only paints the window
        // itself, so the app behind stays visible and clickable.
        div()
            .absolute()
            .inset_0()
            .flex()
            .items_center()
            .justify_center()
            .child(
                v_flex()
                    .occlude()
                    .relative()
                    .left(self.offset.x)
                    .top(self.offset.y)
                    .w(self.size.width)
                    .h(self.size.height)
                    .bg(theme.popover)
                    .text_color(theme.foreground)
                    .border_1()
                    .border_color(theme.border)
                    .rounded(theme.radius_lg)
                    .shadow_xl()
                    .overflow_hidden()
                    .child(header)
                    .child(body)
                    .child(footer),
            )
    }
}

/// A pill for the current step and dots for the rest.
fn step_dots(step: usize, steps: usize, progress: SharedString, cx: &App) -> impl IntoElement {
    let theme = cx.theme();
    h_flex()
        .gap_1()
        .pl_1()
        .debug_selector(move || progress.to_string())
        .children((0..steps).map(|index| {
            let current = index == step;
            div()
                .h_1p5()
                .rounded_full()
                .when(current, |this| this.w_4().bg(theme.foreground))
                .when(!current, |this| this.w_1p5().bg(theme.input))
        }))
}

#[cfg(test)]
mod tests {
    // Explicit imports: `gpui_kit::*` brings GPUI's `test` macro, which
    // shadows the built-in `#[test]`.
    use super::{WindowDrag, wizard_size};
    use gpui_kit::{Pixels, Size, point, px, size};

    fn viewport() -> Size<Pixels> {
        size(px(1000.0), px(700.0))
    }

    #[test]
    fn size_shrinks_to_fit_small_windows() {
        // 52 × 46 rem at 16 px, height capped by the 700 px viewport.
        let natural = wizard_size(viewport(), px(16.0));
        assert_eq!(natural, size(px(832.0), px(668.0)));
        let roomy = wizard_size(size(px(1600.0), px(1000.0)), px(16.0));
        assert_eq!(roomy, size(px(832.0), px(736.0)));
        let small = wizard_size(size(px(400.0), px(300.0)), px(16.0));
        assert_eq!(small, size(px(368.0), px(268.0)));
    }

    #[test]
    fn drag_follows_the_pointer_and_stays_on_screen() {
        let window = size(px(576.0), px(600.0));
        let mut drag = WindowDrag::default();
        assert!(!drag.drag_to(point(px(10.0), px(10.0)), viewport(), window));

        drag.start(point(px(500.0), px(40.0)));
        assert!(drag.drag_to(point(px(560.0), px(50.0)), viewport(), window));
        assert_eq!(drag.offset(), point(px(60.0), px(10.0)));

        // Far right and down: pinned to the margin.
        drag.drag_to(point(px(5000.0), px(5000.0)), viewport(), window);
        assert_eq!(drag.offset(), point(px(196.0), px(34.0)));

        assert!(drag.end());
        assert!(!drag.is_dragging());
        assert!(!drag.drag_to(point(px(0.0), px(0.0)), viewport(), window));
    }
}
