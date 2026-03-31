//! Monokakido-style Zoom Header that drops down from the top.
//!
//! Shows the currently hovered text with surrounding context.

use gpui::{
    div, px, rgb, rgba, IntoElement, ParentElement, Render, Styled, Window,
    Context as ViewContext, InteractiveElement,
};

use crate::GlobalHoverState;
use super::theme::{MaterialTheme, color};

pub struct ZoomHeader {
    theme: MaterialTheme,
}

impl ZoomHeader {
    pub fn new(theme: MaterialTheme) -> Self {
        Self { theme }
    }

    pub fn set_theme(&mut self, theme: MaterialTheme, cx: &mut ViewContext<Self>) {
        self.theme = theme;
        cx.notify();
    }
}

impl Render for ZoomHeader {
    fn render(&mut self, _window: &mut Window, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let hover_state = cx.try_global::<GlobalHoverState>();
        
        // Only show if we have an active hover
        if let Some(state) = hover_state {
            let bg_color = if self.theme.is_dark {
                rgba(0x1C1B1F_FF) // surface
            } else {
                rgba(0xFFFBFE_FF) // surface
            };

            let text_color = color(self.theme.on_surface);
            let highlight_color = rgb(0x4285F4); // Monokakido blue

            div()
                .id("zoom-header")
                .absolute()
                .top_0()
                .left_0()
                .right_0()
                .h(px(108.0)) 
                .bg(bg_color)
                .overflow_hidden()
                .border_b_1()
                .border_color(color(self.theme.outline_variant))
                .flex()
                .flex_row()
                .flex_nowrap()
                .items_end() // Align content to the bottom of the bar
                .justify_center()
                .px_4()
                .pb_2() // Bottom padding for better spacing
                .child(
                    div()
                        .flex()
                        .flex_row()
                        .flex_nowrap()
                        .items_baseline() // USE BASELINE for consistent text height
                        .text_xl()
                        .font_weight(gpui::FontWeight::MEDIUM)
                        .child(
                            div()
                                .flex_none()
                                .text_color(text_color.opacity(0.6))
                                .child(state.context_before.clone())
                        )
                        .child(
                            div()
                                .flex_none()
                                .bg(highlight_color)
                                .text_color(rgb(0xFFFFFF))
                                .rounded_sm()
                                .px_1()
                                .child(state.text.clone())
                        )
                        .child(
                            div()
                                .flex_none()
                                .text_color(text_color.opacity(0.6))
                                .child(state.context_after.clone())
                        )
                )
        } else {
            div().id("zoom-header-hidden")
        }
    }
}
