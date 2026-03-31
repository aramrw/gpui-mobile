//! Material Design 3 Dropdown component.
//!
//! A button that opens a menu when clicked.

use gpui::{
    div, IntoElement, ParentElement, Styled, Stateful,
    InteractiveElement, prelude::FluentBuilder,
};

use super::theme::{color, MaterialTheme};
use super::menu::{Menu, MenuAnchor};

/// A Material Design 3 **Dropdown**.
///
/// A button trigger that reveals a menu of options.
pub struct Dropdown {
    theme: MaterialTheme,
    label: String,
    icon: Option<String>,
    menu_items: Vec<DropdownItem>,
    is_open: bool,
    on_toggle: Option<Box<dyn Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static>>,
}

pub struct DropdownItem {
    pub label: String,
    pub on_click: Box<dyn Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static>,
}

impl Dropdown {
    pub fn new(label: impl Into<String>, theme: MaterialTheme) -> Self {
        Self {
            theme,
            label: label.into(),
            icon: None,
            menu_items: Vec::new(),
            is_open: false,
            on_toggle: None,
        }
    }

    pub fn icon(mut self, icon: impl Into<String>) -> Self {
        self.icon = Some(icon.into());
        self
    }

    pub fn item(mut self, label: impl Into<String>, on_click: impl Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static) -> Self {
        self.menu_items.push(DropdownItem {
            label: label.into(),
            on_click: Box::new(on_click),
        });
        self
    }

    pub fn open(mut self, open: bool) -> Self {
        self.is_open = open;
        self
    }

    pub fn on_toggle(mut self, handler: impl Fn(&gpui::MouseDownEvent, &mut gpui::Window, &mut gpui::App) + 'static) -> Self {
        self.on_toggle = Some(Box::new(handler));
        self
    }
}

impl IntoElement for Dropdown {
    type Element = <Stateful<gpui::Div> as IntoElement>::Element;

    fn into_element(self) -> Self::Element {
        let t = self.theme;
        let bg = color(t.surface_container_low);
        let text_color = color(t.on_surface);

        let mut menu = Menu::new(t);
        for item in self.menu_items {
            menu = menu.item(item.label, "", item.on_click);
        }

        MenuAnchor::new(t)
            .open(self.is_open)
            .anchor(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_2()
                    .px_3()
                    .py_1p5()
                    .rounded_lg()
                    .bg(bg)
                    .border_1()
                    .border_color(color(t.outline_variant))
                    .child(
                        div()
                            .text_sm()
                            .text_color(text_color)
                            .child(self.label)
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(color(t.on_surface_variant))
                            .child(if self.is_open { "▴" } else { "▾" })
                    )
                    .when_some(self.on_toggle, |this, handler| {
                        this.on_mouse_down(gpui::MouseButton::Left, move |e, window, cx| {
                            handler(e, window, cx);
                        })
                    })
            )
            .menu(menu)
            .into_element()
    }
}
