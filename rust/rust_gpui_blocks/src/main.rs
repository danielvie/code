mod diagram_model;
mod diagram_prototype;
mod diagram_router;
mod diagram_selection;
mod diagram_store;
mod diagram_validate;
mod label_editor;

use diagram_prototype::DiagramPrototype;
use gpui::{App, Application, Bounds, WindowBounds, WindowOptions, prelude::*, px, size};

fn main() {
    let (store, snapshot) = diagram_store::Store::open(diagram_store::Store::default_path())
        .unwrap_or_else(|error| {
            eprintln!("Cannot open the diagram: {error}");
            std::process::exit(1);
        });
    Application::new().run(move |cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.), px(860.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                window_min_size: Some(size(px(960.), px(600.))),
                ..Default::default()
            },
            move |window, cx| {
                window.set_window_title("GPUI diagram PoC");
                cx.new(|cx| DiagramPrototype::new(window, cx, store, snapshot))
            },
        )
        .expect("failed to open GPUI window");
        cx.on_window_closed(|cx| {
            if cx.windows().is_empty() {
                cx.quit();
            }
        })
        .detach();
        cx.activate(true);
    });
}
