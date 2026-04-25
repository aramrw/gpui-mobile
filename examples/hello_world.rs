use gpui::{prelude::*, App, Application, WindowOptions, Window, Context, div};
use std::rc::Rc;

fn main() {
    #[cfg(target_os = "macos")]
    let platform = Rc::new(gpui_macos::MacPlatform::new(false));
    
    #[cfg(not(target_os = "macos"))]
    compile_error!("This example currently only supports macOS for host execution.");

    Application::with_platform(platform).run(|cx: &mut App| {
        cx.open_window(WindowOptions::default(), |_window, cx| {
            cx.new(|_cx| HelloWorld)
        }).unwrap();
    });
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
            .child("Hello, GPUI Mobile!")
    }
}
