//! Material Design 3 Selectable Text component (View version).
//!
//! Provides text that can be selected with a Monokakido-style long-press.

use gpui::{
    div, px, rgb, IntoElement, MouseButton, ParentElement, Render, SharedString, Styled, Window, App,
    InteractiveElement, Context as ViewContext, Font, FontWeight, FontStyle, TextStyle, TextRun,
    MouseDownEvent, MouseMoveEvent, MouseUpEvent, Pixels, Point, Bounds, ElementId, canvas,
};

use super::theme::{color, MaterialTheme};

pub struct SelectableTextView {
    text: SharedString,
    theme: MaterialTheme,
    on_lookup: Option<std::rc::Rc<dyn Fn(&str, &mut App)>>,
    selection_index: Option<(usize, usize)>, // (start_byte, end_byte)
    bounds: Bounds<Pixels>,
}

impl SelectableTextView {
    pub fn new(text: impl Into<SharedString>, theme: MaterialTheme, _cx: &mut ViewContext<Self>) -> Self {
        Self {
            text: text.into(),
            theme,
            on_lookup: None,
            selection_index: None,
            bounds: Bounds::default(),
        }
    }

    pub fn on_lookup(&mut self, handler: impl Fn(&str, &mut App) + 'static) {
        self.on_lookup = Some(std::rc::Rc::new(handler));
    }

    pub fn index_for_position(&self, position: Point<Pixels>, window: &mut Window, _cx: &App) -> Option<(usize, usize)> {
        if self.bounds.size.width == px(0.0) || self.text.is_empty() {
            return None;
        }

        let local_x = position.x - self.bounds.origin.x;
        
        let text_style = TextStyle {
            color: color(self.theme.on_surface),
            font_size: px(18.0).into(),
            ..Default::default()
        };
        
        let font = Font {
            family: text_style.font_family.clone(),
            weight: FontWeight::NORMAL,
            style: FontStyle::Normal,
            features: Default::default(),
            fallbacks: Default::default(),
        };

        let run = TextRun {
            len: self.text.len(),
            font,
            color: text_style.color,
            background_color: None,
            underline: None,
            strikethrough: None,
        };

        let lines = window.text_system().shape_line(
            self.text.clone(),
            px(18.0),
            &[run],
            None,
        );
        
        let byte_offset = lines.index_for_x(local_x)?;
        
        // Find UTF-8 character boundaries
        let mut start = byte_offset;
        while start > 0 && !self.text.is_char_boundary(start) {
            start -= 1;
        }
        
        let mut end = start + 1;
        while end <= self.text.len() && !self.text.is_char_boundary(end) {
            end += 1;
        }

        Some((start, end))
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

        div()
            .id(ElementId::Name(SharedString::from(format!("selectable-root-{}", text_hash))))
            .flex()
            .flex_row()
            .flex_wrap()
            .items_center()
            .w_full()
            .min_h(px(24.0)) 
            .child(
                canvas(
                    move |_style, window, cx| {
                        window.request_layout(gpui::Style::default(), None, cx)
                    },
                    move |bounds, _layout_id, _window, cx| {
                        let _ = entity.update(cx, |this, _| {
                            if this.bounds != bounds {
                                this.bounds = bounds;
                            }
                        });
                    }
                )
                .absolute()
                .size_full()
            )
            .on_mouse_down(MouseButton::Right, cx.listener(|this, event: &MouseDownEvent, window, cx| {
                this.selection_index = this.index_for_position(event.position, window, cx);
                cx.notify();
            }))
            .on_mouse_move(cx.listener(|this, event: &MouseMoveEvent, window, cx| {
                if event.pressed_button == Some(MouseButton::Right) && this.selection_index.is_some() {
                    this.selection_index = this.index_for_position(event.position, window, cx);
                    cx.notify();
                }
            }))
            .on_mouse_up(MouseButton::Right, cx.listener(|this, _event: &MouseUpEvent, _window, cx| {
                if let Some((start, end)) = this.selection_index {
                    if let Some(on_lookup) = &this.on_lookup {
                        if let Some(char_str) = this.text.get(start..end) {
                            (on_lookup)(char_str, cx);
                        }
                    }
                }
                this.selection_index = None;
                cx.notify();
            }))
            .children(if let Some((start, end)) = self.selection_index {
                let before = &self.text[..start];
                let selected = &self.text[start..end];
                let after = &self.text[end..];

                let mut children = Vec::new();
                if !before.is_empty() {
                    children.push(div().text_color(text_color).child(before.to_string()).into_any_element());
                }
                children.push(
                    div()
                        .bg(highlight_bg)
                        .text_color(rgb(0xFFFFFF))
                        .rounded_sm()
                        .px(px(1.0))
                        .child(selected.to_string())
                        .into_any_element()
                );
                if !after.is_empty() {
                    children.push(div().text_color(text_color).child(after.to_string()).into_any_element());
                }
                children
            } else {
                vec![div().text_color(text_color).child(self.text.clone()).into_any_element()]
            })
    }
}
