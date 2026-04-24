use std::sync::atomic::{AtomicBool, Ordering};
use std::cell::RefCell;

pub type TextInputCallbackFn = Box<dyn FnMut(&str)>;

thread_local! {
    pub static TEXT_INPUT_CALLBACK: RefCell<Option<TextInputCallbackFn>> = RefCell::new(None);
}

pub static TEXT_INPUT_DIRTY: AtomicBool = AtomicBool::new(false);

pub fn set_text_input_callback(callback: Option<TextInputCallbackFn>) {
    TEXT_INPUT_CALLBACK.with(|cb| {
        *cb.borrow_mut() = callback;
    });
}

pub fn dispatch_text_input(text: &str) -> bool {
    TEXT_INPUT_CALLBACK.with(|cb| {
        if let Some(callback) = cb.borrow_mut().as_mut() {
            log::info!("dispatch_text_input: found callback. Text to process: '{}'", text);
            callback(text);
            TEXT_INPUT_DIRTY.store(true, Ordering::Release);
            true
        } else {
            log::info!("dispatch_text_input: NO callback found on thread: {:?}", std::thread::current().id());
            false
        }
    })
}

pub fn swap_dirty() -> bool {
    TEXT_INPUT_DIRTY.swap(false, Ordering::AcqRel)
}
