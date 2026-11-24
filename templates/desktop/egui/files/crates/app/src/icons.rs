//! The app's icon set, drawn in code on a 16×16 grid.
//!
//! Icons are line drawings built from egui shapes, so they scale to any size,
//! take the theme's colours, and need no image files or icon fonts. Each icon is
//! described in grid units (0–16, y down) and mapped onto the target rectangle.
//! To add one: add a variant, list it in [`Icon::ALL`], and describe it in
//! [`shapes`]. The gallery test checks that every icon draws inside its box.

use std::f32::consts::TAU;

use egui::{Color32, Painter, Pos2, Rect, Shape, Stroke, pos2, vec2};

/// Stroke width in grid units; 1.5 points at the standard 16-point size.
const STROKE: f32 = 1.5;
/// Arc smoothness: segments for a full circle.
const CIRCLE_SEGMENTS: f32 = 40.0;

/// Every icon the UI uses.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Icon {
    Dashboard,
    Library,
    Activity,
    Settings,
    Account,
    About,
    Search,
    Plus,
    Trash,
    Sun,
    Moon,
    Monitor,
    Terminal,
    PanelLeft,
    Minimize,
    Maximize,
    Close,
    Check,
    Sync,
    Database,
    Download,
    Upload,
    Undo,
    ArrowRight,
    Keyboard,
    Minus,
}

impl Icon {
    /// Every icon, for galleries and tests.
    pub const ALL: [Self; 26] = [
        Self::Dashboard,
        Self::Library,
        Self::Activity,
        Self::Settings,
        Self::Account,
        Self::About,
        Self::Search,
        Self::Plus,
        Self::Trash,
        Self::Sun,
        Self::Moon,
        Self::Monitor,
        Self::Terminal,
        Self::PanelLeft,
        Self::Minimize,
        Self::Maximize,
        Self::Close,
        Self::Check,
        Self::Sync,
        Self::Database,
        Self::Download,
        Self::Upload,
        Self::Undo,
        Self::ArrowRight,
        Self::Keyboard,
        Self::Minus,
    ];
}

/// Paints `icon` in `color`, filling the largest square inside `rect`.
pub fn paint(painter: &Painter, rect: Rect, icon: Icon, color: Color32) {
    painter.extend(shapes(icon, square_in(rect), color));
}

/// The shapes that draw `icon` inside the square `rect`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn shapes(icon: Icon, rect: Rect, color: Color32) -> Vec<Shape> {
    let mut pen = Pen::new(rect, color);
    match icon {
        Icon::Dashboard => {
            pen.rect(2.0, 2.0, 7.0, 9.0);
            pen.rect(9.0, 2.0, 14.0, 6.0);
            pen.rect(9.0, 8.0, 14.0, 14.0);
            pen.rect(2.0, 11.0, 7.0, 14.0);
        }
        Icon::Library => {
            pen.line(&[(8.0, 4.5), (8.0, 13.5)]);
            pen.line(&[
                (8.0, 4.5),
                (6.0, 3.0),
                (2.0, 3.0),
                (2.0, 12.0),
                (6.0, 12.0),
                (8.0, 13.5),
            ]);
            pen.line(&[
                (8.0, 4.5),
                (10.0, 3.0),
                (14.0, 3.0),
                (14.0, 12.0),
                (10.0, 12.0),
                (8.0, 13.5),
            ]);
        }
        Icon::Activity => {
            pen.line(&[
                (1.5, 8.0),
                (4.5, 8.0),
                (6.5, 3.0),
                (9.5, 13.0),
                (11.5, 8.0),
                (14.5, 8.0),
            ]);
        }
        Icon::Settings => {
            pen.closed(&gear_points());
            pen.circle(8.0, 8.0, 2.2);
        }
        Icon::Account => {
            pen.circle(8.0, 8.0, 6.5);
            pen.circle(8.0, 6.6, 2.3);
            pen.arc(8.0, 14.2, 4.3, 205.0, 335.0);
        }
        Icon::About => {
            pen.circle(8.0, 8.0, 6.5);
            pen.dot(8.0, 5.0, 0.95);
            pen.line(&[(8.0, 7.5), (8.0, 11.2)]);
        }
        Icon::Search => {
            pen.circle(7.0, 7.0, 4.5);
            pen.line(&[(10.4, 10.4), (14.0, 14.0)]);
        }
        Icon::Plus => {
            pen.line(&[(8.0, 3.0), (8.0, 13.0)]);
            pen.line(&[(3.0, 8.0), (13.0, 8.0)]);
        }
        Icon::Trash => {
            pen.line(&[(2.5, 4.5), (13.5, 4.5)]);
            pen.line(&[(6.0, 4.5), (6.0, 2.5), (10.0, 2.5), (10.0, 4.5)]);
            pen.line(&[(3.8, 4.5), (4.6, 13.5), (11.4, 13.5), (12.2, 4.5)]);
        }
        Icon::Sun => {
            pen.circle(8.0, 8.0, 2.8);
            for step in 0..8_u8 {
                let angle = f32::from(step) * TAU / 8.0;
                let (sin, cos) = angle.sin_cos();
                pen.line(&[
                    (8.0 + 4.8 * cos, 8.0 + 4.8 * sin),
                    (8.0 + 6.5 * cos, 8.0 + 6.5 * sin),
                ]);
            }
        }
        Icon::Moon => pen.closed(&crescent_points()),
        Icon::Monitor => {
            pen.rect(1.5, 2.5, 14.5, 11.0);
            pen.line(&[(8.0, 11.0), (8.0, 13.5)]);
            pen.line(&[(5.0, 13.5), (11.0, 13.5)]);
        }
        Icon::Terminal => {
            pen.line(&[(3.0, 5.0), (6.5, 8.0), (3.0, 11.0)]);
            pen.line(&[(8.5, 11.0), (13.0, 11.0)]);
        }
        Icon::PanelLeft => {
            pen.rect(1.5, 2.5, 14.5, 13.5);
            pen.line(&[(6.0, 2.5), (6.0, 13.5)]);
        }
        Icon::Minimize => pen.line(&[(3.5, 8.0), (12.5, 8.0)]),
        Icon::Maximize => pen.rect(3.5, 3.5, 12.5, 12.5),
        Icon::Close => {
            pen.line(&[(4.0, 4.0), (12.0, 12.0)]);
            pen.line(&[(12.0, 4.0), (4.0, 12.0)]);
        }
        Icon::Check => pen.line(&[(3.0, 8.5), (6.5, 12.0), (13.0, 4.5)]),
        Icon::Sync => {
            pen.arc(8.0, 8.0, 5.5, -60.0, 225.0);
            pen.line(&[(1.9, 8.6), (4.1, 4.1), (6.2, 7.1)]);
        }
        Icon::Database => {
            pen.ellipse_arc(8.0, 4.0, 5.5, 2.0, 0.0, 360.0);
            pen.ellipse_arc(8.0, 8.0, 5.5, 2.0, 0.0, 180.0);
            pen.ellipse_arc(8.0, 12.0, 5.5, 2.0, 0.0, 180.0);
            pen.line(&[(2.5, 4.0), (2.5, 12.0)]);
            pen.line(&[(13.5, 4.0), (13.5, 12.0)]);
        }
        Icon::Download => {
            pen.line(&[(8.0, 2.5), (8.0, 10.0)]);
            pen.line(&[(4.5, 6.5), (8.0, 10.0), (11.5, 6.5)]);
            pen.line(&[(2.5, 10.5), (2.5, 13.5), (13.5, 13.5), (13.5, 10.5)]);
        }
        Icon::Upload => {
            pen.line(&[(8.0, 10.0), (8.0, 2.5)]);
            pen.line(&[(4.5, 6.0), (8.0, 2.5), (11.5, 6.0)]);
            pen.line(&[(2.5, 10.5), (2.5, 13.5), (13.5, 13.5), (13.5, 10.5)]);
        }
        Icon::Undo => {
            pen.line(&[(5.0, 3.5), (2.5, 6.0), (5.0, 8.5)]);
            pen.line(&[(2.5, 6.0), (9.5, 6.0)]);
            pen.arc(9.5, 9.5, 3.5, -90.0, 90.0);
            pen.line(&[(9.5, 13.0), (6.0, 13.0)]);
        }
        Icon::ArrowRight => {
            pen.line(&[(3.0, 8.0), (13.0, 8.0)]);
            pen.line(&[(9.0, 4.0), (13.0, 8.0), (9.0, 12.0)]);
        }
        Icon::Keyboard => {
            pen.rect(1.5, 4.0, 14.5, 12.0);
            for x in [4.5, 7.0, 9.0, 11.5] {
                pen.dot(x, 6.8, 0.6);
            }
            pen.line(&[(5.0, 9.5), (11.0, 9.5)]);
        }
        Icon::Minus => pen.line(&[(3.0, 8.0), (13.0, 8.0)]),
    }
    pen.finish()
}

/// The largest square centred in `rect`.
fn square_in(rect: Rect) -> Rect {
    let side = rect.width().min(rect.height());
    Rect::from_center_size(rect.center(), vec2(side, side))
}

/// Gear outline: eight flat teeth around a ring.
fn gear_points() -> Vec<(f32, f32)> {
    const TEETH: u8 = 8;
    const OUTER: f32 = 6.6;
    const INNER: f32 = 5.0;
    let mut points = Vec::with_capacity(usize::from(TEETH) * 4);
    for tooth in 0..TEETH {
        let center = f32::from(tooth) * 360.0 / f32::from(TEETH);
        for (offset, radius) in [(-17.0, INNER), (-9.0, OUTER), (9.0, OUTER), (17.0, INNER)] {
            let (sin, cos) = (center + offset).to_radians().sin_cos();
            points.push((8.0 + radius * cos, 8.0 + radius * sin));
        }
    }
    points
}

/// Crescent moon: the part of a disc not covered by a second, offset disc.
fn crescent_points() -> Vec<(f32, f32)> {
    const OUTER: (f32, f32, f32) = (8.0, 8.5, 5.8);
    const CUT: (f32, f32, f32) = (11.2, 5.4, 4.6);
    let Some((first, second)) = circle_intersections(OUTER, CUT) else {
        return arc_points(OUTER.0, OUTER.1, OUTER.2, OUTER.2, 0.0, 360.0);
    };
    let angle = |center: (f32, f32, f32), point: (f32, f32)| {
        (point.1 - center.1).atan2(point.0 - center.0).to_degrees()
    };
    // Outer edge: the long way round, away from the cut.
    let (mut start, mut end) = (angle(OUTER, first), angle(OUTER, second));
    if end < start {
        end += 360.0;
    }
    if end - start < 180.0 {
        std::mem::swap(&mut start, &mut end);
        end += 360.0;
    }
    let mut points = arc_points(OUTER.0, OUTER.1, OUTER.2, OUTER.2, start, end);
    // Inner edge: back along the cut circle, the short way, inside the disc.
    let from = angle(CUT, *points.last().unwrap_or(&second));
    let mut to = angle(CUT, points[0]);
    while to > from {
        to -= 360.0;
    }
    if from - to > 180.0 {
        to += 360.0;
    }
    points.extend(arc_points(CUT.0, CUT.1, CUT.2, CUT.2, from, to));
    points
}

/// Where two circles `(x, y, r)` cross, if they do.
fn circle_intersections(
    a: (f32, f32, f32),
    b: (f32, f32, f32),
) -> Option<((f32, f32), (f32, f32))> {
    let (dx, dy) = (b.0 - a.0, b.1 - a.1);
    let distance = dx.hypot(dy);
    if distance > a.2 + b.2 || distance < (a.2 - b.2).abs() || distance == 0.0 {
        return None;
    }
    let along = (a.2 * a.2 - b.2 * b.2 + distance * distance) / (2.0 * distance);
    let half_chord = (a.2 * a.2 - along * along).max(0.0).sqrt();
    let (mx, my) = (a.0 + along * dx / distance, a.1 + along * dy / distance);
    let (ox, oy) = (-dy * half_chord / distance, dx * half_chord / distance);
    Some(((mx + ox, my + oy), (mx - ox, my - oy)))
}

/// Points along an elliptical arc, clockwise (y down) from `from` to `to` degrees.
fn arc_points(cx: f32, cy: f32, rx: f32, ry: f32, from: f32, to: f32) -> Vec<(f32, f32)> {
    let span = to - from;
    let steps = (CIRCLE_SEGMENTS * span.abs() / 360.0).ceil().max(2.0);
    // `steps` is a small positive whole number, so the conversion is exact.
    #[expect(
        clippy::cast_possible_truncation,
        clippy::cast_sign_loss,
        reason = "small positive count"
    )]
    let count = steps as u16;
    (0..=count)
        .map(|step| {
            let angle = (from + span * f32::from(step) / steps).to_radians();
            (cx + rx * angle.cos(), cy + ry * angle.sin())
        })
        .collect()
}

/// Maps grid coordinates onto a rectangle and collects stroked shapes.
struct Pen {
    origin: Pos2,
    scale: f32,
    stroke: Stroke,
    color: Color32,
    shapes: Vec<Shape>,
}

impl Pen {
    fn new(rect: Rect, color: Color32) -> Self {
        let scale = rect.width() / 16.0;
        Self {
            origin: rect.min,
            scale,
            stroke: Stroke::new((STROKE * scale).max(1.0), color),
            color,
            shapes: Vec::new(),
        }
    }

    fn at(&self, (x, y): (f32, f32)) -> Pos2 {
        pos2(
            self.origin.x + x * self.scale,
            self.origin.y + y * self.scale,
        )
    }

    fn line(&mut self, points: &[(f32, f32)]) {
        let points = points.iter().map(|point| self.at(*point)).collect();
        self.shapes.push(Shape::line(points, self.stroke));
    }

    fn closed(&mut self, points: &[(f32, f32)]) {
        let points = points.iter().map(|point| self.at(*point)).collect();
        self.shapes.push(Shape::closed_line(points, self.stroke));
    }

    fn rect(&mut self, left: f32, top: f32, right: f32, bottom: f32) {
        let rect = Rect::from_min_max(self.at((left, top)), self.at((right, bottom)));
        self.shapes.push(Shape::rect_stroke(
            rect,
            1.5 * self.scale,
            self.stroke,
            egui::StrokeKind::Middle,
        ));
    }

    fn circle(&mut self, cx: f32, cy: f32, radius: f32) {
        self.shapes.push(Shape::circle_stroke(
            self.at((cx, cy)),
            radius * self.scale,
            self.stroke,
        ));
    }

    fn dot(&mut self, cx: f32, cy: f32, radius: f32) {
        self.shapes.push(Shape::circle_filled(
            self.at((cx, cy)),
            radius * self.scale,
            self.color,
        ));
    }

    fn arc(&mut self, cx: f32, cy: f32, radius: f32, from: f32, to: f32) {
        self.ellipse_arc(cx, cy, radius, radius, from, to);
    }

    fn ellipse_arc(&mut self, cx: f32, cy: f32, rx: f32, ry: f32, from: f32, to: f32) {
        let points = arc_points(cx, cy, rx, ry, from, to);
        self.line(&points);
    }

    fn finish(self) -> Vec<Shape> {
        self.shapes
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_icon_draws_inside_its_box_at_common_sizes() {
        for side in [12.0, 16.0, 20.0, 32.0] {
            let rect = Rect::from_min_size(pos2(10.0, 20.0), vec2(side, side));
            let allowed = rect.expand(STROKE * side / 16.0);
            for icon in Icon::ALL {
                let drawn = shapes(icon, rect, Color32::WHITE);
                assert!(!drawn.is_empty(), "{icon:?} draws nothing");
                for shape in &drawn {
                    let bounds = shape.visual_bounding_rect();
                    assert!(
                        allowed.contains_rect(bounds),
                        "{icon:?} at {side}pt draws {bounds:?} outside {allowed:?}"
                    );
                }
            }
        }
    }

    #[test]
    fn non_square_boxes_draw_a_centred_square_icon() {
        let wide = Rect::from_min_size(Pos2::ZERO, vec2(40.0, 16.0));
        let square = square_in(wide);
        assert_eq!(square.size(), vec2(16.0, 16.0));
        assert_eq!(square.center(), wide.center());
    }

    #[test]
    fn crescent_is_a_closed_outline_inside_the_grid() {
        let points = crescent_points();
        assert!(points.len() > 10, "crescent has {} points", points.len());
        for (x, y) in points {
            assert!(
                (0.0..=16.0).contains(&x) && (0.0..=16.0).contains(&y),
                "({x}, {y}) off the grid"
            );
        }
    }
}
