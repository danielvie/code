use gpui::{App, AppContext, Bounds, Context, IntoElement, ParentElement, Render, Styled, Window, WindowOptions, div, px, size};
use gpui_platform::application;

struct HelloWorld;

impl Render for HelloWorld {
    fn render(
        &mut self,
        _window: &mut Window,
        _cx: &mut Context<Self>,
    ) -> impl IntoElement {
        div()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child("hello, world!!")
    }
}

fn main () {
    application().run(|cx: &mut App| {
        println!("GPUI started");

        let bounds = Bounds::centered(
            None, size(px(640.0), px(400.0)), cx);
        
        cx.open_window(
            WindowOptions {
                window_bounds: Some(gpui::WindowBounds::Windowed(bounds)),
                ..Default::default()
            }, 
            |_window, cx| cx.new(|_cx| HelloWorld),
        )
        .expect("Failed to open the window");
        
        cx.activate(true);
        
    });
}
