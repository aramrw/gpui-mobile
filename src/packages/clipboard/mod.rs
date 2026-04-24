//! Clipboard access for reading and writing text.
//!
//! Provides a cross-platform clipboard API backed by:
//! - Android: ClipboardManager via JNI (through a Java helper)
//! - iOS: UIPasteboard via Objective-C
//! - Desktop: arboard
//!
//! Feature-gated behind `clipboard`.

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "ios")]
mod ios;
#[cfg(not(any(target_os = "ios", target_os = "android")))]
mod desktop;

/// Copy text to the clipboard.
pub fn set_text(text: &str) -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::set_text(text)
    }
    #[cfg(target_os = "android")]
    {
        android::set_text(text)
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        desktop::set_text(text)
    }
}

/// Read text from the clipboard. Returns None if clipboard is empty or not text.
pub fn get_text() -> Result<Option<String>, String> {
    #[cfg(target_os = "ios")]
    {
        ios::get_text()
    }
    #[cfg(target_os = "android")]
    {
        android::get_text()
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        desktop::get_text()
    }
}

/// Check if the clipboard has text.
pub fn has_text() -> Result<bool, String> {
    #[cfg(target_os = "ios")]
    {
        ios::has_text()
    }
    #[cfg(target_os = "android")]
    {
        android::has_text()
    }
    #[cfg(not(any(target_os = "ios", target_os = "android")))]
    {
        desktop::has_text()
    }
}
