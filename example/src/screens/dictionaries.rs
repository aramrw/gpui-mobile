use gpui::{
    Context, Entity, IntoElement, ParentElement, Styled,
    div, rgb, AsyncApp, InteractiveElement, AppContext,
};
use gpui_mobile::components::material::MaterialTheme;
use crate::GlobalYomichan;
use super::Router;
use yomichan_rs::settings::DictionaryOptions;

pub struct DictionariesState {}

impl DictionariesState {
    pub fn new(_window: &mut gpui::Window, _cx: &mut Context<Self>) -> Self {
        Self {}
    }

    pub fn toggle_dictionary_enabled(
        dict_name: String,
        new_state: bool,
        cx: &mut Context<Router>,
    ) {
        let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
        
        // Update in memory
        {
            let options_ptr = global_yomichan.read().options();
            let profile_ptr = options_ptr.read().get_current_profile().unwrap().clone();
            let mut profile_guard = profile_ptr.write();
            if let Some((_, dict_options)) = profile_guard
                .options_mut()
                .dictionaries_mut()
                .iter_mut()
                .find(|(_, opt)| opt.name == dict_name)
            {
                dict_options.enabled = new_state;
            }
        }

        // Persist
        cx.spawn(|_, cx: &mut AsyncApp| {
            let cx = cx.clone();
            async move {
                let _ = cx.read_global(|g: &GlobalYomichan, _| {
                    g.write().update_options()
                });
            }
        }).detach();
        
        cx.notify();
    }
}

pub fn render(_state: &Entity<DictionariesState>, router: &Router, cx: &mut Context<Router>) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    
    let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
    let dictionaries = {
        let options_ptr = global_yomichan.read().options();
        let profile_ptr = options_ptr.read().get_current_profile().unwrap().clone();
        let dicts = profile_ptr.read().options().dictionaries.clone();
        dicts
    };

    div()
        .flex()
        .flex_col()
        .size_full()
        .gap_4()
        .px_4()
        .py_4()
        .child(
            div()
                .text_xl()
                .text_color(rgb(theme.on_surface))
                .child("Dictionaries")
        )
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .children(dictionaries.into_iter().map(|(_, opt)| {
                    render_dictionary_card(opt, theme, cx)
                }))
        )
}

fn render_dictionary_card(opt: DictionaryOptions, theme: MaterialTheme, cx: &mut Context<Router>) -> impl IntoElement {
    let name = opt.name.clone();
    let enabled = opt.enabled;
    
    div()
        .flex()
        .flex_row()
        .items_center()
        .justify_between()
        .bg(rgb(theme.surface_container_high))
        .p_4()
        .rounded_xl()
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_lg()
                        .text_color(rgb(theme.on_surface))
                        .child(name.clone())
                )
        )
        .child(
            div()
                .px_4()
                .py_2()
                .rounded_full()
                .bg(rgb(if enabled { theme.primary } else { theme.on_surface_variant }))
                .text_color(rgb(if enabled { theme.on_primary } else { theme.on_surface }))
                .child(if enabled { "ON" } else { "OFF" })
                .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |_, _, _, cx| {
                    DictionariesState::toggle_dictionary_enabled(name.clone(), !enabled, cx);
                }))
        )
}
