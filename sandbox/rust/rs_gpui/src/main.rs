use gpui::{
    App, Application, Bounds, Context, Window, WindowBounds, WindowOptions, div, prelude::*, px,
    rgb, size,
};

struct Counter {
    count: u64,
}

impl Render for Counter {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(rgb(0x1e1e2e))
            .text_color(rgb(0xffffff))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_4()
                    .p_6()
                    .rounded_lg()
                    .bg(rgb(0x313244))
                    .child("Hello, GPUI!")
                    .child(div().text_3xl().child(format!("Count: {}", self.count)))
                    .child(
                        div()
                            .id("increment")
                            .px_4()
                            .py_2()
                            .rounded_md()
                            .bg(rgb(0x89b4fa))
                            .hover(|style| style.bg(rgb(0x8fa5ca)))
                            .text_color(rgb(0x1e1e2e))
                            .cursor_pointer()
                            .child("Add one")
                            .on_click(cx.listener(|this, _, _, cx| {
                                this.count += 1;
                                cx.notify();
                            })),
                    ),
            )
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
            |_, cx| cx.new(|_| Counter { count: 0 }),
        )
        .expect("failed to open GPUI window");
        cx.activate(true);
    });
}
