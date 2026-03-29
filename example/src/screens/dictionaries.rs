use gpui::{
    AsyncApp, Context, Entity, IntoElement, ParentElement, SharedString, Styled,
    div, px, rgb, AppContext, InteractiveElement, StatefulInteractiveElement,
};
use gpui_mobile::components::material::MaterialTheme;
use gpui_mobile::packages::file_selector::{open_file, OpenFileOptions, TypeGroup};
use crate::GlobalYomichan;
use super::Router;
use yomichan_rs::settings::DictionaryOptions;
use std::path::PathBuf;

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

    pub fn set_language(lang: String, cx: &mut Context<Router>) {
        let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
        {
            let ycd = global_yomichan.write();
            let _ = ycd.set_language(&lang);
            let _ = ycd.update_options();
        }
        cx.notify();
    }

    pub fn import_dictionary(cx: &mut Context<Router>) {
        let options = OpenFileOptions {
            accept_type_groups: vec![TypeGroup {
                label: "Yomichan Dictionary".into(),
                extensions: vec!["zip".into()],
                ..Default::default()
            }],
            ..Default::default()
        };

        match open_file(&options) {
            Ok(Some(file)) => {
                let path = PathBuf::from(file.path);
                let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
                
                cx.spawn(|_, cx: &mut AsyncApp| {
                    let cx = cx.clone();
                    async move {
                        // import_dictionaries takes a slice of paths
                        let _ = global_yomichan.read().import_dictionaries(&[path]);
                        let _ = cx.read_global(|g: &GlobalYomichan, _| {
                            g.write().update_options()
                        });
                    }
                }).detach();
            }
            _ => {}
        }
        cx.notify();
    }
}

pub fn render(_state: &Entity<DictionariesState>, router: &Router, cx: &mut Context<Router>) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    
    let global_yomichan = cx.read_global(|g: &GlobalYomichan, _cx| g.clone());
    let (dictionaries, current_lang) = {
        let ycd = global_yomichan.read();
        let options_ptr = ycd.options();
        let profile_ptr = options_ptr.read().get_current_profile().unwrap().clone();
        let profile = profile_ptr.read();
        (profile.options().dictionaries.clone(), profile.options().general().language.clone())
    };

    div()
        .id("dictionaries-page")
        .flex()
        .flex_col()
        .size_full()
        .gap_4()
        .px_4()
        .py_4()
        .overflow_y_scroll()
        .child(
            div()
                .flex()
                .flex_row()
                .items_center()
                .justify_between()
                .child(
                    div()
                        .text_xl()
                        .text_color(rgb(theme.on_surface))
                        .child("Dictionaries")
                )
                .child(
                    div()
                        .px_4()
                        .py_2()
                        .bg(rgb(theme.primary))
                        .text_color(rgb(theme.on_primary))
                        .rounded_lg()
                        .child("Import ZIP")
                        .on_mouse_down(gpui::MouseButton::Left, cx.listener(|_, _, _, cx| {
                            DictionariesState::import_dictionary(cx);
                        }))
                )
        )
        // ── Language Section ─────────────────────────────────────────────
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2()
                .child(div().text_sm().text_color(rgb(theme.on_surface_variant)).child("LANGUAGE"))
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .gap_2()
                        .child(lang_chip("Japanese", "ja", &current_lang, theme, cx))
                        .child(lang_chip("English", "en", &current_lang, theme, cx))
                        .child(lang_chip("Spanish", "es", &current_lang, theme, cx))
                )
        )
        // ── Dictionaries List ───────────────────────────────────────────
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

fn lang_chip(label: &str, iso: &str, current: &str, theme: MaterialTheme, cx: &mut Context<Router>) -> impl IntoElement {
    let active = current == iso;
    let iso = iso.to_string();
    div()
        .px_3()
        .py_1()
        .rounded_full()
        .bg(rgb(if active { theme.primary_container } else { theme.surface_container_high }))
        .text_color(rgb(if active { theme.on_primary_container } else { theme.on_surface }))
        .text_sm()
        .child(label.to_string())
        .on_mouse_down(gpui::MouseButton::Left, cx.listener(move |_, _, _, cx| {
            DictionariesState::set_language(iso.clone(), cx);
        }))
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
