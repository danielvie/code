//! Offscreen wgpu renderer. Readback runs on its own thread, never on GPUI's UI thread.
use crate::scene::{self, SceneKind, Settings, Snapshot, Vertex};
use anyhow::{Context, Result};
use glam::Mat4;
use std::{sync::mpsc, time::Instant};
use wgpu::util::DeviceExt;

// Match GPUI's byte order directly; avoid a per-pixel CPU channel conversion.
const FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8Unorm;

#[repr(C)]
#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
struct Uniforms {
    view_projection: [[f32; 4]; 4],
    model: [[f32; 4]; 4],
    options: [f32; 4],
}

pub struct Frame {
    pub snapshot: Snapshot,
    pub bgra: Vec<u8>,
    pub render_ms: f32,
}

pub enum Event {
    Ready(String),
    Frame(Frame),
    Error(String),
}

pub struct Worker {
    pub requests: mpsc::SyncSender<Snapshot>,
    pub events: mpsc::Receiver<Event>,
}

impl Worker {
    pub fn start() -> Self {
        // The UI sends only when the previous result has arrived. No growing frame queue.
        let (requests, receiver) = mpsc::sync_channel::<Snapshot>(1);
        let (sender, events) = mpsc::sync_channel(1);
        std::thread::Builder::new()
            .name("wgpu-offscreen".into())
            .spawn(move || {
                let result = (|| -> Result<()> {
                    let mut renderer = pollster::block_on(Renderer::new())?;
                    if sender.send(Event::Ready(renderer.adapter.clone())).is_err() {
                        return Ok(());
                    }
                    while let Ok(snapshot) = receiver.recv() {
                        let frame = renderer.render(snapshot)?;
                        if sender.send(Event::Frame(frame)).is_err() {
                            break;
                        }
                    }
                    Ok(())
                })();
                if let Err(error) = result {
                    let _ = sender.send(Event::Error(format!("{error:#}")));
                }
            })
            .expect("could not start wgpu worker");
        Self { requests, events }
    }
}

struct Targets {
    width: u32,
    height: u32,
    color: wgpu::Texture,
    color_view: wgpu::TextureView,
    _depth: wgpu::Texture,
    depth_view: wgpu::TextureView,
    staging: wgpu::Buffer,
    row_bytes: u32,
}

struct Geometry {
    scene: SceneKind,
    triangles: wgpu::Buffer,
    edges: wgpu::Buffer,
    grid: wgpu::Buffer,
}

pub struct Renderer {
    device: wgpu::Device,
    queue: wgpu::Queue,
    pub adapter: String,
    solid: wgpu::RenderPipeline,
    solid_no_depth: wgpu::RenderPipeline,
    lines: wgpu::RenderPipeline,
    lines_no_depth: wgpu::RenderPipeline,
    object_uniform: wgpu::Buffer,
    grid_uniform: wgpu::Buffer,
    object_bind: wgpu::BindGroup,
    grid_bind: wgpu::BindGroup,
    targets: Option<Targets>,
    geometry: Option<Geometry>,
}

impl Renderer {
    pub async fn new() -> Result<Self> {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: None,
                force_fallback_adapter: false,
            })
            .await
            .context("No wgpu adapter available. The GPUI-only pane still works.")?;
        let info = adapter.get_info();
        let adapter_name = format!(
            "{} / {:?} / {:?}",
            info.name, info.backend, info.device_type
        );
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: Some("3D lab device"),
                    ..Default::default()
                },
                None,
            )
            .await
            .context("Cannot create wgpu device")?;
        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("scene uniforms"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let uniform = || {
            device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("camera + model"),
                size: std::mem::size_of::<Uniforms>() as u64,
                usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            })
        };
        let object_uniform = uniform();
        let grid_uniform = uniform();
        let bind = |buffer: &wgpu::Buffer| {
            device.create_bind_group(&wgpu::BindGroupDescriptor {
                label: Some("scene bind group"),
                layout: &layout,
                entries: &[wgpu::BindGroupEntry {
                    binding: 0,
                    resource: buffer.as_entire_binding(),
                }],
            })
        };
        let object_bind = bind(&object_uniform);
        let grid_bind = bind(&grid_uniform);
        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("3D lab WGSL"),
            source: wgpu::ShaderSource::Wgsl(include_str!("scene.wgsl").into()),
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("3D lab pipeline layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });
        let pipeline = |topology, depth| {
            device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
                label: Some("3D lab pipeline"),
                layout: Some(&pipeline_layout),
                vertex: wgpu::VertexState {
                    module: &shader,
                    entry_point: Some("vs_main"),
                    compilation_options: Default::default(),
                    buffers: &[wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as u64,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![0 => Float32x3, 1 => Float32x3, 2 => Float32x3],
                    }],
                },
                fragment: Some(wgpu::FragmentState {
                    module: &shader,
                    entry_point: Some("fs_main"),
                    compilation_options: Default::default(),
                    targets: &[Some(wgpu::ColorTargetState {
                        format: FORMAT,
                        blend: None,
                        write_mask: wgpu::ColorWrites::ALL,
                    })],
                }),
                primitive: wgpu::PrimitiveState { topology, cull_mode: None, ..Default::default() },
                depth_stencil: Some(wgpu::DepthStencilState {
                    format: wgpu::TextureFormat::Depth32Float,
                    depth_write_enabled: depth,
                    depth_compare: if depth { wgpu::CompareFunction::LessEqual } else { wgpu::CompareFunction::Always },
                    stencil: Default::default(),
                    bias: Default::default(),
                }),
                multisample: Default::default(),
                multiview: None,
                cache: None,
            })
        };
        let solid = pipeline(wgpu::PrimitiveTopology::TriangleList, true);
        let solid_no_depth = pipeline(wgpu::PrimitiveTopology::TriangleList, false);
        let lines = pipeline(wgpu::PrimitiveTopology::LineList, true);
        let lines_no_depth = pipeline(wgpu::PrimitiveTopology::LineList, false);
        Ok(Self {
            device,
            queue,
            adapter: adapter_name,
            solid,
            solid_no_depth,
            lines,
            lines_no_depth,
            object_uniform,
            grid_uniform,
            object_bind,
            grid_bind,
            targets: None,
            geometry: None,
        })
    }

    fn prepare(&mut self, snapshot: &Snapshot) {
        if self
            .targets
            .as_ref()
            .is_none_or(|t| t.width != snapshot.width || t.height != snapshot.height)
        {
            let (width, height) = (snapshot.width, snapshot.height);
            let size = wgpu::Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            };
            let texture = |label, format, usage| {
                self.device.create_texture(&wgpu::TextureDescriptor {
                    label: Some(label),
                    size,
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: wgpu::TextureDimension::D2,
                    format,
                    usage,
                    view_formats: &[],
                })
            };
            let color = texture(
                "offscreen color",
                FORMAT,
                wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::COPY_SRC,
            );
            let depth = texture(
                "offscreen depth",
                wgpu::TextureFormat::Depth32Float,
                wgpu::TextureUsages::RENDER_ATTACHMENT,
            );
            let row_bytes = (width * 4).div_ceil(wgpu::COPY_BYTES_PER_ROW_ALIGNMENT)
                * wgpu::COPY_BYTES_PER_ROW_ALIGNMENT;
            let staging = self.device.create_buffer(&wgpu::BufferDescriptor {
                label: Some("readback staging"),
                size: row_bytes as u64 * height as u64,
                usage: wgpu::BufferUsages::COPY_DST | wgpu::BufferUsages::MAP_READ,
                mapped_at_creation: false,
            });
            self.targets = Some(Targets {
                width,
                height,
                color_view: color.create_view(&Default::default()),
                color,
                depth_view: depth.create_view(&Default::default()),
                _depth: depth,
                staging,
                row_bytes,
            });
        }
        if self
            .geometry
            .as_ref()
            .is_none_or(|g| g.scene != snapshot.settings.scene)
        {
            let buffer = |label, vertices: &[Vertex]| {
                self.device
                    .create_buffer_init(&wgpu::util::BufferInitDescriptor {
                        label: Some(label),
                        contents: bytemuck::cast_slice(vertices),
                        usage: wgpu::BufferUsages::VERTEX,
                    })
            };
            self.geometry = Some(Geometry {
                scene: snapshot.settings.scene,
                triangles: buffer("triangles", &snapshot.mesh.triangles),
                edges: buffer("wire edges", &snapshot.mesh.edges),
                grid: buffer("grid", &snapshot.mesh.grid),
            });
        }
    }

    pub fn render(&mut self, snapshot: Snapshot) -> Result<Frame> {
        let started = Instant::now();
        self.prepare(&snapshot);
        let settings = snapshot.settings;
        let (vp, model) = settings.matrices(snapshot.width as f32 / snapshot.height as f32);
        let uniforms = |model: Mat4, checker: bool| Uniforms {
            view_projection: vp.to_cols_array_2d(),
            model: model.to_cols_array_2d(),
            options: [if checker { 1. } else { 0. }, 0., 0., 0.],
        };
        self.queue.write_buffer(
            &self.object_uniform,
            0,
            bytemuck::bytes_of(&uniforms(model, settings.checker)),
        );
        self.queue.write_buffer(
            &self.grid_uniform,
            0,
            bytemuck::bytes_of(&uniforms(Mat4::IDENTITY, false)),
        );
        let targets = self.targets.as_ref().unwrap();
        let geometry = self.geometry.as_ref().unwrap();
        let line_pipeline = if settings.depth {
            &self.lines
        } else {
            &self.lines_no_depth
        };
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("3D frame + readback"),
            });
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("3D scene"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &targets.color_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 11. / 255.,
                            g: 18. / 255.,
                            b: 27. / 255.,
                            a: 1.,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &targets.depth_view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.),
                        store: wgpu::StoreOp::Discard,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            pass.set_pipeline(line_pipeline);
            pass.set_bind_group(0, &self.grid_bind, &[]);
            pass.set_vertex_buffer(0, geometry.grid.slice(..));
            pass.draw(0..snapshot.mesh.grid.len() as u32, 0..1);
            pass.set_bind_group(0, &self.object_bind, &[]);
            if settings.wireframe {
                pass.set_vertex_buffer(0, geometry.edges.slice(..));
                pass.draw(0..snapshot.mesh.edges.len() as u32, 0..1);
            } else {
                pass.set_pipeline(if settings.depth {
                    &self.solid
                } else {
                    &self.solid_no_depth
                });
                pass.set_vertex_buffer(0, geometry.triangles.slice(..));
                pass.draw(0..snapshot.mesh.triangles.len() as u32, 0..1);
            }
        }
        encoder.copy_texture_to_buffer(
            wgpu::TexelCopyTextureInfo {
                texture: &targets.color,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
                aspect: wgpu::TextureAspect::All,
            },
            wgpu::TexelCopyBufferInfo {
                buffer: &targets.staging,
                layout: wgpu::TexelCopyBufferLayout {
                    offset: 0,
                    bytes_per_row: Some(targets.row_bytes),
                    rows_per_image: Some(targets.height),
                },
            },
            wgpu::Extent3d {
                width: targets.width,
                height: targets.height,
                depth_or_array_layers: 1,
            },
        );
        self.queue.submit(Some(encoder.finish()));
        let slice = targets.staging.slice(..);
        let (sender, receiver) = mpsc::sync_channel(1);
        slice.map_async(wgpu::MapMode::Read, move |result| {
            let _ = sender.send(result);
        });
        self.device.poll(wgpu::Maintain::Wait);
        receiver
            .recv()
            .context("GPU readback callback disconnected")??;
        let mapped = slice.get_mapped_range();
        let mut bgra = Vec::with_capacity((targets.width * targets.height * 4) as usize);
        for row in mapped.chunks_exact(targets.row_bytes as usize) {
            // Strip GPU row padding. RenderImage wants BGRA even though its
            // constructor wraps image::RgbaImage. The render target already matches.
            bgra.extend_from_slice(&row[..targets.width as usize * 4]);
        }
        drop(mapped);
        targets.staging.unmap();
        Ok(Frame {
            snapshot,
            bgra,
            render_ms: started.elapsed().as_secs_f32() * 1000.,
        })
    }
}

pub fn smoke() -> Result<()> {
    let mut renderer = pollster::block_on(Renderer::new())?;
    println!("Adapter: {}", renderer.adapter);
    std::fs::create_dir_all(".local")?;
    // Non-aligned width also exercises staging-row padding removal.
    let snapshot = |settings: Settings| Snapshot {
        mesh: scene::mesh(settings.scene),
        settings,
        width: 513,
        height: 384,
    };
    let save = |name: &str, frame: &Frame| -> Result<()> {
        anyhow::ensure!(
            frame.bgra.len() == (frame.snapshot.width * frame.snapshot.height * 4) as usize,
            "wrong image size"
        );
        let mut rgba = frame.bgra.clone();
        for pixel in rgba.chunks_exact_mut(4) {
            pixel.swap(0, 2);
        }
        image::save_buffer(
            format!(".local/{name}.png"),
            &rgba,
            frame.snapshot.width,
            frame.snapshot.height,
            image::ColorType::Rgba8,
        )?;
        println!("{name}: {:.2} ms, {} bytes", frame.render_ms, rgba.len());
        Ok(())
    };
    let cube = renderer.render(snapshot(Settings::default()))?;
    save("cube", &cube)?;
    let mut settings = Settings {
        scene: SceneKind::Intersections,
        yaw: 0.2,
        pitch: 0.15,
        ..Default::default()
    };
    let depth = renderer.render(snapshot(settings))?;
    save("intersection-depth", &depth)?;
    settings.depth = false;
    let no_depth = renderer.render(snapshot(settings))?;
    save("intersection-no-depth", &no_depth)?;
    let changed = depth
        .bgra
        .chunks_exact(4)
        .zip(no_depth.bgra.chunks_exact(4))
        .filter(|(a, b)| a != b)
        .count();
    anyhow::ensure!(
        changed > 100,
        "depth toggle did not materially change the output"
    );
    settings = Settings {
        checker: true,
        ..Default::default()
    };
    let checker = renderer.render(snapshot(settings))?;
    save("checker", &checker)?;
    anyhow::ensure!(
        cube.bgra != checker.bgra,
        "checker shader did not change the output"
    );
    for kind in [SceneKind::Cube, SceneKind::Intersections, SceneKind::Cubes] {
        for orthographic in [false, true] {
            for wireframe in [false, true] {
                let sample = snapshot(Settings {
                    scene: kind,
                    orthographic,
                    wireframe,
                    distance: 3.,
                    ..Default::default()
                });
                let drawing = scene::project(&sample);
                anyhow::ensure!(
                    drawing
                        .polygons
                        .iter()
                        .flat_map(|p| &p.points)
                        .chain(drawing.segments.iter().flat_map(|s| &s.points))
                        .flatten()
                        .all(|v| v.is_finite() && (-0.001..=1.001).contains(v)),
                    "invalid clipped coordinate"
                );
                renderer.render(sample)?;
            }
        }
    }
    println!(
        "PASS: scenes, clipping, projections, wireframe, shader, depth and row padding. Depth changed {changed} pixels."
    );
    Ok(())
}
