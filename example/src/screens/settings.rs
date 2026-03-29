use gpui::{
    AsyncApp, Context, Entity, IntoElement, ParentElement, Render, SharedString, Styled,
    WeakEntity, div, px, rgb, AppContext, InteractiveElement, StatefulInteractiveElement,
};
use gpui_mobile::components::material::MaterialTheme;
use crate::GlobalYomichan;
use super::Router;

pub struct SettingsState {}

impl SettingsState {
    pub fn new(_window: &mut gpui::Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn switch_profile(name: String, cx: &mut Context<Router>) {
        let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
        {
            let ycd = global_yomichan.read();
            let opts = ycd.options();
            let mut opts_guard = opts.write();
            if let Some(idx) = opts_guard.profiles.get_index_of(&name) {
                opts_guard.current_profile = idx;
            }
            let _ = ycd.update_options();
        }
        cx.notify();
    }
}

pub fn render(_state: &Entity<SettingsState>, router: &Router, cx: &mut Context<Router>) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    let text_color = theme.on_surface;
    let sub_text = theme.on_surface_variant;
    let card_bg = theme.surface_container_high;

    let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
    let (profiles, current_profile_idx) = {
        let ycd = global_yomichan.read();
        let opts_ptr = ycd.options();
        let opts = opts_ptr.read();
        (opts.profiles.keys().cloned().collect::<Vec<String>>(), opts.current_profile)
    };

    div()
        .id("settings-page")
        .flex()
        .flex_col()
        .size_full()
        .gap_4()
        .px_4()
        .py_6()
        .overflow_y_scroll()
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
        // ── Section: Profiles ─────────────────────────────────────────────
        .child(section_header("Profiles", sub_text))
        .child(
            settings_card(card_bg)
                .children(profiles.into_iter().enumerate().map(|(i, name)| {
                    let active = i == current_profile_idx;
                    let name_clone = name.clone();
                    action_row(
                        &name,
                        if active { "Active" } else { "Switch to this profile" },
                        active,
                        theme,
                        cx.listener(move |_, _, _, cx| {
                            SettingsState::switch_profile(name_clone.clone(), cx);
                        }),
                    )
                }))
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
    let toggle_text = if is_on { theme.on_primary } else { theme.on_surface };

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
    active: bool,
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
                        .text_color(rgb(if active { theme.primary } else { theme.on_surface }))
                        .child(title.to_string()),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(theme.on_surface_variant))
                        .child(description.to_string()),
                ),
        )
        .child(if active {
            div().text_sm().text_color(rgb(theme.primary)).child("✓")
        } else {
            div().text_sm().text_color(rgb(theme.on_surface_variant)).child("→")
        })
        .on_mouse_down(gpui::MouseButton::Left, handler)
}
