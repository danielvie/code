# GPUI 3D capability lab

An exploratory native Rust app with two synchronized viewports. This is a rendering experiment, not a production 3D engine.

- **01, GPUI canvas.** Rust projects 3D geometry to 2D. GPUI draws the resulting paths. Lighting is calculated per face, and triangles are sorted back to front. Closed cubes use CPU backface culling; the intersecting sheets remain double-sided.
- **02, wgpu + GPUI image.** A separate wgpu device draws the same geometry with a depth buffer and a WGSL shader. The app renders directly to a BGRA texture, reads the pixels back to CPU memory, and displays them through GPUI's image API.

GPUI is GPU-accelerated in both cases. The difference is who handles the 3D rendering.

## Run

```sh
task run
```

Or `cargo run`. For a more representative performance check, use `task release`.

The project pins GPUI 0.2.2, matching the adjacent GPUI project. It uses wgpu 24. Windows needs the Rust MSVC toolchain, Visual Studio C++ build tools, and working graphics drivers. The first build downloads and compiles a substantial dependency tree.

## Try this sequence

1. Start with **Cube**. Drag either pane to orbit both cameras. Scroll to zoom. Rotation pauses while dragging and resumes on release if enabled.
2. Toggle **Perspective / Orthographic** and **Solid faces / Wireframe**. These settings affect both panes. Wireframe displays triangle edges, including each quad's diagonal; it is not hidden-line rendering.
3. Select **Intersections**. This pauses rotation and chooses a useful starting camera. Orbit slowly. The canvas pane sorts whole triangles, which cannot correctly resolve intersecting faces. The wgpu pane resolves visibility per pixel.
4. Turn **Depth off** on the right to see why the GPU depth buffer matters. This disables depth testing, not a switch to CPU-style sorting. Turn it back on afterward.
5. Enable **Checker** on the right. The fragment shader computes a procedural checker pattern per pixel. It is not an image texture, and the left renderer intentionally has no matching feature.
6. Select **125 cubes** for 1,500 triangles. Compare interaction with the smaller scenes.
7. Click the resolution button to cycle **320 / 640 / 960 px**. This is the maximum offscreen dimension, not the window size. Lower settings reduce readback traffic but make the right pane softer. The canvas pane stays at the window's native resolution.

**Reset view** resets camera, object rotation, projection, drawing mode, checker, depth, and image resolution for the selected scene. It preserves the play/pause state.

## What the measurements mean

| Display | Included | Not included |
| --- | --- | --- |
| Left timing | CPU projection, clipping, triangle sorting, GPUI path construction and recording | GPUI's later GPU rendering and presentation |
| Right timing | Offscreen preparation, submission, GPU wait, readback, row-padding removal | GPUI's image upload, rendering and presentation |
| Image wrap + eviction | Creating the GPUI image wrapper and evicting the old atlas entry | Uploading the new image to GPUI's GPU |
| Paired updates/s | Completed snapshots delivered to both panes | Display refresh rate or standalone renderer throughput |

These are wall-clock measurements of different work, **not a fair GPU benchmark**. Debug builds also distort CPU costs. The right renderer uses single-sample rendering; its edges can look rougher than GPUI's antialiased paths.

The UI targets 30 Hz and permits only one in-flight frame. Actual delivered frequency can be lower, and short measurement windows can fluctuate around the target. Both panes display the camera and object state from the same completed GPU frame. This prevents misleading side-by-side comparisons, but it also means a slow right renderer limits the left pane's update frequency. Paused scenes render on demand after input or resize.

The footer reports unpadded image bytes per frame. Those bytes travel back from wgpu, then are uploaded to GPUI again. Staging rows can include alignment padding. There is no zero-copy texture sharing here.

If wgpu initialization fails, the app displays the error and keeps the GPUI canvas interactive. GPUI itself still needs a supported graphics device to open the window.

## Where each approach fits

Use projected GPUI paths for modest wireframes, spatial diagrams, axes, and simple illustrative objects. You own projection, clipping, shading, picking, and visibility. Increasing mesh complexity increases CPU work and GPUI path count. The grid is drawn underneath faces on the left, rather than depth-tested.

Use a separate renderer when you need reliable mesh visibility, fragment shaders, textures, or more advanced lighting. This example implements depth testing, flat lighting, and a procedural shader. Model loading, shadows, picking, physically based materials, and transparency are not implemented.

The image bridge is easy to understand and portable at the pixel level, but its copying cost matters for large or high-refresh viewports. A production integration may need platform-specific texture sharing or changes to GPUI's renderer. GPUI 0.2.2 does not expose a portable custom wgpu render-pass callback through `canvas`.

## Files

- `src/scene.rs`: shared geometry, camera matrices, CPU clipping and projection.
- `src/playground.rs`: split UI, controls, paired snapshots, GPUI path painting, image lifetime.
- `src/gpu.rs`: offscreen device, reusable buffers and textures, worker thread, readback, smoke check.
- `src/scene.wgsl`: GPU transforms, flat lighting, checker fragment shader.

The worker reuses its device, pipelines, geometry buffers, depth/color textures, and staging buffer. It rebuilds geometry buffers only when the scene changes, and render targets only when their dimensions change. Replaced GPUI images are explicitly evicted from the sprite atlas so animation does not accumulate cached frames.

## Validation

```sh
task check
task lint
task smoke
```

`task smoke` needs a working GPU but does not open a window. It renders all scenes with both projection modes and wireframe/solid modes, checks CPU clipping coordinates, verifies that depth and checker toggles change pixels, and exercises a non-aligned readback width. It saves PNGs under `.local/` for visual inspection. It does not verify native window layout or mouse interaction.
