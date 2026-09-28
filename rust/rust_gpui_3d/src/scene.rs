//! Shared geometry and camera. Neither renderer owns the scene.
use glam::{Mat4, Vec3, Vec4};
use std::sync::Arc;

pub const BACKGROUND: u32 = 0x0b121b;
pub const FOV: f32 = 45.0_f32.to_radians();

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SceneKind {
    Cube,
    Intersections,
    Cubes,
}

impl SceneKind {
    pub fn title(self) -> &'static str {
        match self {
            Self::Cube => "Cube",
            Self::Intersections => "Intersections",
            Self::Cubes => "125 cubes",
        }
    }

    pub fn hint(self) -> &'static str {
        match self {
            Self::Cube => "Start here. Both panes use the same geometry, camera and flat lighting.",
            Self::Intersections => {
                "Pause and orbit. Sorting whole triangles cannot resolve intersecting faces; a depth buffer can."
            }
            Self::Cubes => {
                "Orbit through 1,500 triangles. Compare interaction and workload, not just the timing numbers."
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub color: [f32; 3],
}

#[derive(Clone)]
pub struct Mesh {
    pub triangles: Vec<Vertex>,
    pub edges: Vec<Vertex>,
    pub grid: Vec<Vertex>,
}

#[derive(Clone, Copy, Debug)]
pub struct Settings {
    pub scene: SceneKind,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    pub angle: f32,
    pub orthographic: bool,
    pub wireframe: bool,
    pub checker: bool,
    pub depth: bool,
    pub resolution: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            scene: SceneKind::Cube,
            yaw: 0.65,
            pitch: 0.35,
            distance: 8.5,
            angle: 0.0,
            orthographic: false,
            wireframe: false,
            checker: false,
            depth: true,
            resolution: 640,
        }
    }
}

impl Settings {
    pub fn eye(self) -> Vec3 {
        self.distance
            * Vec3::new(
                self.yaw.sin() * self.pitch.cos(),
                self.pitch.sin(),
                self.yaw.cos() * self.pitch.cos(),
            )
    }

    pub fn matrices(self, aspect: f32) -> (Mat4, Mat4) {
        let view = Mat4::look_at_rh(self.eye(), Vec3::ZERO, Vec3::Y);
        let projection = if self.orthographic {
            let half = self.distance * (FOV / 2.0).tan();
            Mat4::orthographic_rh(-half * aspect, half * aspect, -half, half, 0.1, 100.0)
        } else {
            Mat4::perspective_rh(FOV, aspect, 0.1, 100.0)
        };
        (projection * view, Mat4::from_rotation_y(self.angle))
    }
}

#[derive(Clone)]
pub struct Snapshot {
    pub settings: Settings,
    pub mesh: Arc<Mesh>,
    pub width: u32,
    pub height: u32,
}

fn rgb(color: u32) -> [f32; 3] {
    [
        ((color >> 16) & 255) as f32 / 255.0,
        ((color >> 8) & 255) as f32 / 255.0,
        (color & 255) as f32 / 255.0,
    ]
}

fn quad(out: &mut Vec<Vertex>, positions: [Vec3; 4], color: u32) {
    let normal = (positions[1] - positions[0])
        .cross(positions[2] - positions[0])
        .normalize();
    for index in [0, 1, 2, 0, 2, 3] {
        out.push(Vertex {
            position: positions[index].to_array(),
            normal: normal.to_array(),
            color: rgb(color),
        });
    }
}

fn cube(out: &mut Vec<Vertex>, center: Vec3, half: f32) {
    let p = |x, y, z| center + Vec3::new(x, y, z) * half;
    for (positions, color) in [
        (
            [
                p(-1., -1., 1.),
                p(1., -1., 1.),
                p(1., 1., 1.),
                p(-1., 1., 1.),
            ],
            0x52dcc2,
        ),
        (
            [
                p(1., -1., -1.),
                p(-1., -1., -1.),
                p(-1., 1., -1.),
                p(1., 1., -1.),
            ],
            0x729cff,
        ),
        (
            [
                p(1., -1., 1.),
                p(1., -1., -1.),
                p(1., 1., -1.),
                p(1., 1., 1.),
            ],
            0xefa55d,
        ),
        (
            [
                p(-1., -1., -1.),
                p(-1., -1., 1.),
                p(-1., 1., 1.),
                p(-1., 1., -1.),
            ],
            0xa69aef,
        ),
        (
            [
                p(-1., 1., 1.),
                p(1., 1., 1.),
                p(1., 1., -1.),
                p(-1., 1., -1.),
            ],
            0x91cef2,
        ),
        (
            [
                p(-1., -1., -1.),
                p(1., -1., -1.),
                p(1., -1., 1.),
                p(-1., -1., 1.),
            ],
            0xee7f89,
        ),
    ] {
        quad(out, positions, color);
    }
}

fn line(out: &mut Vec<Vertex>, a: Vec3, b: Vec3, color: u32) {
    for position in [a, b] {
        out.push(Vertex {
            position: position.to_array(),
            normal: [0.; 3],
            color: rgb(color),
        });
    }
}

pub fn mesh(kind: SceneKind) -> Arc<Mesh> {
    let mut triangles = Vec::new();
    match kind {
        SceneKind::Cube => cube(&mut triangles, Vec3::ZERO, 1.0),
        SceneKind::Intersections => {
            for (sign, color) in [(1.0, 0x52dcc2), (-1.0, 0xefa55d)] {
                quad(
                    &mut triangles,
                    [
                        Vec3::new(-2., -1.2, -1.4 * sign),
                        Vec3::new(2., -1.2, 1.4 * sign),
                        Vec3::new(2., 1.2, 1.4 * sign),
                        Vec3::new(-2., 1.2, -1.4 * sign),
                    ],
                    color,
                );
            }
        }
        SceneKind::Cubes => {
            for x in -2..=2 {
                for y in -2..=2 {
                    for z in -2..=2 {
                        cube(
                            &mut triangles,
                            Vec3::new(x as f32, y as f32, z as f32),
                            0.32,
                        );
                    }
                }
            }
        }
    }
    let mut edges = Vec::new();
    for triangle in triangles.chunks_exact(3) {
        for (a, b) in [(0, 1), (1, 2), (2, 0)] {
            line(
                &mut edges,
                triangle[a].position.into(),
                triangle[b].position.into(),
                0xb1d8e5,
            );
        }
    }
    let mut grid = Vec::new();
    let floor = if kind == SceneKind::Cubes { -2.5 } else { -1.5 };
    for i in -5..=5 {
        let v = i as f32;
        line(
            &mut grid,
            Vec3::new(v, floor, -5.),
            Vec3::new(v, floor, 5.),
            0x233347,
        );
        line(
            &mut grid,
            Vec3::new(-5., floor, v),
            Vec3::new(5., floor, v),
            0x233347,
        );
    }
    let origin = Vec3::new(-3., floor + 0.02, 2.5);
    line(&mut grid, origin, origin + Vec3::X * 1.2, 0xef7986);
    line(&mut grid, origin, origin + Vec3::Y * 1.2, 0x52dcc2);
    line(&mut grid, origin, origin + Vec3::Z * 1.2, 0x729cff);
    Arc::new(Mesh {
        triangles,
        edges,
        grid,
    })
}

pub fn lit_color(vertex: &Vertex, model: Mat4) -> [f32; 3] {
    let normal = model
        .transform_vector3(vertex.normal.into())
        .normalize_or_zero();
    let light = Vec3::new(0.4, 0.8, 0.6).normalize();
    let intensity = 0.28 + 0.72 * normal.dot(light).max(0.0);
    vertex.color.map(|channel| channel * intensity)
}

pub struct Polygon {
    pub points: Vec<[f32; 2]>,
    pub color: [f32; 3],
    pub depth: f32,
}

pub struct Segment {
    pub points: [[f32; 2]; 2],
    pub color: [f32; 3],
}

pub struct Drawing {
    pub polygons: Vec<Polygon>,
    pub segments: Vec<Segment>,
}

// WebGPU's homogeneous clip volume: -w <= x,y <= w and 0 <= z <= w.
fn plane(p: Vec4, index: usize) -> f32 {
    match index {
        0 => p.x + p.w,
        1 => p.w - p.x,
        2 => p.y + p.w,
        3 => p.w - p.y,
        4 => p.z,
        _ => p.w - p.z,
    }
}

fn clip_polygon(mut points: Vec<Vec4>) -> Vec<Vec4> {
    for index in 0..6 {
        let input = std::mem::take(&mut points);
        let Some(&mut_previous) = input.last() else {
            break;
        };
        let mut previous = mut_previous;
        let mut previous_d = plane(previous, index);
        for current in input {
            let d = plane(current, index);
            if (d >= 0.) != (previous_d >= 0.) {
                points.push(previous.lerp(current, previous_d / (previous_d - d)));
            }
            if d >= 0. {
                points.push(current);
            }
            previous = current;
            previous_d = d;
        }
    }
    points
}

fn screen(p: Vec4) -> [f32; 2] {
    [p.x / p.w * 0.5 + 0.5, 0.5 - p.y / p.w * 0.5]
}

fn project_lines(vertices: &[Vertex], transform: Mat4, out: &mut Vec<Segment>) {
    for pair in vertices.chunks_exact(2) {
        let mut a = transform * Vec3::from_array(pair[0].position).extend(1.0);
        let mut b = transform * Vec3::from_array(pair[1].position).extend(1.0);
        let mut visible = true;
        for i in 0..6 {
            let da = plane(a, i);
            let db = plane(b, i);
            if da < 0. && db < 0. {
                visible = false;
                break;
            }
            if (da >= 0.) != (db >= 0.) {
                let intersection = a.lerp(b, da / (da - db));
                if da < 0. {
                    a = intersection;
                } else {
                    b = intersection;
                }
            }
        }
        if visible {
            out.push(Segment {
                points: [screen(a), screen(b)],
                color: pair[0].color,
            });
        }
    }
}

pub fn project(snapshot: &Snapshot) -> Drawing {
    let (vp, model) = snapshot
        .settings
        .matrices(snapshot.width as f32 / snapshot.height as f32);
    let mut drawing = Drawing {
        polygons: Vec::new(),
        segments: Vec::new(),
    };
    project_lines(&snapshot.mesh.grid, vp, &mut drawing.segments);
    if snapshot.settings.wireframe {
        project_lines(&snapshot.mesh.edges, vp * model, &mut drawing.segments);
    } else {
        for triangle in snapshot.mesh.triangles.chunks_exact(3) {
            // Closed cubes do not need their back faces painted. Keep the intersecting
            // sheets double-sided so that the painter's algorithm limitation stays visible.
            if snapshot.settings.scene != SceneKind::Intersections {
                let normal = model.transform_vector3(triangle[0].normal.into());
                let toward_eye = if snapshot.settings.orthographic {
                    snapshot.settings.eye()
                } else {
                    snapshot.settings.eye() - model.transform_point3(triangle[0].position.into())
                };
                if normal.dot(toward_eye) <= 0. {
                    continue;
                }
            }
            let clipped = clip_polygon(
                triangle
                    .iter()
                    .map(|v| vp * model * Vec3::from_array(v.position).extend(1.))
                    .collect(),
            );
            if clipped.len() >= 3 {
                drawing.polygons.push(Polygon {
                    depth: clipped.iter().map(|v| v.z / v.w).sum::<f32>() / clipped.len() as f32,
                    points: clipped.into_iter().map(screen).collect(),
                    color: lit_color(&triangle[0], model),
                });
            }
        }
        drawing.polygons.sort_by(|a, b| b.depth.total_cmp(&a.depth));
    }
    drawing
}
