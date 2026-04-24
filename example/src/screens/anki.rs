use gpui::{
    div, rgb, Context, Entity, InteractiveElement, IntoElement, ParentElement, Render, Styled,
};
use gpui_mobile::components::material::MaterialTheme;
use crate::GlobalPendingCards;

use super::Router;

pub struct AnkiRouter {
    // We don't store drafts here anymore, we read from GlobalPendingCards
}

impl AnkiRouter {
    pub fn new() -> Self {
        Self {}
    }
}

pub fn render(
    _state: &Entity<AnkiRouter>,
    router: &Router,
    cx: &mut Context<Router>,
) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    let pending_cards = cx.global::<GlobalPendingCards>().clone();
    let drafts = pending_cards.read().clone();

    let mut list = Vec::new();
    for entry in drafts {
        let term = entry.headwords.first().map(|h| h.term.clone()).unwrap_or_default();
        list.push(
            div()
                .p_2()
                .bg(rgb(theme.surface_container))
                .rounded_lg()
                .child(term)
        );
    }

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
                .children(list)
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
