# GPUI Canvas

A small desktop drawing app built with Rust and GPUI.

## Run

```sh
task run
```

Or use `cargo run` directly. Run `task --list` to see build, test, and lint commands.

Select Pencil, Line, Rectangle, or Ellipse, then click and drag on the white canvas. Pick a color and stroke thickness in the sidebar. For a cubic Bézier curve, click four points in order: start, first control point, second control point, end. The curve previews as you move the mouse after the third click.

Lines show draggable endpoints. Rectangles and ellipses show four corner handles. Drag a corner to resize the object while the opposite corner stays fixed. Bézier curves show their endpoints and two control points; drag any of these to reshape the curve. Handles can be dragged with any drawing tool selected.

Select Eraser to see its action radius around the pointer. Press `]` to increase the radius or `[` to decrease it. Click or drag across a drawn stroke to remove the entire object. Undo reverses drawing, editing, erasing, or clearing, and cancels a pending Bézier curve. Clear removes all strokes. Drawings stay in memory and are not saved to disk.
