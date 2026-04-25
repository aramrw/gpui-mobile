#![cfg(target_os = "macos")]
//! macOS platform implementation for GPUI.
//!
//! macOS screens have a y axis that goes up from the bottom of the screen and
//! an origin at the bottom left of the main display.

mod dispatcher;
mod display;
mod display_link;
mod events;
mod keyboard;
mod metal_atlas;
pub mod metal_renderer;
mod open_type;
pub mod pasteboard;
mod platform;
pub mod window;
pub mod window_appearance;

pub use dispatch2;
pub use metal_renderer as renderer;

#[cfg(feature = "screen-capture")]
mod screen_capture;

#[cfg(feature = "font-kit")]
mod text_system;

use cocoa::{
    base::{id, nil},
    foundation::{NSAutoreleasePool, NSNotFound, NSString, NSUInteger},
};

use objc::{msg_send, sel, sel_impl};
use objc::runtime::{BOOL, NO, YES};
use std::{
    ffi::{CStr, c_char},
    ops::Range,
};

pub(crate) use dispatcher::*;
pub(crate) use display::*;
pub(crate) use display_link::*;
pub(crate) use keyboard::*;
pub(crate) use platform::*;
pub use window::*;

#[cfg(feature = "font-kit")]
pub(crate) use text_system::*;

pub use platform::MacPlatform;

pub trait BoolExt {
    fn to_objc(self) -> BOOL;
}

impl BoolExt for bool {
    fn to_objc(self) -> BOOL {
        if self { YES } else { NO }
    }
}

pub trait NSStringExt {
    fn to_str(&self) -> &str;
}

impl NSStringExt for id {
    fn to_str(&self) -> &str {
        unsafe {
            let cstr: *const c_char = msg_send![*self, UTF8String];
            if cstr.is_null() {
                ""
            } else {
                CStr::from_ptr(cstr).to_str().unwrap()
            }
        }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NSRange {
    pub location: NSUInteger,
    pub length: NSUInteger,
}

unsafe impl objc::Encode for NSRange {
    fn encode() -> objc::Encoding {
        unsafe { objc::Encoding::from_str("{_NSRange=QQ}") }
    }
}

impl NSRange {
    pub fn invalid() -> Self {
        Self {
            location: NSNotFound as NSUInteger,
            length: 0,
        }
    }

    pub fn is_invalid(&self) -> bool {
        self.location == NSNotFound as NSUInteger
    }

    pub fn to_range(&self) -> Option<Range<usize>> {
        if self.is_invalid() {
            None
        } else {
            Some(self.location as usize..self.location as usize + self.length as usize)
        }
    }
}

impl From<Range<usize>> for NSRange {
    fn from(range: Range<usize>) -> Self {
        Self {
            location: range.start as NSUInteger,
            length: range.len() as NSUInteger,
        }
    }
}

impl From<NSRange> for Range<usize> {
    fn from(range: NSRange) -> Self {
        range.location as usize..range.location as usize + range.length as usize
    }
}

/// Allow NSString::alloc use here because it sets autorelease
#[allow(clippy::disallowed_methods)]
pub unsafe fn ns_string(string: &str) -> id {
    unsafe { NSString::alloc(nil).init_str(string).autorelease() }
}
