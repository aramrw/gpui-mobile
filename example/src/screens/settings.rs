use gpui::{
    Context, Entity, IntoElement, ParentElement, Styled,
    div, rgb, AppContext, InteractiveElement,
};
use gpui_mobile::components::material::MaterialTheme;
use crate::GlobalYomichan;
use super::Router;

pub struct SettingsState {}

impl SettingsState {
    pub fn new(_window: &mut gpui::Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }
}

pub fn render(_state: &Entity<SettingsState>, router: &Router, cx: &mut Context<Router>) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    let text_color = theme.on_surface;
    let sub_text = theme.on_surface_variant;
    let card_bg = theme.surface_container_high;

    div()
        .flex()
        .flex_col()
        .size_full()
        .gap_4()
        .px_4()
        .py_6()
        // ── Section: Appearance ───────────────────────────────────────────
        .child(section_header("Appearance", sub_text))
        .child(
            settings_card(card_bg)
                .child(toggle_row(
                    "Dark Mode",
                    "Use a dark colour scheme",
                    dark_mode,
                    text_color,
                    sub_text,
                    theme,
                    cx.listener(|this, _, _, cx| {
                        this.dark_mode = !this.dark_mode;
                        cx.notify();
                    }),
                ))
        )
        // ── Section: Profile ─────────────────────────────────────────────
        .child(section_header("Profile", sub_text))
        .child(
            settings_card(card_bg)
                .child(action_row(
                    "Current Profile",
                    &format!("Currently: {}", cx.read_global(|g: &GlobalYomichan, _| {
                        let opts = g.read().options();
                        let opts_guard = opts.read();
                        opts_guard.profiles.get_index(opts_guard.current_profile).map(|(k, _)| k.clone()).unwrap_or_else(|| "Default".to_string())
                    })),
                    theme,
                    cx.listener(|_, _, _, _| {
                        // TODO: Implement profile switching
                    }),
                ))
        )
        .child(
            div()
                .mt_4()
                .text_xs()
                .text_center()
                .text_color(rgb(sub_text))
                .child("Ported from ycd-rs")
        )
}

fn section_header(title: &str, color_val: u32) -> impl IntoElement {
    div()
        .text_xs()
        .text_color(rgb(color_val))
        .px_1()
        .child(title.to_string().to_uppercase())
}

fn settings_card(bg: u32) -> gpui::Div {
    div()
        .flex()
        .flex_col()
        .rounded_xl()
        .bg(rgb(bg))
        .overflow_hidden()
}

fn toggle_row(
    title: &str,
    description: &str,
    is_on: bool,
    text_color: u32,
    sub_text: u32,
    theme: MaterialTheme,
    handler: impl Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    let toggle_bg = if is_on { theme.primary } else { theme.on_surface_variant };
    let toggle_label = if is_on { "ON" } else { "OFF" };
    let toggle_text = if is_on { theme.on_primary } else { theme.on_surface_variant };

    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_3()
        .px_4()
        .py_3()
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .gap_1()
                .child(
                    div()
                        .text_base()
                        .text_color(rgb(text_color))
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(sub_text))
                        .child(description.to_string()),
                ),
        )
        .child(
            div()
                .px_3()
                .py_1()
                .rounded_full()
                .bg(rgb(toggle_bg))
                .text_xs()
                .text_color(rgb(toggle_text))
                .child(toggle_label),
        )
        .on_mouse_down(gpui::MouseButton::Left, handler)
}

fn action_row(
    title: &str,
    description: &str,
    theme: MaterialTheme,
    handler: impl Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static,
) -> impl IntoElement {
    div()
        .flex()
        .flex_row()
        .items_center()
        .gap_3()
        .px_4()
        .py_3()
        .child(
            div()
                .flex()
                .flex_col()
                .flex_1()
                .gap_1()
                .child(
                    div()
                        .text_base()
                        .text_color(rgb(theme.on_surface))
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(theme.on_surface_variant))
                        .child(description.to_string()),
                ),
        )
        .child(div().text_sm().text_color(rgb(theme.primary)).child("→"))
        .on_mouse_down(gpui::MouseButton::Left, handler)
}
