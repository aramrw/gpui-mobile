use gpui::{
    AsyncApp, Context, Entity, IntoElement, ParentElement, Styled, Task,
    WeakEntity, div,
};
use gpui_mobile::components::material::search_bar::SearchBar;
use gpui_mobile::components::material::MaterialTheme;
use gpui_mobile::{set_text_input_callback, show_keyboard};
use regex::Regex;
use std::sync::LazyLock;
use unidecode::unidecode;
use yomichan_rs::TermSearchResultsSegment;

use crate::GlobalYomichan;
use super::Router;

pub static CLEANUP_REGEX: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"[[:punct:]]").unwrap());

pub struct SearchState {
    pub query: String,
    pub search_results: Option<Vec<TermSearchResultsSegment>>,
    pub search_task: Option<Task<()>>,
    pub selected_term_index: Option<usize>,
}

impl SearchState {
    pub fn new(_window: &mut gpui::Window, _cx: &mut Context<Self>) -> Self {
        Self {
            query: String::new(),
            search_results: None,
            search_task: None,
            selected_term_index: None,
        }
    }

    fn queue_search(&mut self, term: &str, cx: &mut Context<Self>, immediate: bool) {
        self.search_task.take();
        
        let trimmed_term = term.trim();
        let ascii_term = unidecode(trimmed_term);
        let clean_term = CLEANUP_REGEX.replace_all(&ascii_term, "");
        let term = clean_term.to_string();

        if term.is_empty() {
            self.search_results = None;
            cx.notify();
            return;
        }

        let new_search_task = cx.spawn(
            move |this_handle: WeakEntity<SearchState>, cx: &mut AsyncApp| {
                let mut cx_clone = cx.clone();
                async move {
                    if !immediate {
                        let _ = portable_async_sleep::async_sleep(std::time::Duration::from_millis(200)).await;
                    }

                    let ycd = cx_clone
                        .read_global(|g: &GlobalYomichan, _cx| g.0.clone());

                    let results = cx_clone
                        .background_executor()
                        .spawn(async move { ycd.write().search(&term) })
                        .await;

                    let _ = this_handle
                        .update(&mut cx_clone, |this, cx| {
                            this.search_results = results;
                            if this.selected_term_index.is_none() && this.search_results.is_some() {
                                this.selected_term_index = Some(0);
                            }
                            cx.notify();
                        });
                }
            },
        );
        self.search_task = Some(new_search_task);
    }
}

pub fn render(search_state: &Entity<SearchState>, router: &Router, cx: &mut Context<Router>) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);
    let state = search_state.read(cx);
    let query = state.query.clone();
    let results = state.search_results.clone();

    let search_state_handle = search_state.clone();
    let search_state_handle_for_trailing = search_state.clone();
    
    div()
        .flex()
        .flex_col()
        .size_full()
        .gap_4()
        .px_4()
        .py_4()
        .child(
            SearchBar::new(theme)
                .query(query.clone())
                .placeholder("Search term...")
                .on_tap(cx.listener(move |_, _, _, cx| {
                    let search_state_handle = search_state_handle.clone();
                    let async_cx = cx.to_async();
                    show_keyboard();
                    set_text_input_callback(Some(Box::new(move |text| {
                        let search_state_handle = search_state_handle.clone();
                        let text = text.to_string();
                        let async_cx = async_cx.clone();
                        
                        let _ = async_cx.update(|cx| {
                            search_state_handle.update(cx, |state, cx| {
                                state.query.push_str(&text);
                                let q = state.query.clone();
                                state.queue_search(&q, cx, false);
                                cx.notify();
                            });
                        });
                    })));
                }))
                .on_trailing_tap(cx.listener(move |_, _, _, cx| {
                    search_state_handle_for_trailing.update(cx, |state, cx| {
                        state.query.clear();
                        state.search_results = None;
                        cx.notify();
                    });
                }))
        )
        .child(
            div()
                .flex_1()
                .child(if let Some(results) = results {
                    div().child(format!("Found {} results", results.len()))
                } else {
                    div().child("No results yet")
                })
        )
}
