use gpui::{App, AppContext, WindowOptions};

pub mod demos;
pub mod screens;

#[gpui_mobile::main]
fn main(cx: &mut App) {
    let initial_screen = match gpui_mobile::packages::deeplink::get_initial_link() {
        Ok(Some(url)) => screens::Screen::from_deeplink_url(&url).unwrap_or_default(),
        _ => screens::Screen::default(),
    };

    cx.open_window(WindowOptions::default(), |window, cx| {
        cx.new(|cx| screens::Router::with_initial_screen(initial_screen, window, cx))
    })
    .unwrap();
}
