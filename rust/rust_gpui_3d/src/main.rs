mod gpu;
mod playground;
mod scene;

use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

fn main() -> anyhow::Result<()> {
    if std::env::args().any(|arg| arg == "--smoke") {
        return gpu::smoke();
    }
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1440.), px(940.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(1080.), px(760.))),
                ..Default::default()
            },
            |window, cx| {
                window.set_window_title("GPUI / 3D capability lab");
                cx.new(playground::Playground::new)
            },
        )
        .expect("could not open GPUI window");
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
    Ok(())
}
