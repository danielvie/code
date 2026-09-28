use gpui::{Pixels, Point, point, px};

use crate::{Stroke, Tool};

const HANDLE_RADIUS: f32 = 9.0;

fn xy(p: Point<Pixels>) -> (f32, f32) {
    (p.x.into(), p.y.into())
}

fn distance_squared(a: Point<Pixels>, b: Point<Pixels>) -> f32 {
    let (ax, ay) = xy(a);
    let (bx, by) = xy(b);
    (ax - bx).powi(2) + (ay - by).powi(2)
}

fn corners(stroke: &Stroke) -> Option<[Point<Pixels>; 4]> {
    let [a, b, ..] = stroke.points.as_slice() else {
        return None;
    };
    let left = a.x.min(b.x);
    let right = a.x.max(b.x);
    let top = a.y.min(b.y);
    let bottom = a.y.max(b.y);
    Some([
        point(left, top),
        point(right, top),
        point(right, bottom),
        point(left, bottom),
    ])
}

pub fn handles(stroke: &Stroke) -> Vec<Point<Pixels>> {
    match stroke.tool {
        Tool::Rectangle | Tool::Ellipse => corners(stroke).map(|c| c.to_vec()).unwrap_or_default(),
        Tool::Line if stroke.points.len() == 2 => stroke.points.clone(),
        Tool::Bezier if stroke.points.len() == 4 => stroke.points.clone(),
        _ => Vec::new(),
    }
}

pub fn handle_at(stroke: &Stroke, position: Point<Pixels>) -> Option<usize> {
    handles(stroke)
        .iter()
        .enumerate()
        .rev()
        .find(|(_, handle)| distance_squared(**handle, position) <= HANDLE_RADIUS.powi(2))
        .map(|(index, _)| index)
}

pub fn opposite_corner(stroke: &Stroke, handle: usize) -> Option<Point<Pixels>> {
    match stroke.tool {
        Tool::Rectangle | Tool::Ellipse => corners(stroke).map(|corners| corners[(handle + 2) % 4]),
        _ => None,
    }
}

pub fn move_handle(
    stroke: &mut Stroke,
    handle: usize,
    position: Point<Pixels>,
    opposite: Option<Point<Pixels>>,
) {
    match stroke.tool {
        Tool::Rectangle | Tool::Ellipse => {
            if let (Some(anchor), [first, second, ..]) = (opposite, stroke.points.as_mut_slice()) {
                *first = anchor;
                *second = position;
            }
        }
        Tool::Line | Tool::Bezier => {
            if let Some(point) = stroke.points.get_mut(handle) {
                *point = position;
            }
        }
        _ => {}
    }
}

// Square distance from a point to a finite line segment.
fn point_segment_distance_squared(p: Point<Pixels>, a: Point<Pixels>, b: Point<Pixels>) -> f32 {
    let (px, py) = xy(p);
    let (ax, ay) = xy(a);
    let (bx, by) = xy(b);
    let dx = bx - ax;
    let dy = by - ay;
    let length = dx * dx + dy * dy;
    if length <= f32::EPSILON {
        return distance_squared(p, a);
    }
    let t = (((px - ax) * dx + (py - ay) * dy) / length).clamp(0., 1.);
    (px - ax - t * dx).powi(2) + (py - ay - t * dy).powi(2)
}

fn cross(a: (f32, f32), b: (f32, f32), c: (f32, f32)) -> f32 {
    (b.0 - a.0) * (c.1 - a.1) - (b.1 - a.1) * (c.0 - a.0)
}

fn segments_touch(
    a: Point<Pixels>,
    b: Point<Pixels>,
    c: Point<Pixels>,
    d: Point<Pixels>,
    radius: f32,
) -> bool {
    let (a0, b0, c0, d0) = (xy(a), xy(b), xy(c), xy(d));
    let ab_c = cross(a0, b0, c0);
    let ab_d = cross(a0, b0, d0);
    let cd_a = cross(c0, d0, a0);
    let cd_b = cross(c0, d0, b0);
    if ab_c * ab_d <= 0.
        && cd_a * cd_b <= 0.
        && (a0.0.min(b0.0) <= c0.0.max(d0.0))
        && (c0.0.min(d0.0) <= a0.0.max(b0.0))
        && (a0.1.min(b0.1) <= c0.1.max(d0.1))
        && (c0.1.min(d0.1) <= a0.1.max(b0.1))
    {
        return true;
    }
    let limit = radius * radius;
    point_segment_distance_squared(a, c, d) <= limit
        || point_segment_distance_squared(b, c, d) <= limit
        || point_segment_distance_squared(c, a, b) <= limit
        || point_segment_distance_squared(d, a, b) <= limit
}

fn lerp(a: Point<Pixels>, b: Point<Pixels>, t: f32) -> Point<Pixels> {
    point(a.x * (1. - t) + b.x * t, a.y * (1. - t) + b.y * t)
}

fn outline(stroke: &Stroke) -> Vec<Point<Pixels>> {
    match stroke.tool {
        Tool::Pencil | Tool::Line => stroke.points.clone(),
        Tool::Eraser => Vec::new(),
        Tool::Rectangle => corners(stroke)
            .map(|c| vec![c[0], c[1], c[2], c[3], c[0]])
            .unwrap_or_default(),
        Tool::Ellipse => {
            let [a, b, ..] = stroke.points.as_slice() else {
                return Vec::new();
            };
            let cx = (f32::from(a.x) + f32::from(b.x)) / 2.;
            let cy = (f32::from(a.y) + f32::from(b.y)) / 2.;
            let rx = (f32::from(b.x) - f32::from(a.x)).abs() / 2.;
            let ry = (f32::from(b.y) - f32::from(a.y)).abs() / 2.;
            (0..=96)
                .map(|i| {
                    let angle = i as f32 * std::f32::consts::TAU / 96.;
                    point(px(cx + rx * angle.cos()), px(cy + ry * angle.sin()))
                })
                .collect()
        }
        Tool::Bezier => {
            let [a, b, c, d, ..] = stroke.points.as_slice() else {
                return Vec::new();
            };
            (0..=96)
                .map(|i| {
                    let t = i as f32 / 96.;
                    let ab = lerp(*a, *b, t);
                    let bc = lerp(*b, *c, t);
                    let cd = lerp(*c, *d, t);
                    lerp(lerp(ab, bc, t), lerp(bc, cd, t), t)
                })
                .collect()
        }
    }
}

// The cursor sweep prevents a fast eraser drag from skipping narrow strokes.
pub fn touched_by_eraser(
    stroke: &Stroke,
    from: Point<Pixels>,
    to: Point<Pixels>,
    eraser_radius: f32,
) -> bool {
    let path = outline(stroke);
    let radius = stroke.width / 2. + eraser_radius;
    if path.len() == 1 {
        return point_segment_distance_squared(path[0], from, to) <= radius * radius;
    }
    path.windows(2)
        .any(|segment| segments_touch(from, to, segment[0], segment[1], radius))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn shape(tool: Tool, points: &[(f32, f32)]) -> Stroke {
        Stroke {
            tool,
            points: points.iter().map(|&(x, y)| point(px(x), px(y))).collect(),
            color: 0,
            width: 4.,
        }
    }

    #[test]
    fn rectangle_handle_moves_its_corner_and_keeps_opposite_corner_fixed() {
        let mut rect = shape(Tool::Rectangle, &[(10., 20.), (90., 80.)]);
        let opposite = opposite_corner(&rect, 0);
        move_handle(&mut rect, 0, point(px(0.), px(5.)), opposite);
        assert_eq!(
            rect.points,
            vec![point(px(90.), px(80.)), point(px(0.), px(5.))]
        );
        assert_eq!(handles(&rect)[0], point(px(0.), px(5.)));
        assert_eq!(handles(&rect)[2], point(px(90.), px(80.)));
    }

    #[test]
    fn ellipse_corner_resizes_and_bezier_control_moves() {
        let mut ellipse = shape(Tool::Ellipse, &[(10., 20.), (90., 80.)]);
        let opposite = opposite_corner(&ellipse, 1);
        move_handle(&mut ellipse, 1, point(px(110.), px(0.)), opposite);
        assert_eq!(handles(&ellipse)[1], point(px(110.), px(0.)));
        assert_eq!(handles(&ellipse)[3], point(px(10.), px(80.)));
        let mut curve = shape(Tool::Bezier, &[(0., 0.), (20., 10.), (40., 10.), (60., 0.)]);
        move_handle(&mut curve, 1, point(px(20.), px(40.)), None);
        assert_eq!(curve.points[1], point(px(20.), px(40.)));
        assert_eq!(curve.points[0], point(px(0.), px(0.)));
    }

    #[test]
    fn line_endpoints_can_be_dragged_independently() {
        let mut line = shape(Tool::Line, &[(10., 20.), (90., 80.)]);
        assert_eq!(handle_at(&line, point(px(10.), px(20.))), Some(0));
        move_handle(&mut line, 1, point(px(120.), px(40.)), None);
        assert_eq!(
            handles(&line),
            vec![point(px(10.), px(20.)), point(px(120.), px(40.))]
        );
    }

    #[test]
    fn eraser_only_removes_strokes_it_crosses() {
        let line = shape(Tool::Line, &[(10., 10.), (100., 10.)]);
        assert!(touched_by_eraser(
            &line,
            point(px(50.), px(0.)),
            point(px(50.), px(30.)),
            5.
        ));
        assert!(!touched_by_eraser(
            &line,
            point(px(50.), px(30.)),
            point(px(60.), px(30.)),
            5.
        ));
        let rect = shape(Tool::Rectangle, &[(10., 10.), (90., 90.)]);
        assert!(!touched_by_eraser(
            &rect,
            point(px(50.), px(50.)),
            point(px(50.), px(50.)),
            5.
        ));
        assert!(touched_by_eraser(
            &rect,
            point(px(50.), px(50.)),
            point(px(95.), px(50.)),
            5.
        ));
        let ellipse = shape(Tool::Ellipse, &[(10., 10.), (90., 90.)]);
        assert!(!touched_by_eraser(
            &ellipse,
            point(px(50.), px(50.)),
            point(px(50.), px(50.)),
            5.
        ));
        assert!(touched_by_eraser(
            &ellipse,
            point(px(0.), px(50.)),
            point(px(20.), px(50.)),
            5.
        ));
        let curve = shape(
            Tool::Bezier,
            &[(0., 0.), (30., 100.), (70., 100.), (100., 0.)],
        );
        assert!(touched_by_eraser(
            &curve,
            point(px(50.), px(65.)),
            point(px(50.), px(85.)),
            5.
        ));
        assert!(!touched_by_eraser(
            &curve,
            point(px(50.), px(10.)),
            point(px(50.), px(10.)),
            5.
        ));
        let dot = shape(Tool::Pencil, &[(20., 20.)]);
        assert!(touched_by_eraser(
            &dot,
            point(px(20.), px(20.)),
            point(px(20.), px(20.)),
            5.
        ));
    }

    #[test]
    fn increasing_radius_expands_hit_area() {
        let line = shape(Tool::Line, &[(0., 0.), (100., 0.)]);
        let cursor = point(px(50.), px(24.));
        assert!(!touched_by_eraser(&line, cursor, cursor, 16.));
        assert!(touched_by_eraser(&line, cursor, cursor, 24.));
    }
}
