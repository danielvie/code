#[path = "../src/drag_drop.rs"]
mod drag_drop;
#[path = "../src/increment_button.rs"]
mod increment_button;

use drag_drop::DragDrop;
use gpui::{
    App, Application, Bounds, Context, Entity, Window, WindowBounds, WindowOptions, div,
    prelude::*, px, rgb, size,
};
use increment_button::increment_button;

// GPUI owns this view as an Entity<Counter>. Its fields hold the UI's state.
struct Counter {
    count: i32,
    drag_drop: Entity<DragDrop>,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_4()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xffffff))
            .child("My first GPUI app")
            .child(div().text_3xl().child(format!("Count: {}", self.count)))
            .child(increment_button(cx.listener(
                |this, _event, _window, cx| {
                    this.count += 1;
                    cx.notify(); // Tell GPUI to render this view again.
                },
            )))
            .child(self.drag_drop.clone())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(400.0), px(300.0)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| {
                cx.new(|cx| Counter {
                    count: 0,
                    drag_drop: cx.new(|_| DragDrop::new()),
                })
            },
        )
        .expect("failed to open GPUI window");
        cx.activate(true);
    });
}
