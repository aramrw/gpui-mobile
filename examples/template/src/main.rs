use gpui::{prelude::*, App, WindowOptions, div};

fn main() {
    App::new().run(|cx| {
        cx.open_window(WindowOptions::default(), |_, cx| {
            cx.new(|_| HelloWorld)
        }).unwrap();
    });
}

struct HelloWorld;
impl Render for HelloWorld {
    fn render(&mut self, _cx: &mut ViewContext<Self>) -> impl IntoElement {
        div()
            .flex()
            .items_center()
            .justify_center()
            .size_full()
            .bg(gpui::white())
            .child("GPUI Mobile Template")
    }
}
