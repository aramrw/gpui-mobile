//! Material Design 3 Popup Modal component.
//!
//! Provides a fullscreen, blurry, semi-transparent backdrop with a
//! flexible content container.

use gpui::{
    div, px, rgba, AnyElement, IntoElement, ParentElement, Styled,
    InteractiveElement, prelude::*, MouseButton, Stateful,
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
    #[allow(dead_code)]
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
    type Element = <Stateful<gpui::Div> as IntoElement>::Element;

    fn into_element(self) -> Self::Element {
        let margin = match self.position {
            ModalPosition::Top => px(80.0),
            ModalPosition::Center => px(0.0),
            ModalPosition::Bottom => px(80.0),
        };

        // Root container for the entire modal system
        div()
            .id("popup-modal-root")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(
                // BACKDROP LAYER: Sibling behind the content
                div()
                    .id("modal-backdrop")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .bg(rgba(0x000000_66))
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        // Clicking the backdrop hides keyboard and closes modal
                        crate::hide_keyboard();
                        if let Some(handler) = &self.on_close {
                            (handler)(window, cx);
                        }
                    })
            )
            .child(
                // CONTENT LAYER: Sibling in front of the backdrop
                div()
                    .id("modal-content-layer")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .flex()
                    .flex_col()
                    .items_center()
                    .map(|this: Stateful<gpui::Div>| match self.position {
                        ModalPosition::Top => this.justify_start(),
                        ModalPosition::Center => this.justify_center(),
                        ModalPosition::Bottom => this.justify_end(),
                    })
                    .when(self.position == ModalPosition::Top, |this: Stateful<gpui::Div>| this.pt(margin))
                    .when(self.position == ModalPosition::Bottom, |this: Stateful<gpui::Div>| this.pb(margin))
                    .child(
                        // The actual content box
                        div()
                            .id("modal-content-box")
                            .on_mouse_down(MouseButton::Left, |_, _, _| {
                                // CONSUME EVENT: Stop propagation to backdrop
                            })
                            .child(self.child.unwrap_or_else(|| div().into_any_element()))
                    )
            )
            .into_element()
    }
}
