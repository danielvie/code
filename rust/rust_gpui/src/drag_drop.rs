use gpui::{Context, Render, Window, div, prelude::*, rgb};

pub struct DragDrop {
    dropped: bool,
}

impl DragDrop {
    pub fn new() -> Self {
        Self { dropped: false }
    }
}

struct DragItem;

struct DragPreview;

impl Render for DragPreview {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .px_4()
            .py_2()
            .rounded_md()
            .bg(rgb(0xb4d0ff))
            .text_color(rgb(0x1e1e2e))
            .shadow_md()
            .child("Drag me")
    }
}

impl Render for DragDrop {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .when(!self.dropped, |element| {
                element.child(
                    div()
                        .id("drag-item")
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .bg(rgb(0x89b4fa))
                        .text_color(rgb(0x1e1e2e))
                        .cursor_pointer()
                        .hover(|style| style.bg(rgb(0xb4d0ff)))
                        .active(|style| style.bg(rgb(0x6c8fd1)))
                        .child("Drag me")
                        .on_drag(DragItem, |_, _, _, cx| cx.new(|_| DragPreview)),
                )
            })
            .child(
                div()
                    .id("drop-zone")
                    .w(gpui::px(220.0))
                    .h(gpui::px(80.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .rounded_md()
                    .border_2()
                    .border_color(rgb(0x89b4fa))
                    .bg(rgb(0x313244))
                    .drag_over::<DragItem>(|style, _, _, _| {
                        style.bg(rgb(0x45475a)).border_color(rgb(0xa6e3a1))
                    })
                    .when(self.dropped, |element| {
                        element.bg(rgb(0x314d40)).border_color(rgb(0xa6e3a1))
                    })
                    .child(if self.dropped {
                        "Dropped!"
                    } else {
                        "Drop here"
                    })
                    .on_drop(cx.listener(|this, _: &DragItem, _, cx| {
                        this.dropped = true;
                        cx.notify();
                    })),
            )
            .when(self.dropped, |element| {
                element.child(
                    div()
                        .id("reset-drag-drop")
                        .px_4()
                        .py_2()
                        .rounded_md()
                        .bg(rgb(0x89b4fa))
                        .text_color(rgb(0x1e1e2e))
                        .cursor_pointer()
                        .hover(|style| style.bg(rgb(0xb4d0ff)))
                        .child("Reset")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.dropped = false;
                            cx.notify();
                        })),
                )
            })
    }
}
