mod geometry;

use std::{cell::Cell, rc::Rc};

use gpui::{
    Application, Bounds, Context, FocusHandle, KeyDownEvent, MouseButton, MouseDownEvent,
    MouseMoveEvent, MouseUpEvent, PathBuilder, Pixels, Point, Render, Window, WindowBounds,
    WindowOptions, canvas, div, point, prelude::*, px, quad, rgb, size,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Tool {
    Pencil,
    Line,
    Rectangle,
    Ellipse,
    Bezier,
    Eraser,
}

impl Tool {
    fn label(self) -> &'static str {
        match self {
            Self::Pencil => "Pencil",
            Self::Line => "Line",
            Self::Rectangle => "Rectangle",
            Self::Ellipse => "Ellipse",
            Self::Bezier => "Bezier",
            Self::Eraser => "Eraser",
        }
    }
}

const TOOLS: [Tool; 6] = [
    Tool::Pencil,
    Tool::Line,
    Tool::Rectangle,
    Tool::Ellipse,
    Tool::Bezier,
    Tool::Eraser,
];
const COLORS: [(&str, u32); 8] = [
    ("Ink", 0x202938),
    ("Slate", 0x64748b),
    ("Red", 0xe34b55),
    ("Orange", 0xf28c45),
    ("Yellow", 0xe7ba45),
    ("Green", 0x31a780),
    ("Blue", 0x4e81da),
    ("Purple", 0x9b6bd4),
];
const WIDTHS: [f32; 5] = [2., 4., 8., 12., 20.];
const ERASER_RADIUS_STEP: f32 = 4.;
const ERASER_RADIUS_MIN: f32 = 2.;
const ERASER_RADIUS_MAX: f32 = 128.;

#[derive(Clone)]
struct Stroke {
    tool: Tool,
    points: Vec<Point<Pixels>>, // Coordinates relative to the drawing area.
    color: u32,
    width: f32,
}

#[derive(Clone, Copy)]
enum Gesture {
    Editing {
        index: usize,
        handle: usize,
        opposite: Option<Point<Pixels>>,
        changed: bool,
    },
    Erasing {
        last: Point<Pixels>,
        changed: bool,
    },
}

struct DrawingApp {
    strokes: Vec<Stroke>,
    history: Vec<Vec<Stroke>>,
    gesture: Option<Gesture>,
    draft: Option<Stroke>,
    bezier_points: Vec<Point<Pixels>>,
    hover: Option<Point<Pixels>>,
    eraser_cursor: Option<Point<Pixels>>,
    eraser_radius: f32,
    focus_handle: Option<FocusHandle>,
    tool: Tool,
    color: u32,
    width: f32,
    canvas_bounds: Rc<Cell<Bounds<Pixels>>>,
}

impl DrawingApp {
    fn new() -> Self {
        Self {
            strokes: Vec::new(),
            history: Vec::new(),
            gesture: None,
            draft: None,
            bezier_points: Vec::new(),
            hover: None,
            eraser_cursor: None,
            eraser_radius: 16.,
            focus_handle: None,
            tool: Tool::Pencil,
            color: COLORS[0].1,
            width: WIDTHS[1],
            canvas_bounds: Rc::new(Cell::new(Bounds::default())),
        }
    }

    fn local_position(&self, position: Point<Pixels>) -> Point<Pixels> {
        let bounds = self.canvas_bounds.get();
        point(
            (position.x - bounds.origin.x).clamp(px(0.), bounds.size.width),
            (position.y - bounds.origin.y).clamp(px(0.), bounds.size.height),
        )
    }

    fn select_tool(&mut self, tool: Tool, cx: &mut Context<Self>) {
        self.tool = tool;
        self.gesture = None;
        self.draft = None;
        self.bezier_points.clear();
        self.hover = None;
        self.eraser_cursor = None;
        cx.notify();
    }

    fn change_eraser_radius(&mut self, key: &str) -> bool {
        let next = match key {
            "[" => self.eraser_radius - ERASER_RADIUS_STEP,
            "]" => self.eraser_radius + ERASER_RADIUS_STEP,
            _ => return false,
        };
        self.eraser_radius = next.clamp(ERASER_RADIUS_MIN, ERASER_RADIUS_MAX);
        true
    }

    fn key_down(&mut self, event: &KeyDownEvent, cx: &mut Context<Self>) {
        if self.tool == Tool::Eraser && self.change_eraser_radius(&event.keystroke.key) {
            cx.notify();
        }
    }

    fn add_bezier_point(&mut self, position: Point<Pixels>) {
        self.bezier_points.push(position);
        if self.bezier_points.len() == 4 {
            self.history.push(self.strokes.clone());
            self.strokes.push(Stroke {
                tool: Tool::Bezier,
                points: std::mem::take(&mut self.bezier_points),
                color: self.color,
                width: self.width,
            });
            self.hover = None;
        }
    }

    fn edit_handle(&mut self, position: Point<Pixels>) {
        let Some(Gesture::Editing {
            index,
            handle,
            opposite,
            changed,
        }) = self.gesture
        else {
            return;
        };
        let Some(stroke) = self.strokes.get(index) else {
            return;
        };
        let old_position = geometry::handles(stroke).get(handle).copied();
        if old_position == Some(position) && !changed {
            return;
        }
        if !changed {
            self.history.push(self.strokes.clone());
        }
        geometry::move_handle(&mut self.strokes[index], handle, position, opposite);
        self.gesture = Some(Gesture::Editing {
            index,
            handle,
            opposite,
            changed: true,
        });
    }

    fn erase_segment(&mut self, from: Point<Pixels>, to: Point<Pixels>) {
        let changed = matches!(self.gesture, Some(Gesture::Erasing { changed: true, .. }));
        if !self
            .strokes
            .iter()
            .any(|stroke| geometry::touched_by_eraser(stroke, from, to, self.eraser_radius))
        {
            return;
        }
        if !changed {
            self.history.push(self.strokes.clone());
        }
        self.strokes
            .retain(|stroke| !geometry::touched_by_eraser(stroke, from, to, self.eraser_radius));
        self.gesture = Some(Gesture::Erasing {
            last: to,
            changed: true,
        });
    }

    fn mouse_down(&mut self, event: &MouseDownEvent, cx: &mut Context<Self>) {
        let position = self.local_position(event.position);
        if self.tool == Tool::Eraser {
            self.eraser_cursor = Some(position);
            self.gesture = Some(Gesture::Erasing {
                last: position,
                changed: false,
            });
            self.erase_segment(position, position);
        } else if let Some((index, handle)) =
            self.strokes
                .iter()
                .enumerate()
                .rev()
                .find_map(|(index, stroke)| {
                    geometry::handle_at(stroke, position).map(|handle| (index, handle))
                })
        {
            let opposite = geometry::opposite_corner(&self.strokes[index], handle);
            self.gesture = Some(Gesture::Editing {
                index,
                handle,
                opposite,
                changed: false,
            });
            self.bezier_points.clear();
            self.hover = None;
        } else if self.tool == Tool::Bezier {
            self.add_bezier_point(position);
        } else {
            self.draft = Some(Stroke {
                tool: self.tool,
                points: vec![position],
                color: self.color,
                width: self.width,
            });
        }
        cx.notify();
    }

    fn mouse_move(&mut self, event: &MouseMoveEvent, cx: &mut Context<Self>) {
        let position = self.local_position(event.position);
        if self.tool == Tool::Eraser {
            self.eraser_cursor = Some(position);
        }
        match self.gesture {
            Some(Gesture::Editing { .. }) => self.edit_handle(position),
            Some(Gesture::Erasing { last, .. }) => {
                self.erase_segment(last, position);
                if let Some(Gesture::Erasing { last, .. }) = self.gesture.as_mut() {
                    *last = position;
                }
            }
            None => {
                if let Some(draft) = self.draft.as_mut() {
                    if draft.tool == Tool::Pencil {
                        if draft.points.last() != Some(&position) {
                            draft.points.push(position);
                        }
                    } else if draft.points.len() == 1 {
                        draft.points.push(position);
                    } else {
                        draft.points[1] = position;
                    }
                } else if !self.bezier_points.is_empty() {
                    self.hover = Some(position);
                }
            }
        }
        cx.notify();
    }

    fn mouse_up(&mut self, event: &MouseUpEvent, cx: &mut Context<Self>) {
        let position = self.local_position(event.position);
        match self.gesture.take() {
            Some(edit @ Gesture::Editing { .. }) => {
                self.gesture = Some(edit);
                self.edit_handle(position);
                self.gesture = None;
            }
            Some(Gesture::Erasing { last, changed }) => {
                self.gesture = Some(Gesture::Erasing { last, changed });
                self.erase_segment(last, position);
                self.gesture = None;
            }
            None => {
                if let Some(mut draft) = self.draft.take() {
                    if draft.tool == Tool::Pencil {
                        if draft.points.last() != Some(&position) {
                            draft.points.push(position);
                        }
                    } else if draft.points.len() == 1 {
                        draft.points.push(position);
                    } else {
                        draft.points[1] = position;
                    }
                    self.history.push(self.strokes.clone());
                    self.strokes.push(draft);
                }
            }
        }
        cx.notify();
    }

    fn undo(&mut self, cx: &mut Context<Self>) {
        if !self.bezier_points.is_empty() {
            self.bezier_points.clear();
            self.hover = None;
        } else if self.draft.is_some() {
            self.draft = None;
        } else if let Some(previous) = self.history.pop() {
            self.strokes = previous;
        }
        self.gesture = None;
        cx.notify();
    }
}

fn draw_stroke(stroke: &Stroke, origin: Point<Pixels>, window: &mut Window) {
    let at = |p: Point<Pixels>| point(origin.x + p.x, origin.y + p.y);
    let first = match stroke.points.first() {
        Some(point) => *point,
        None => return,
    };
    let mut path = PathBuilder::stroke(px(stroke.width));
    path.move_to(at(first));
    match stroke.tool {
        Tool::Pencil => {
            for &p in stroke.points.iter().skip(1) {
                path.line_to(at(p));
            }
        }
        Tool::Line => {
            if let Some(&end) = stroke.points.get(1) {
                path.line_to(at(end));
            }
        }
        Tool::Rectangle => {
            if let Some(&end) = stroke.points.get(1) {
                path.line_to(at(point(end.x, first.y)));
                path.line_to(at(end));
                path.line_to(at(point(first.x, end.y)));
                path.close();
            }
        }
        Tool::Ellipse => {
            if let Some(&end) = stroke.points.get(1) {
                let rx = (end.x - first.x).abs() * 0.5;
                let ry = (end.y - first.y).abs() * 0.5;
                if rx > px(0.) && ry > px(0.) {
                    let center = point((first.x + end.x) * 0.5, (first.y + end.y) * 0.5);
                    let right = at(point(center.x + rx, center.y));
                    let left = at(point(center.x - rx, center.y));
                    path = PathBuilder::stroke(px(stroke.width));
                    path.move_to(right);
                    path.arc_to(point(rx, ry), px(0.), false, false, left);
                    path.arc_to(point(rx, ry), px(0.), false, false, right);
                    path.close();
                }
            }
        }
        Tool::Eraser => return,
        Tool::Bezier => {
            if stroke.points.len() == 4 {
                path.cubic_bezier_to(
                    at(stroke.points[3]),
                    at(stroke.points[1]),
                    at(stroke.points[2]),
                );
            } else {
                for &p in stroke.points.iter().skip(1) {
                    path.line_to(at(p));
                }
            }
        }
    }
    if let Ok(path) = path.build() {
        window.paint_path(path, rgb(stroke.color));
    }
    // A single click should leave a visible mark, even without a mouse move.
    if stroke.tool == Tool::Pencil && stroke.points.len() == 1 {
        let radius = px(stroke.width / 2.);
        let center = at(first);
        let mut dot = PathBuilder::fill();
        let right = point(center.x + radius, center.y);
        let left = point(center.x - radius, center.y);
        dot.move_to(right);
        dot.arc_to(point(radius, radius), px(0.), false, false, left);
        dot.arc_to(point(radius, radius), px(0.), false, false, right);
        dot.close();
        if let Ok(path) = dot.build() {
            window.paint_path(path, rgb(stroke.color));
        }
    }
}

fn draw_handles(stroke: &Stroke, origin: Point<Pixels>, window: &mut Window) {
    if stroke.tool == Tool::Bezier && stroke.points.len() == 4 {
        for pair in [(0, 1), (2, 3)] {
            let mut guide = PathBuilder::stroke(px(1.));
            let a = stroke.points[pair.0];
            let b = stroke.points[pair.1];
            guide.move_to(point(origin.x + a.x, origin.y + a.y));
            guide.line_to(point(origin.x + b.x, origin.y + b.y));
            if let Ok(path) = guide.build() {
                window.paint_path(path, rgb(0x94a3b8));
            }
        }
    }
    for handle in geometry::handles(stroke) {
        let center = point(origin.x + handle.x, origin.y + handle.y);
        window.paint_quad(quad(
            Bounds {
                origin: point(center.x - px(5.), center.y - px(5.)),
                size: size(px(10.), px(10.)),
            },
            px(5.),
            rgb(0xffffff),
            px(1.),
            rgb(stroke.color),
            Default::default(),
        ));
    }
}

fn draw_eraser_cursor(
    position: Point<Pixels>,
    radius: f32,
    origin: Point<Pixels>,
    window: &mut Window,
) {
    let center = point(origin.x + position.x, origin.y + position.y);
    let radius = px(radius);
    let right = point(center.x + radius, center.y);
    let left = point(center.x - radius, center.y);
    let mut ring = PathBuilder::stroke(px(1.5));
    ring.move_to(right);
    ring.arc_to(point(radius, radius), px(0.), false, false, left);
    ring.arc_to(point(radius, radius), px(0.), false, false, right);
    ring.close();
    if let Ok(path) = ring.build() {
        window.paint_path(path, rgb(0x0f766e));
    }
}

fn tool_button(
    tool: Tool,
    selected: bool,
    cx: &mut Context<DrawingApp>,
) -> impl IntoElement + use<> {
    div()
        .id(tool.label())
        .w_full()
        .px_3()
        .py_2()
        .rounded_md()
        .cursor_pointer()
        .bg(rgb(if selected { 0x344564 } else { 0x1e293b }))
        .text_color(rgb(if selected { 0xffffff } else { 0xcbd5e1 }))
        .child(tool.label())
        .on_click(cx.listener(move |this, _, window, cx| {
            this.select_tool(tool, cx);
            if let Some(handle) = &this.focus_handle {
                window.focus(handle);
            }
        }))
}

impl Render for DrawingApp {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let strokes = self.strokes.clone();
        let draft = self.draft.clone();
        let bezier_points = self.bezier_points.clone();
        let hover = self.hover;
        let color = self.color;
        let width = self.width;
        let eraser_cursor = if self.tool == Tool::Eraser {
            self.eraser_cursor
        } else {
            None
        };
        let eraser_radius = self.eraser_radius;
        let bounds = self.canvas_bounds.clone();
        let hint = if self.tool == Tool::Eraser {
            format!(
                "Eraser: {} px radius  |  [ smaller  ] larger",
                eraser_radius as u32
            )
        } else if self.tool == Tool::Bezier {
            format!(
                "Bezier: click start, control 1, control 2, end ({}/4)",
                self.bezier_points.len()
            )
        } else {
            format!(
                "{}: drag to draw; drag a shape handle to edit",
                self.tool.label()
            )
        };

        div()
            .id("drawing-app")
            .when_some(self.focus_handle.clone(), |this, handle| {
                this.track_focus(&handle)
            })
            .on_key_down(cx.listener(|this, event, _, cx| this.key_down(event, cx)))
            .size_full()
            .flex()
            .flex_col()
            .bg(rgb(0x101827))
            .text_color(rgb(0xe2e8f0))
            .child(
                div()
                    .h(px(64.))
                    .px_6()
                    .flex()
                    .items_center()
                    .justify_between()
                    .border_b_1()
                    .border_color(rgb(0x334155))
                    .child(div().text_xl().child("Canvas"))
                    .child(div().text_sm().text_color(rgb(0x94a3b8)).child(hint)),
            )
            .child(
                div()
                    .flex()
                    .flex_1()
                    .min_h_0()
                    .child(
                        div()
                            .w(px(192.))
                            .p_4()
                            .flex()
                            .flex_col()
                            .gap_2()
                            .border_r_1()
                            .border_color(rgb(0x334155))
                            .child(div().text_sm().text_color(rgb(0x94a3b8)).child("TOOLS"))
                            .children(TOOLS.map(|tool| tool_button(tool, tool == self.tool, cx)))
                            .child(
                                div()
                                    .mt_4()
                                    .text_sm()
                                    .text_color(rgb(0x94a3b8))
                                    .child("COLOR"),
                            )
                            .child(div().flex().flex_wrap().gap_2().children(COLORS.map(
                                |(name, value)| {
                                    div()
                                        .id(name)
                                        .size(px(32.))
                                        .rounded_full()
                                        .bg(rgb(value))
                                        .cursor_pointer()
                                        .border_2()
                                        .border_color(rgb(if value == color {
                                            0xffffff
                                        } else {
                                            0x1e293b
                                        }))
                                        .on_click(cx.listener(move |this, _, _, cx| {
                                            this.color = value;
                                            cx.notify();
                                        }))
                                },
                            )))
                            .child(
                                div()
                                    .mt_4()
                                    .text_sm()
                                    .text_color(rgb(0x94a3b8))
                                    .child("THICKNESS"),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_wrap()
                                    .gap_2()
                                    .children(WIDTHS.map(|value| {
                                        div()
                                            .id(("width", value as u32))
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .cursor_pointer()
                                            .bg(rgb(if value == width {
                                                0x344564
                                            } else {
                                                0x1e293b
                                            }))
                                            .child(format!("{} px", value as u32))
                                            .on_click(cx.listener(move |this, _, _, cx| {
                                                this.width = value;
                                                cx.notify();
                                            }))
                                    })),
                            )
                            .child(
                                div()
                                    .mt_6()
                                    .flex()
                                    .gap_2()
                                    .child(
                                        div()
                                            .id("undo")
                                            .cursor_pointer()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(0x344564))
                                            .child("Undo")
                                            .on_click(cx.listener(|this, _, _, cx| this.undo(cx))),
                                    )
                                    .child(
                                        div()
                                            .id("clear")
                                            .cursor_pointer()
                                            .px_2()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(0x344564))
                                            .child("Clear")
                                            .on_click(cx.listener(|this, _, _, cx| {
                                                if !this.strokes.is_empty() {
                                                    this.history.push(this.strokes.clone());
                                                    this.strokes.clear();
                                                }
                                                this.gesture = None;
                                                this.draft = None;
                                                this.bezier_points.clear();
                                                this.hover = None;
                                                cx.notify();
                                            })),
                                    ),
                            ),
                    )
                    .child(
                        div().flex_1().min_w_0().p_5().child(
                            div()
                                .id("canvas-area")
                                .size_full()
                                .bg(rgb(0xffffff))
                                .overflow_hidden()
                                .cursor_crosshair()
                                .child(
                                    canvas(
                                        move |area, _, _| bounds.set(area),
                                        move |area, _, window, _| {
                                            for stroke in &strokes {
                                                draw_stroke(stroke, area.origin, window);
                                            }
                                            for stroke in &strokes {
                                                draw_handles(stroke, area.origin, window);
                                            }
                                            if let Some(stroke) = &draft {
                                                draw_stroke(stroke, area.origin, window);
                                            }
                                            if !bezier_points.is_empty() {
                                                let mut points = bezier_points;
                                                if let Some(hover) = hover {
                                                    points.push(hover);
                                                }
                                                draw_stroke(
                                                    &Stroke {
                                                        tool: Tool::Bezier,
                                                        points,
                                                        color,
                                                        width,
                                                    },
                                                    area.origin,
                                                    window,
                                                );
                                            }
                                            if let Some(cursor) = eraser_cursor {
                                                draw_eraser_cursor(
                                                    cursor,
                                                    eraser_radius,
                                                    area.origin,
                                                    window,
                                                );
                                            }
                                        },
                                    )
                                    .size_full(),
                                )
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, event, window, cx| {
                                        if let Some(handle) = &this.focus_handle {
                                            window.focus(handle);
                                        }
                                        this.mouse_down(event, cx);
                                    }),
                                )
                                .on_mouse_move(cx.listener(|this, event, _, cx| {
                                    this.mouse_move(event, cx);
                                }))
                                .on_hover(cx.listener(|this, hovered: &bool, _, cx| {
                                    if !hovered && this.eraser_cursor.take().is_some() {
                                        cx.notify();
                                    }
                                }))
                                .on_mouse_up(
                                    MouseButton::Left,
                                    cx.listener(|this, event, _, cx| {
                                        this.mouse_up(event, cx);
                                    }),
                                )
                                .on_mouse_up_out(
                                    MouseButton::Left,
                                    cx.listener(|this, event, _, cx| {
                                        this.mouse_up(event, cx);
                                    }),
                                ),
                        ),
                    ),
            )
    }
}

fn main() {
    Application::new().run(|cx| {
        let bounds = Bounds::centered(None, size(px(1100.), px(760.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                focus: true,
                ..Default::default()
            },
            |window, cx| {
                cx.new(|cx| {
                    let mut app = DrawingApp::new();
                    let handle = cx.focus_handle();
                    window.focus(&handle);
                    app.focus_handle = Some(handle);
                    app
                })
            },
        )
        .expect("could not open drawing window");
        cx.on_window_closed(|cx| cx.quit()).detach();
        cx.activate(true);
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn positions_are_local_and_clamped_to_canvas() {
        let app = DrawingApp::new();
        app.canvas_bounds.set(Bounds {
            origin: point(px(100.), px(50.)),
            size: size(px(200.), px(150.)),
        });
        assert_eq!(
            app.local_position(point(px(140.), px(75.))),
            point(px(40.), px(25.))
        );
        assert_eq!(
            app.local_position(point(px(350.), px(20.))),
            point(px(200.), px(0.))
        );
    }

    #[test]
    fn bracket_keys_adjust_eraser_radius_with_limits() {
        let mut app = DrawingApp::new();
        assert_eq!(app.eraser_radius, 16.);
        assert!(app.change_eraser_radius("]"));
        assert_eq!(app.eraser_radius, 20.);
        assert!(app.change_eraser_radius("["));
        assert_eq!(app.eraser_radius, 16.);
        for _ in 0..100 {
            app.change_eraser_radius("[");
        }
        assert_eq!(app.eraser_radius, ERASER_RADIUS_MIN);
        for _ in 0..100 {
            app.change_eraser_radius("]");
        }
        assert_eq!(app.eraser_radius, ERASER_RADIUS_MAX);
        assert!(!app.change_eraser_radius("x"));
        assert_eq!(app.eraser_radius, ERASER_RADIUS_MAX);
    }

    #[test]
    fn four_bezier_clicks_commit_one_curve_with_selected_style() {
        let mut app = DrawingApp::new();
        app.color = 0xe34b55;
        app.width = 12.;
        let points = [
            point(px(10.), px(20.)),
            point(px(30.), px(40.)),
            point(px(50.), px(60.)),
            point(px(70.), px(80.)),
        ];
        for p in points.iter().take(3) {
            app.add_bezier_point(*p);
        }
        assert!(app.strokes.is_empty());
        app.add_bezier_point(points[3]);
        assert!(app.bezier_points.is_empty());
        assert_eq!(app.strokes.len(), 1);
        assert_eq!(app.strokes[0].points, points);
        assert_eq!(app.strokes[0].color, 0xe34b55);
        assert_eq!(app.strokes[0].width, 12.);
    }

    #[test]
    fn edit_and_erase_each_record_one_undo_step() {
        let mut app = DrawingApp::new();
        app.strokes.push(Stroke {
            tool: Tool::Rectangle,
            points: vec![point(px(10.), px(10.)), point(px(100.), px(100.))],
            color: 0,
            width: 4.,
        });
        app.gesture = Some(Gesture::Editing {
            index: 0,
            handle: 0,
            opposite: geometry::opposite_corner(&app.strokes[0], 0),
            changed: false,
        });
        app.edit_handle(point(px(5.), px(5.)));
        app.edit_handle(point(px(0.), px(0.)));
        assert_eq!(app.history.len(), 1);
        assert_eq!(geometry::handles(&app.strokes[0])[0], point(px(0.), px(0.)));
        app.gesture = Some(Gesture::Erasing {
            last: point(px(50.), px(50.)),
            changed: false,
        });
        app.erase_segment(point(px(50.), px(50.)), point(px(110.), px(50.)));
        assert!(app.strokes.is_empty());
        assert_eq!(app.history.len(), 2);
        app.strokes = app.history.pop().unwrap();
        assert_eq!(geometry::handles(&app.strokes[0])[0], point(px(0.), px(0.)));
        app.strokes = app.history.pop().unwrap();
        assert_eq!(
            geometry::handles(&app.strokes[0])[0],
            point(px(10.), px(10.))
        );
    }
}
