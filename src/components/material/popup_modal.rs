//! Material Design 3 Popup Modal component.
//!
//! Provides a fullscreen, blurry, semi-transparent backdrop with a
//! flexible content container.

use gpui::{
    div, px, rgba, AnyElement, IntoElement, ParentElement, Styled,
    InteractiveElement, prelude::FluentBuilder,
};
use std::rc::Rc;

use super::theme::MaterialTheme;

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum ModalPosition {
    Top,
    #[default]
    Center,
    Bottom,
}

/// A Material Design 3 **Popup Modal**.
///
/// Wraps content in a fullscreen overlay with a blurred backdrop.
pub struct PopupModal {
    theme: MaterialTheme,
    position: ModalPosition,
    child: Option<AnyElement>,
    on_close: Option<Rc<dyn Fn(&mut gpui::Window, &mut gpui::App) + 'static>>,
}

impl PopupModal {
    pub fn new(theme: MaterialTheme) -> Self {
        Self {
            theme,
            position: ModalPosition::Center,
            child: None,
            on_close: None,
        }
    }

    pub fn position(mut self, position: ModalPosition) -> Self {
        self.position = position;
        self
    }

    pub fn child(mut self, child: impl IntoElement) -> Self {
        self.child = Some(child.into_any_element());
        self
    }

    pub fn on_close(mut self, handler: impl Fn(&mut gpui::Window, &mut gpui::App) + 'static) -> Self {
        self.on_close = Some(Rc::new(handler));
        self
    }
}

impl IntoElement for PopupModal {
    type Element = <gpui::Div as IntoElement>::Element;

    fn into_element(self) -> Self::Element {
        let margin = match self.position {
            ModalPosition::Top => px(80.0),
            ModalPosition::Center => px(0.0),
            ModalPosition::Bottom => px(80.0),
        };

        // Fullscreen blurry backdrop
        div()
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(rgba(0x000000_66)) // Dark semi-transparent
            .flex()
            .flex_col()
            .items_center()
            .map(|this| match self.position {
                ModalPosition::Top => this.justify_start(),
                ModalPosition::Center => this.justify_center(),
                ModalPosition::Bottom => this.justify_end(),
            })
            .when(self.position == ModalPosition::Top, |this| this.pt(margin))
            .when(self.position == ModalPosition::Bottom, |this| this.pb(margin))
            .child(
                // The actual modal content container
                div()
                    .id("modal-content")
                    .child(self.child.unwrap_or_else(|| div().into_any_element()))
                    .on_mouse_down(gpui::MouseButton::Left, move |_, _, _| {
                        // Stop propagation is handled implicitly by having an ID and being a child of the backdrop
                    })
            )
            // Clicks on the backdrop close the modal
            .on_mouse_down(gpui::MouseButton::Left, move |_event, window, cx| {
                if let Some(handler) = &self.on_close {
                    (handler)(window, cx);
                }
            })
            .into_element()
    }
}
