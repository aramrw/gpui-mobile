use gpui::{prelude::*, App, WindowOptions, Window, Context, div};

#[gpui_mobile::main]
fn main(cx: &mut App) {
    cx.open_window(WindowOptions::default(), |_window, cx| {
        cx.new(|_cx| HelloWorld)
    }).unwrap();
}

struct HelloWorld;

impl Render for HelloWorld {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<'_, Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(gpui::white())
            .text_color(gpui::black())
            .child("Hello, GPUI Mobile with Macro!")
    }
}
