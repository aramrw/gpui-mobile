//! Material Design 3 Selectable Text component (View version).
//!
//! Provides text that can be selected with a Monokakido-style long-press.

use gpui::{
    div, px, rgb, IntoElement, MouseButton, ParentElement, Render, SharedString, Styled, Window, App,
    InteractiveElement, Context as ViewContext, Pixels, Point, Bounds, ElementId, canvas,
    prelude::FluentBuilder, MouseDownEvent, MouseMoveEvent, MouseUpEvent,
};

use super::theme::{color, MaterialTheme};

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
                let hit = this.hit_test(event.position);
                this.anchor_range = hit;
                this.selection_range = hit;
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, _, cx| {
                if event.pressed_button == Some(MouseButton::Right) {
                    if let Some(anchor) = this.anchor_range {
                        if let Some(current) = this.hit_test(event.position) {
                            // Calculate the union of the anchor and current character
                            let start = anchor.0.min(current.0);
                            let end = anchor.1.max(current.1);
                            let new_range = Some((start, end));
                            
                            if this.selection_range != new_range {
                                this.selection_range = new_range;
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
