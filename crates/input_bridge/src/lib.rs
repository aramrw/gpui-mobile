use std::sync::Mutex;
use once_cell::sync::Lazy;

pub static DISPATCHER: Lazy<Mutex<Option<Box<dyn Fn(&str) + Send + Sync>>>> = Lazy::new(|| Mutex::new(None));

pub fn register_dispatcher(f: Box<dyn Fn(&str) + Send + Sync>) {
    *DISPATCHER.lock().unwrap() = Some(f);
}

pub fn dispatch(text: &str) {
    if let Some(f) = DISPATCHER.lock().unwrap().as_ref() {
        f(text);
    }
}
