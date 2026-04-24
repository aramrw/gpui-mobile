use gpui::{
    div, rgb, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled,
};
use gpui_mobile::components::material::MaterialTheme;
use yomichan_rs::TermDictionaryEntry;

use super::Router;

pub struct AnkiRouter {
    pub drafts: Vec<TermDictionaryEntry>,
}

impl AnkiRouter {
    pub fn new() -> Self {
        Self { drafts: Vec::new() }
    }
}

pub fn render(
    _state: &Entity<AnkiRouter>,
    _router: &Router,
    cx: &mut Context<Router>,
) -> impl IntoElement {
    let dark_mode = _router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);

    div()
        .id("anki-router")
        .flex()
        .flex_col()
        .size_full()
        .p_4()
        .gap_4()
        .child(
            div()
                .text_2xl()
                .font_weight(gpui::FontWeight::BOLD)
                .text_color(rgb(theme.on_surface))
                .child("Anki Drafts"),
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child("Drafts will appear here.")
        )
        .child(
            div()
                .p_4()
                .rounded_lg()
                .bg(rgb(theme.primary))
                .text_color(rgb(theme.on_primary))
                .child("Push to Anki")
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(|_this, _, _, _| {
                    log::info!("Push to Anki clicked");
                }))
        )
}
