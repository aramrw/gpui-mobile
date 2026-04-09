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

        let on_close = self.on_close.clone();

        // Root container for the entire modal system
        div()
            .id("popup-modal-root")
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .child(
                // 1. BACKDROP LAYER: Sibling behind the content.
                // This covers the WHOLE screen and handles closing.
                div()
                    .id("modal-backdrop")
                    .absolute()
                    .top_0()
                    .left_0()
                    .size_full()
                    .bg(rgba(0x000000_66))
                    .on_mouse_down(MouseButton::Left, move |_, window, cx| {
                        log::info!("PopupModal: Backdrop mouse_down");
                        crate::hide_keyboard();
                        if let Some(handler) = &on_close {
                            (handler)(window, cx);
                        }
                    })
            )
            .child(
                // 2. CONTENT LAYER: Sibling in front of the backdrop.
                // This layer is NOT interactive itself (doesn't have a handler),
                // so it doesn't catch clicks that should go to the backdrop.
                // But it positions the content box.
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
                        // The actual content box: Since its parent (content-layer) 
                        // doesn't have an on_mouse_down, clicks here won't bubble 
                        // to any close handler in the content-layer.
                        // And since it's a SIBLING of the backdrop, clicks here 
                        // won't bubble to the backdrop either.
                        div()
                            .id("modal-content-box")
                            .on_mouse_down(MouseButton::Left, move |_, _, _| {
                                log::info!("PopupModal: Content box mouse_down (stopping propagation)");
                            })
                            .child(self.child.unwrap_or_else(|| div().into_any_element()))
                    )
            )
            .into_element()
    }
}
