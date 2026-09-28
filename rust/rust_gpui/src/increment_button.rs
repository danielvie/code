use gpui::{App, ClickEvent, IntoElement, Window, div, prelude::*, rgb};

pub fn increment_button(
    on_click: impl Fn(&ClickEvent, &mut Window, &mut App) + 'static,
) -> impl IntoElement {
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
        .py_3()
        .on_click(on_click)
}
