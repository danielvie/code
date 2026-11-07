# GPUI Bezier canvas

A cubic Bezier curve drawn with GPUI's canvas API. Drag the two filled blue anchor circles or the two hollow orange control circles. Guide lines connect each anchor to its control point.

Tab and Shift+Tab select a point. Arrow keys move the selected point by 1 pixel; hold Shift to move by 10 pixels. Points stay inside the canvas and scale with the window.

Requires Rust and [Task](https://taskfile.dev/). Run from this directory:

```sh
task run
```

Use `task test` to check the app and run the canvas interaction tests, or `task clear` to remove build artifacts. Without Task, run `cargo run` directly.

The drawing and interaction code is in `src/main.rs`. The canvas prepaint callback records its bounds so mouse coordinates can be mapped to the curve's normalized coordinates.

API references for GPUI 0.2.2:
- [Canvas callbacks](https://docs.rs/gpui/0.2.2/gpui/fn.canvas.html)
- [Cubic Bezier drawing](https://docs.rs/gpui/0.2.2/gpui/struct.PathBuilder.html#method.cubic_bezier_to)
- [Mouse and keyboard event handlers](https://docs.rs/gpui/0.2.2/gpui/trait.InteractiveElement.html)
