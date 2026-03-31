//! Material Design 3 Selectable Text component (View version).
//!
//! Provides text that can be selected with a Monokakido-style long-press.

use gpui::{
    div, px, rgb, IntoElement, MouseButton, ParentElement, Render, SharedString, Styled, Window, App,
    InteractiveElement, Context as ViewContext, Pixels, Point, Bounds, ElementId, canvas,
    prelude::FluentBuilder, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
};

use super::theme::{color, MaterialTheme};
use crate::GlobalHoverState;

pub struct SelectableTextView {
    text: SharedString,
    theme: MaterialTheme,
    on_lookup: Option<std::rc::Rc<dyn Fn(&str, &mut App)>>,
    /// The final resolved selection range (start_byte, end_byte)
    selection_range: Option<(usize, usize)>,
    /// The starting character range where the long-press began
    anchor_range: Option<(usize, usize)>,
    char_bounds: Vec<(usize, usize, Bounds<Pixels>)>,
}

impl SelectableTextView {
    pub fn new(text: impl Into<SharedString>, theme: MaterialTheme, _cx: &mut ViewContext<Self>) -> Self {
        Self {
            text: text.into(),
            theme,
            on_lookup: None,
            selection_range: None,
            anchor_range: None,
            char_bounds: Vec::new(),
        }
    }

    pub fn on_lookup(&mut self, handler: impl Fn(&str, &mut App) + 'static) {
        self.on_lookup = Some(std::rc::Rc::new(handler));
    }

    fn hit_test(&self, point: Point<Pixels>) -> Option<(usize, usize)> {
        for (start, end, bounds) in &self.char_bounds {
            if bounds.contains(&point) {
                return Some((*start, *end));
            }
        }
        None
    }

    /// Expand a character range to word boundaries if it's not CJK.
    fn expand_to_word_boundaries(&self, range: (usize, usize)) -> (usize, usize) {
        let (start, end) = range;
        let text = self.text.as_ref();
        
        // If the character is CJK, don't expand
        if let Some(c) = text[start..end].chars().next() {
            if is_cjk(c) {
                return range;
            }
        }

        // Expand backwards to find word start
        let mut word_start = start;
        for (idx, c) in text[..start].char_indices().rev() {
            if !c.is_alphanumeric() {
                break;
            }
            word_start = idx;
        }

        // Expand forwards to find word end
        let mut word_end = end;
        for (idx, c) in text[end..].char_indices() {
            if !c.is_alphanumeric() {
                break;
            }
            word_end = end + idx + c.len_utf8();
        }

        (word_start, word_end)
    }

    fn update_global_hover(&self, range: (usize, usize), cx: &mut ViewContext<Self>) {
        let (start, end) = range;
        let text = self.text.as_ref();
        
        // The "active" point is where the finger is (the 'end' of the selection during a drag)
        let active_idx = end;

        // Extract a window of the SELECTED text (max 10 chars) ending at the finger
        let selected_text_start = text[start..end].char_indices().rev().take(10).last().map(|(idx, _)| start + idx).unwrap_or(start);
        let mut hovered_text = text[selected_text_start..end].to_string();
        if selected_text_start > start {
            hovered_text = format!("...{}", hovered_text);
        }

        // Extract context around the active point
        // 5 chars before the selection start (or before the window start)
        let before_window = selected_text_start;
        let context_before_start = text[..before_window].char_indices().rev().take(5).last().map(|(idx, _)| idx).unwrap_or(0);
        let context_before = text[context_before_start..before_window].to_string();

        // 5 chars after the finger
        let context_after_end = text[end..].char_indices().take(5).last().map(|(idx, c)| end + idx + c.len_utf8()).unwrap_or(text.len());
        let context_after = text[end..context_after_end].to_string();

        cx.set_global(GlobalHoverState {
            text: SharedString::from(hovered_text),
            context_before: SharedString::from(context_before),
            context_after: SharedString::from(context_after),
            range,
        });
    }
}

fn is_cjk(c: char) -> bool {
    matches!(c, 
        '\u{3040}'..='\u{309F}' | // Hiragana
        '\u{30A0}'..='\u{30FF}' | // Katakana
        '\u{4E00}'..='\u{9FFF}' | // Kanji
        '\u{AC00}'..='\u{D7AF}' | // Hangul
        '\u{FF00}'..='\u{FFEF}'   // Full-width forms
    )
}

impl Render for SelectableTextView {
    fn render(&mut self, _window: &mut Window, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let text_color = color(self.theme.on_surface);
        let highlight_bg = rgb(0x4285F4); // Monokakido blue
        
        let text_hash = {
            use std::collections::hash_map::DefaultHasher;
            use std::hash::{Hash, Hasher};
            let mut hasher = DefaultHasher::new();
            self.text.hash(&mut hasher);
            hasher.finish()
        };

        let entity = cx.entity().clone();
        // Clear char bounds before re-rendering
        self.char_bounds.clear();

        div()
            .id(ElementId::Name(SharedString::from(format!("selectable-root-{}", text_hash))))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .w_full()
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, _, cx| {
                if let Some(hit) = this.hit_test(event.position) {
                    let snapped = this.expand_to_word_boundaries(hit);
                    this.anchor_range = Some(snapped);
                    this.selection_range = Some(snapped);
                    this.update_global_hover(snapped, cx);
                    cx.notify();
                }
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if event.pressed_button == Some(MouseButton::Right) {
                    if let Some(anchor) = this.anchor_range {
                        if let Some(current) = this.hit_test(event.position) {
                            let snapped = this.expand_to_word_boundaries(current);
                            
                            // Calculate the union of the anchor word and current word
                            let start = anchor.0.min(snapped.0);
                            let end = anchor.1.max(snapped.1);
                            let new_range = Some((start, end));
                            
                            if this.selection_range != new_range {
                                this.selection_range = new_range;
                                this.update_global_hover((start, end), cx);
                                cx.notify();
                            }
                        }
                    }
                }
            }))
            .on_mouse_up(MouseButton::Right, cx.listener(|this, _event: &MouseUpEvent, _, cx| {
                if let Some((start, end)) = this.selection_range {
                    if let Some(on_lookup) = &this.on_lookup {
                        if let Some(char_str) = this.text.get(start..end) {
                            (on_lookup)(char_str, cx);
                        }
                    }
                }
                this.selection_range = None;
                this.anchor_range = None;
                cx.remove_global::<GlobalHoverState>();
                cx.notify();
            }))
            .children(self.text.char_indices().map(|(idx, c)| {
                let is_selected = self.selection_range.map_or(false, |(start, end)| idx >= start && idx < end);
                let char_str = c.to_string();
                
                // Find UTF-8 character boundaries for this specific character
                let start = idx;
                let mut end = start + 1;
                while end <= self.text.len() && !self.text.is_char_boundary(end) {
                    end += 1;
                }

                let entity = entity.clone();
                div()
                    .id(ElementId::Name(SharedString::from(format!("char-{}-{}", text_hash, idx))))
                    .relative()
                    .child(
                        canvas(
                            move |_style, window, cx| {
                                window.request_layout(gpui::Style::default(), None, cx)
                            },
                            move |bounds, _layout_id, _window, cx| {
                                let _ = entity.update(cx, |this, _| {
                                    this.char_bounds.push((start, end, bounds));
                                });
                            }
                        )
                        .absolute()
                        .size_full()
                    )
                    .when(is_selected, |this| this.bg(highlight_bg).text_color(rgb(0xFFFFFF)).rounded_sm())
                    .when(!is_selected, |this| this.text_color(text_color))
                    .child(char_str)
            }))
    }
}
