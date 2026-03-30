use gpui::{
    div, rgb, AsyncApp, Context, Entity, InteractiveElement, IntoElement, ParentElement,
    StatefulInteractiveElement, Styled, Task, WeakEntity,
};
use gpui_mobile::components::material::search_bar::SearchBar;
use gpui_mobile::components::material::MaterialTheme;
use gpui_mobile::{set_text_input_callback, show_keyboard};
use regex::Regex;
use std::sync::LazyLock;
use yomichan_rs::TermSearchResultsSegment;

use super::Router;
use crate::GlobalYomichan;

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
        let clean_term = CLEANUP_REGEX.replace_all(trimmed_term, "");
        let term = clean_term.to_string();

        if term.is_empty() {
            self.search_results = None;
            cx.notify();
            return;
        }

        let new_search_task = cx.spawn(
            move |this_handle: WeakEntity<SearchState>, cx: &mut AsyncApp| {
                let mut cx = cx.clone();
                async move {
                    if !immediate {
                        let _ = portable_async_sleep::async_sleep(
                            std::time::Duration::from_millis(200),
                        )
                        .await;
                    }

                    let ycd = cx.read_global(|g: &GlobalYomichan, _cx| g.0.clone());

                    let results = cx
                        .background_executor()
                        .spawn(async move { ycd.write().search(&term) })
                        .await;

                    let _ = this_handle.update(&mut cx, |this, cx| {
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

pub fn render(
    search_state: &Entity<SearchState>,
    router: &Router,
    cx: &mut Context<Router>,
) -> impl IntoElement {
    let dark_mode = router.dark_mode;
    let theme = MaterialTheme::from_appearance(dark_mode);

    let (query, results, selected_index) = {
        let state = search_state.read(cx);
        (
            state.query.clone(),
            state.search_results.clone(),
            state.selected_term_index,
        )
    };

    let search_state_handle = search_state.clone();
    let search_state_handle_for_trailing = search_state.clone();

    div()
        .id("search-page")
        .flex()
        .flex_col()
        .size_full()
        .gap_2()
        .px_2()
        .py_2()
        .child(
            SearchBar::new(theme)
                .query(query.clone())
                .placeholder("Search term...")
                .on_tap(cx.listener(move |_, _, _, cx| {
                    let search_state_handle = search_state_handle.clone();
                    let async_cx = cx.to_async();
                    show_keyboard();
                    set_text_input_callback(Some(Box::new(move |text| {
                        if text == "\n" {
                            gpui_mobile::hide_keyboard();
                            return;
                        }
                        if text == "\x08" {
                            let search_state_handle = search_state_handle.clone();
                            let async_cx = async_cx.clone();
                            async_cx.update(move |cx| {
                                let _ = search_state_handle.update(cx, |state, cx| {
                                    state.query.pop();
                                    let q = state.query.clone();
                                    state.queue_search(&q, cx, true);
                                    cx.notify();
                                });
                            });
                            return;
                        }
                        let search_state_handle = search_state_handle.clone();
                        let text = text.to_string();

                        let async_cx = async_cx.clone();
                        async_cx.update(move |cx| {
                            let _ = search_state_handle.update(cx, |state, cx| {
                                state.query.push_str(&text);
                                let q = state.query.clone();
                                state.queue_search(&q, cx, true);
                                cx.notify();
                            });
                        });
                    })));
                }))
                .on_trailing_tap(cx.listener(move |_, _, _, cx| {
                    search_state_handle_for_trailing.update(cx, |state, cx| {
                        state.query.clear();
                        state.search_results = None;
                        state.selected_term_index = None;
                        cx.notify();
                    });
                })),
        )
        .child(render_segment_selector(search_state, theme, cx))
        .child(
            div()
                .id("search-results")
                .flex_1()
                .overflow_y_scroll()
                .on_mouse_down(
                    gpui::MouseButton::Left,
                    cx.listener(|_, _, _, _| {
                        gpui_mobile::hide_keyboard();
                    }),
                )
                .child(if let Some(results) = results {
                    if results.is_empty() {
                        div().px_4().child("No results found.")
                    } else if let Some(selected_index) = selected_index {
                        if let Some(segment) = results.get(selected_index) {
                            div().flex().flex_col().gap_1().px_2().children(
                                segment
                                    .results
                                    .as_ref()
                                    .map(|r| {
                                        r.dictionary_entries
                                            .iter()
                                            .map(|entry| render_dictionary_entry(entry, theme))
                                            .collect::<Vec<_>>()
                                    })
                                    .unwrap_or_default(),
                            )
                        } else {
                            div().px_4().child("Select a word above")
                        }
                    } else {
                        div().px_4().child("Select a word above")
                    }
                } else {
                    div() /* .px_4().child("Type to search...") */
                }),
        )
}

fn render_segment_selector(
    search_state: &Entity<SearchState>,
    theme: MaterialTheme,
    cx: &mut Context<Router>,
) -> impl IntoElement {
    let (results, selected_index) = {
        let state = search_state.read(cx);
        (state.search_results.clone(), state.selected_term_index)
    };
    let search_state_handle = search_state.clone();

    if let Some(results) = results {
        div()
            .flex()
            .flex_row()
            .flex_wrap()
            .gap_2()
            .px_2()
            .py_2()
            .children(results.into_iter().enumerate().filter_map(|(i, segment)| {
                // Filter out segments that are just whitespace or empty
                if segment.text.trim().is_empty() {
                    return None;
                }
                let is_selected = Some(i) == selected_index;
                let search_state_handle = search_state_handle.clone();
                let text = segment.text.clone();

                Some(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_xl()
                        .bg(rgb(if is_selected {
                            theme.primary_container
                        } else {
                            theme.surface_container_high
                        }))
                        .text_color(rgb(if is_selected {
                            theme.on_primary_container
                        } else {
                            theme.on_surface
                        }))
                        .text_xl()
                        .font_weight(gpui::FontWeight::BOLD)
                        .child(text)
                        .on_mouse_down(
                            gpui::MouseButton::Left,
                            cx.listener(move |_, _, _, cx| {
                                let _ = search_state_handle.update(cx, |state, cx| {
                                    state.selected_term_index = Some(i);
                                    cx.notify();
                                });
                            }),
                        ),
                )
            }))
    } else {
        div()
    }
}

fn render_dictionary_entry(
    entry: &yomichan_rs::TermDictionaryEntry,
    theme: MaterialTheme,
) -> impl IntoElement {
    let headword = entry.headwords.first();
    let term = headword.map(|h| h.term.clone()).unwrap_or_default();
    let reading = headword.map(|h| h.reading.clone()).unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .bg(rgb(theme.surface_container_high))
        .p_4()
        .rounded_2xl()
        .gap_2()
        .child(
            div()
                .flex()
                .flex_col()
                .child(
                    div()
                        .text_sm()
                        .text_color(rgb(theme.secondary))
                        .child(reading),
                )
                .child(
                    div()
                        .text_2xl()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(rgb(theme.primary))
                        .child(term),
                ),
        )
        .children(entry.definitions.iter().flat_map(|def| {
            def.entries
                .iter()
                .map(|gloss| {
                    div()
                        .text_lg()
                        .text_color(rgb(theme.on_surface))
                        .child(gloss.plain_text.clone())
                })
                .collect::<Vec<_>>()
        }))
}

fn render_search_result(res: TermSearchResultsSegment, theme: MaterialTheme) -> impl IntoElement {
    // This function is now deprecated in favor of segmented rendering above,
    // but kept for reference or single-result cases if needed.
    let entries = res
        .results
        .as_ref()
        .map(|r| r.dictionary_entries.clone())
        .unwrap_or_default();

    div()
        .flex()
        .flex_col()
        .bg(rgb(theme.surface_container_high))
        .p_4()
        .rounded_xl()
        .gap_2()
        .child(
            div().flex().flex_row().justify_between().child(
                div()
                    .text_lg()
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(theme.on_surface))
                    .child(res.text.clone()),
            ),
        )
        .children(entries.into_iter().map(|entry| {
            let headword = entry
                .headwords
                .first()
                .map(|h| format!("{} [{}]", h.term, h.reading))
                .unwrap_or_default();
            div()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_sm()
                        .font_weight(gpui::FontWeight::BOLD)
                        .text_color(rgb(theme.primary))
                        .child(headword),
                )
                .children(entry.definitions.into_iter().flat_map(|def| {
                    def.entries.into_iter().map(|gloss| {
                        div()
                            .text_xs()
                            .text_color(rgb(theme.on_surface))
                            .child(gloss.plain_text)
                    })
                }))
        }))
}
