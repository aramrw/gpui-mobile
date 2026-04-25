#![allow(warnings)]
//! Binary entry point for the cross-platform GPUI example app.
//!
//! On **iOS** this provides the standard `fn main()` entry point which
//! delegates to [`gpui_mobile_example::ios_main`].
//!
//! On **Android** the `android-activity` crate invokes `android_main` directly
//! from the cdylib defined in `lib.rs` — so `main` is never called.  We still
//! provide a stub so that `cargo check` (without a target triple) succeeds.
//!
//! ## Building
//!
//! ```text
//! # iOS simulator
//! cargo build --target aarch64-apple-ios-sim -p gpui-mobile-example --features font-kit
//!
//! # iOS device
//! cargo build --target aarch64-apple-ios -p gpui-mobile-example --features font-kit
//!
//! # Android (uses lib.rs cdylib, not this binary)
//! cargo ndk -t arm64-v8a build -p gpui-mobile-example
//! ```

pub mod gpui_platform;

#[cfg(target_os = "ios")]
fn main() {
    gpui_mobile_example::ios_main();
}

#[cfg(target_os = "android")]
fn main() {
    // On Android the real entry point is `android_main` in lib.rs, which is
    // called by the `android-activity` crate from the cdylib.  This binary
    // target is unused but must compile.
    eprintln!("This binary is not used on Android. The app enters via android_main() in lib.rs.");
}

#[cfg(not(any(target_os = "ios", target_os = "android")))]
fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    use gpui::{App, Application};
    use gpui_mobile_example::open_main_window;
    eprintln!(
        "This example is designed for iOS and Android. \
         Please build with --target aarch64-apple-ios-sim (iOS) or via cargo-ndk (Android)."
    );
    let current_platform = gpui_platform::current_platform(false);

    Application::with_platform(current_platform).run(|cx: &mut App| {
        open_main_window(cx);
    });
}
