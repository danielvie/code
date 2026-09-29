use crate::{
    gpu::{self, Event, Worker},
    scene::{self, SceneKind, Settings, Snapshot},
};
use gpui::{
    Bounds, Context, Div, IntoElement, MouseButton, MouseDownEvent, MouseMoveEvent, PathBuilder,
    Pixels, Point, Render, RenderImage, SharedString, Stateful, Task, Window, canvas, div, point,
    prelude::*, px, rgb,
};
use std::{
    cell::Cell,
    rc::Rc,
    sync::{Arc, mpsc::TryRecvError},
    time::{Duration, Instant},
};

const TEXT: u32 = 0xe6edf5;
const MUTED: u32 = 0x97a8bd;
const BORDER: u32 = 0x27364a;
const MINT: u32 = 0x52dcc2;
const BLUE: u32 = 0x8dafff;

#[derive(Clone, Copy)]
enum Action {
    Scene(SceneKind),
    Play,
    Reset,
    Projection,
    Wireframe,
    Checker,
    Depth,
    Resolution,
}

pub struct Playground {
    settings: Settings,
    playing: bool,
    dirty: bool,
    refresh_idle_metrics: bool,
    drag: Option<Point<Pixels>>,
    meshes: [Arc<scene::Mesh>; 3],
    shown: Snapshot,
    image: Option<Arc<RenderImage>>,
    worker: Worker,
    ready: bool,
    busy: bool,
    error: Option<String>,
    adapter: String,
    viewport: Rc<Cell<[f32; 2]>>,
    cpu_ms: Rc<Cell<f32>>,
    gpu_ms: f32,
    image_ms: f32,
    fps: f32,
    frames: u32,
    fps_start: Instant,
    last_tick: Instant,
    next_submit: Instant,
    _ticker: Task<()>,
}

impl Playground {
    pub fn new(cx: &mut Context<Self>) -> Self {
        let meshes = [
            scene::mesh(SceneKind::Cube),
            scene::mesh(SceneKind::Intersections),
            scene::mesh(SceneKind::Cubes),
        ];
        let settings = Settings::default();
        let shown = Snapshot {
            settings,
            mesh: meshes[0].clone(),
            width: 640,
            height: 480,
        };
        let ticker = cx.spawn(async move |this, cx| {
            loop {
                cx.background_executor()
                    .timer(Duration::from_millis(16))
                    .await;
                if this.update(cx, |this, cx| this.tick(cx)).is_err() {
                    break;
                }
            }
        });
        Self {
            settings,
            playing: true,
            dirty: true,
            refresh_idle_metrics: false,
            drag: None,
            meshes,
            shown,
            image: None,
            worker: Worker::start(),
            ready: false,
            busy: false,
            error: None,
            adapter: "Starting the offscreen renderer...".into(),
            viewport: Rc::new(Cell::new([640., 480.])),
            cpu_ms: Rc::new(Cell::new(0.)),
            gpu_ms: 0.,
            image_ms: 0.,
            fps: 0.,
            frames: 0,
            fps_start: Instant::now(),
            last_tick: Instant::now(),
            next_submit: Instant::now(),
            _ticker: ticker,
        }
    }

    fn snapshot(&self) -> Snapshot {
        let [width, height] = self.viewport.get();
        let width = width.max(1.);
        let height = height.max(1.);
        let scale = self.settings.resolution as f32 / width.max(height);
        let index = match self.settings.scene {
            SceneKind::Cube => 0,
            SceneKind::Intersections => 1,
            SceneKind::Cubes => 2,
        };
        Snapshot {
            settings: self.settings,
            mesh: self.meshes[index].clone(),
            width: (width * scale).round().max(1.) as u32,
            height: (height * scale).round().max(1.) as u32,
        }
    }

    fn tick(&mut self, cx: &mut Context<Self>) {
        // Paint records the CPU timing after labels have rendered. Refresh once
        // for a paused snapshot so its label does not keep the previous scene's cost.
        if std::mem::take(&mut self.refresh_idle_metrics) && !self.playing {
            cx.notify();
        }
        let now = Instant::now();
        let dt = now.duration_since(self.last_tick).as_secs_f32().min(0.1);
        self.last_tick = now;
        if self.playing && self.drag.is_none() {
            self.settings.angle = (self.settings.angle + dt * 0.38) % std::f32::consts::TAU;
            self.dirty = true;
        }
        loop {
            match self.worker.events.try_recv() {
                Ok(Event::Ready(adapter)) => {
                    self.ready = true;
                    self.adapter = adapter;
                    cx.notify();
                }
                Ok(Event::Frame(frame)) => {
                    self.busy = false;
                    self.accept_frame(frame, cx);
                }
                Ok(Event::Error(error)) => {
                    self.fail(error, cx);
                    break;
                }
                Err(TryRecvError::Empty) => break,
                Err(TryRecvError::Disconnected) => {
                    if self.error.is_none() {
                        self.fail("The wgpu worker stopped. Check the terminal for driver or validation errors.".into(), cx);
                    }
                    break;
                }
            }
        }
        let snapshot = self.snapshot();
        if snapshot.width != self.shown.width || snapshot.height != self.shown.height {
            self.dirty = true;
        }
        if self.error.is_some() {
            if self.dirty {
                self.shown = snapshot;
                self.dirty = false;
                cx.notify();
            }
        } else if self.ready && !self.busy && self.dirty && now >= self.next_submit {
            if self.worker.requests.try_send(snapshot).is_ok() {
                self.busy = true;
                self.dirty = false;
                let interval = Duration::from_secs_f32(1. / 30.);
                self.next_submit += interval;
                if self.next_submit <= now {
                    self.next_submit = now + interval;
                }
            } else {
                self.fail("Cannot send a frame to the wgpu worker.".into(), cx);
            }
        }
    }

    fn fail(&mut self, error: String, cx: &mut Context<Self>) {
        eprintln!("wgpu: {error}");
        self.error = Some(error);
        self.ready = false;
        self.busy = false;
        self.dirty = true;
        cx.notify();
    }

    fn accept_frame(&mut self, frame: gpu::Frame, cx: &mut Context<Self>) {
        let started = Instant::now();
        self.shown = frame.snapshot;
        self.refresh_idle_metrics = true;
        self.gpu_ms = frame.render_ms;
        let buffer = image::RgbaImage::from_raw(self.shown.width, self.shown.height, frame.bgra)
            .expect("worker returned a malformed image");
        let next = Arc::new(RenderImage::new(vec![image::Frame::new(buffer)]));
        if let Some(previous) = self.image.replace(next) {
            // Every image gets a new ID. Explicit eviction prevents unbounded atlas growth.
            cx.drop_image(previous, None);
        }
        self.image_ms = started.elapsed().as_secs_f32() * 1000.;
        self.frames += 1;
        let elapsed = self.fps_start.elapsed().as_secs_f32();
        if elapsed >= 0.5 {
            self.fps = self.frames as f32 / elapsed;
            self.frames = 0;
            self.fps_start = Instant::now();
        }
        // Both panes switch to exactly the snapshot that produced this image.
        cx.notify();
    }

    fn act(&mut self, action: Action, cx: &mut Context<Self>) {
        match action {
            Action::Scene(kind) => {
                self.settings.scene = kind;
                self.settings.angle = 0.;
                self.settings.distance = if kind == SceneKind::Cubes { 12. } else { 8.5 };
                if kind == SceneKind::Intersections {
                    self.settings.yaw = 0.2;
                    self.settings.pitch = 0.15;
                    self.playing = false;
                }
            }
            Action::Play => {
                self.playing = !self.playing;
                self.frames = 0;
                self.fps = 0.;
                self.fps_start = Instant::now();
            }
            Action::Reset => {
                let scene = self.settings.scene;
                self.settings = Settings {
                    scene,
                    ..Default::default()
                };
                if scene == SceneKind::Cubes {
                    self.settings.distance = 12.;
                }
                if scene == SceneKind::Intersections {
                    self.settings.yaw = 0.2;
                    self.settings.pitch = 0.15;
                }
            }
            Action::Projection => self.settings.orthographic = !self.settings.orthographic,
            Action::Wireframe => self.settings.wireframe = !self.settings.wireframe,
            Action::Checker => self.settings.checker = !self.settings.checker,
            Action::Depth => self.settings.depth = !self.settings.depth,
            Action::Resolution => {
                self.settings.resolution = match self.settings.resolution {
                    320 => 640,
                    640 => 960,
                    _ => 320,
                }
            }
        }
        self.dirty = true;
        cx.notify();
    }

    fn control(
        &self,
        id: &'static str,
        label: impl Into<SharedString>,
        active: bool,
        action: Action,
        cx: &Context<Self>,
    ) -> Stateful<Div> {
        div()
            .id(id)
            .px_3()
            .py_2()
            .rounded_md()
            .border_1()
            .border_color(rgb(if active { 0x42665f } else { BORDER }))
            .bg(rgb(if active { 0x193b36 } else { 0x162131 }))
            .text_color(rgb(if active { MINT } else { TEXT }))
            .text_size(px(12.))
            .cursor_pointer()
            .hover(|s| s.bg(rgb(0x26374a)))
            .child(label.into())
            .on_click(cx.listener(move |this, _, _, cx| this.act(action, cx)))
    }

    fn viewport(&self, native: bool, cx: &Context<Self>) -> Stateful<Div> {
        let snapshot = self.shown.clone();
        let viewport = self.viewport.clone();
        let cpu_ms = self.cpu_ms.clone();
        let image = self.image.clone();
        let drawing = canvas(
            move |bounds, _, _| {
                if native {
                    viewport.set([f32::from(bounds.size.width), f32::from(bounds.size.height)]);
                    let started = Instant::now();
                    let drawing = scene::project(&snapshot);
                    (Some(drawing), started.elapsed().as_secs_f32() * 1000.)
                } else {
                    (None, 0.)
                }
            },
            move |bounds, (drawing, projection_ms), window, _| {
                if let Some(drawing) = drawing {
                    let started = Instant::now();
                    for segment in drawing.segments {
                        paint_path(&segment.points, segment.color, false, bounds, window);
                    }
                    for polygon in drawing.polygons {
                        paint_path(&polygon.points, polygon.color, true, bounds, window);
                    }
                    cpu_ms.set(projection_ms + started.elapsed().as_secs_f32() * 1000.);
                } else if let Some(image) = image
                    && let Err(error) =
                        window.paint_image(bounds, Default::default(), image, 0, false)
                {
                    eprintln!("Cannot paint wgpu image: {error}");
                }
            },
        )
        .absolute()
        .size_full();
        let mut viewport = div()
            .id(if native {
                "native-viewport"
            } else {
                "wgpu-viewport"
            })
            .relative()
            .flex_1()
            .min_h_0()
            .w_full()
            .overflow_hidden()
            .bg(rgb(scene::BACKGROUND))
            .cursor_crosshair()
            .child(drawing)
            .on_mouse_down(
                MouseButton::Left,
                cx.listener(|this, event: &MouseDownEvent, _, cx| {
                    this.drag = Some(event.position);
                    cx.notify();
                }),
            )
            .on_scroll_wheel(cx.listener(|this, event: &gpui::ScrollWheelEvent, _, cx| {
                let delta = f32::from(event.delta.pixel_delta(px(20.)).y);
                this.settings.distance =
                    (this.settings.distance * (-delta * 0.002).exp()).clamp(2.5, 25.);
                this.dirty = true;
                cx.notify();
            }));
        if !native {
            if let Some(error) = &self.error {
                viewport = viewport.child(div().absolute().inset_0().p_6().flex().flex_col().justify_center().gap_3()
                    .bg(rgb(0x17202e)).text_color(rgb(0xf4b7a3))
                    .child("wgpu unavailable")
                    .child(div().text_sm().child(error.clone()))
                    .child(div().text_sm().text_color(rgb(MUTED)).child("The GPUI canvas remains interactive. Restart after checking the driver or terminal error.")));
            } else if self.image.is_none() {
                viewport = viewport.child(
                    div()
                        .absolute()
                        .inset_0()
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(MUTED))
                        .child("Creating device and compiling shaders..."),
                );
            }
        }
        viewport
    }

    fn pane(&self, native: bool, cx: &Context<Self>) -> Div {
        let color = if native { MINT } else { BLUE };
        let title = if native {
            "GPUI canvas"
        } else {
            "wgpu + GPUI image"
        };
        let subtitle = if native {
            "3D math in Rust. 2D paths in GPUI."
        } else {
            "A separate 3D renderer, embedded as pixels."
        };
        let metric = if native {
            format!("{:.2} ms", self.cpu_ms.get())
        } else {
            format!("{:.2} ms", self.gpu_ms)
        };
        let metric_label = if native {
            "CPU projection + path recording"
        } else {
            "Offscreen render + CPU readback"
        };
        let details = if native {
            "Flat lighting / triangle sorting / no depth buffer"
        } else {
            "WGSL shader / depth buffer / CPU image transfer"
        };
        let use_case = if native {
            "Useful for gizmos, wireframes, simple diagrams and lightweight spatial views."
        } else {
            "Useful for mesh viewers and richer scenes. Image transfer is the integration cost to watch."
        };
        div()
            .flex()
            .flex_col()
            .flex_1()
            .min_w_0()
            .h_full()
            .overflow_hidden()
            .rounded_lg()
            .border_1()
            .border_color(rgb(BORDER))
            .bg(rgb(0x111d2b))
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .gap_3()
                    .px_4()
                    .py_4()
                    .border_b_1()
                    .border_color(rgb(BORDER))
                    .child(
                        div()
                            .flex()
                            .gap_3()
                            .items_center()
                            .child(
                                div()
                                    .text_size(px(24.))
                                    .text_color(rgb(color))
                                    .child(if native { "01" } else { "02" }),
                            )
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .text_size(px(18.))
                                            .font_weight(gpui::FontWeight::SEMIBOLD)
                                            .child(title),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.))
                                            .text_color(rgb(MUTED))
                                            .child(subtitle),
                                    ),
                            ),
                    )
                    .child(
                        div()
                            .text_size(px(10.))
                            .text_color(rgb(color))
                            .child(if native {
                                "PROJECTED"
                            } else if self.shown.settings.depth {
                                "DEPTH-TESTED"
                            } else {
                                "DEPTH OFF"
                            }),
                    ),
            )
            .child(self.viewport(native, cx))
            .child(
                div()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(rgb(BORDER))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .items_baseline()
                            .gap_2()
                            .child(
                                div()
                                    .text_size(px(21.))
                                    .text_color(rgb(color))
                                    .child(metric),
                            )
                            .child(
                                div()
                                    .text_size(px(11.))
                                    .text_color(rgb(MUTED))
                                    .child(metric_label),
                            ),
                    )
                    .child(div().text_size(px(12.)).child(details))
                    .child(
                        div()
                            .text_size(px(11.))
                            .text_color(rgb(MUTED))
                            .child(use_case),
                    ),
            )
    }
}

impl Render for Playground {
    fn render(&mut self, _: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let s = self.settings;
        let transferred = (self.shown.width * self.shown.height * 4) as f32 / (1024. * 1024.);
        let activity = if !self.playing && self.drag.is_none() {
            "On demand".into()
        } else {
            format!("{:.0} paired updates/s", self.fps)
        };
        div().size_full().flex().flex_col().bg(rgb(0x0c1420)).text_color(rgb(TEXT))
            .font_family("Segoe UI").text_size(px(14.))
            .child(div().px_6().pt_5().pb_4().flex().flex_col().gap_2()
                .child(div().text_size(px(10.)).text_color(rgb(MINT)).child("CAPABILITY LAB  /  RUST + GPUI  /  EXPERIMENT 01"))
                .child(div().flex().items_center().justify_between()
                    .child(div().text_size(px(28.)).font_weight(gpui::FontWeight::SEMIBOLD).child("Two ways to draw in 3D"))
                    .child(div().text_size(px(12.)).text_color(rgb(MUTED)).child(format!("{activity}  /  30 Hz target"))))
                .child(div().text_size(px(12.)).text_color(rgb(MUTED)).child("Same scene. Same camera. Different rendering responsibilities. Drag either pane to orbit; scroll to zoom.")))
            .child(div().px_6().pb_3().flex().items_center().gap_2()
                .child(self.control("cube", "Cube", s.scene == SceneKind::Cube, Action::Scene(SceneKind::Cube), cx))
                .child(self.control("intersections", "Intersections", s.scene == SceneKind::Intersections, Action::Scene(SceneKind::Intersections), cx))
                .child(self.control("cubes", "125 cubes", s.scene == SceneKind::Cubes, Action::Scene(SceneKind::Cubes), cx))
                .child(div().flex_1())
                .child(self.control("play", if self.playing { "Pause rotation" } else { "Play rotation" }, self.playing, Action::Play, cx))
                .child(self.control("reset", "Reset view", false, Action::Reset, cx)))
            .child(div().px_6().pb_3().flex().items_center().gap_2()
                .child(self.control("projection", if s.orthographic { "Orthographic" } else { "Perspective" }, s.orthographic, Action::Projection, cx))
                .child(self.control("wireframe", if s.wireframe { "Wireframe" } else { "Solid faces" }, s.wireframe, Action::Wireframe, cx))
                .child(div().px_2().text_size(px(11.)).text_color(rgb(MUTED)).child("RIGHT ONLY"))
                .child(self.control("checker", if s.checker { "Checker on" } else { "Checker off" }, s.checker, Action::Checker, cx))
                .child(self.control("depth", if s.depth { "Depth on" } else { "Depth off" }, s.depth, Action::Depth, cx))
                .child(self.control("resolution", format!("{} px", s.resolution), false, Action::Resolution, cx)))
            .child(div().px_6().pb_3().text_size(px(12.)).text_color(rgb(0xb9c9dd)).child(s.scene.hint()))
            .child(div().px_6().flex().gap_3().flex_1().min_h_0()
                .child(self.pane(true, cx)).child(self.pane(false, cx)))
            .child(div().px_6().py_4().flex().flex_col().gap_2()
                .child(div().flex().justify_between().text_size(px(11.)).text_color(rgb(MUTED))
                    .child(format!("{} / {} triangles / {} x {} px / {:.2} MiB read back per frame, then uploaded again", self.shown.settings.scene.title(), self.shown.mesh.triangles.len() / 3, self.shown.width, self.shown.height, transferred))
                    .child(format!("Image wrap + eviction: {:.2} ms", self.image_ms)))
                .child(div().text_size(px(11.)).text_color(rgb(MUTED)).child(format!("Adapter: {}", self.adapter)))
                .child(div().text_size(px(11.)).text_color(rgb(0xb9c9dd)).child("Timings cover different work, not a GPU benchmark. GPUI's final rendering / image upload is not timed. Both panes wait for the same completed snapshot.")))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if let Some(previous) = this.drag {
                    if event.pressed_button != Some(MouseButton::Left) { this.drag = None; return; }
                    this.settings.yaw -= f32::from(event.position.x - previous.x) * 0.008;
                    this.settings.pitch = (this.settings.pitch + f32::from(event.position.y - previous.y) * 0.008).clamp(-1.35, 1.35);
                    this.drag = Some(event.position);
                    this.dirty = true;
                    cx.notify();
                }
            }))
            .on_mouse_up(MouseButton::Left, cx.listener(|this, _, _, cx| { this.drag = None; cx.notify(); }))
            .on_mouse_up_out(MouseButton::Left, cx.listener(|this, _, _, cx| { this.drag = None; cx.notify(); }))
    }
}

fn paint_path(
    points: &[[f32; 2]],
    color: [f32; 3],
    filled: bool,
    bounds: Bounds<Pixels>,
    window: &mut Window,
) {
    let mut path = if filled {
        PathBuilder::fill()
    } else {
        PathBuilder::stroke(px(1.))
    };
    for (i, p) in points.iter().enumerate() {
        let position = point(
            bounds.left() + bounds.size.width * p[0],
            bounds.top() + bounds.size.height * p[1],
        );
        if i == 0 {
            path.move_to(position);
        } else {
            path.line_to(position);
        }
    }
    if filled {
        path.close();
    }
    if let Ok(path) = path.build() {
        window.paint_path(
            path,
            gpui::Rgba {
                r: color[0],
                g: color[1],
                b: color[2],
                a: 1.,
            },
        );
    }
}
