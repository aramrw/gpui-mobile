#[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard"))]
use arboard::Clipboard;

pub fn set_text(text: &str) -> Result<(), String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard"))]
    {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        clipboard.set_text(text.to_string()).map_err(|e| e.to_string())
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard")))]
    {
        let _ = text;
        Err("Clipboard is not available on this platform or feature is disabled".into())
    }
}

pub fn get_text() -> Result<Option<String>, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard"))]
    {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        match clipboard.get_text() {
            Ok(text) => Ok(Some(text)),
            Err(_) => Ok(None),
        }
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard")))]
    {
        Err("Clipboard is not available on this platform or feature is disabled".into())
    }
}

pub fn has_text() -> Result<bool, String> {
    #[cfg(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard"))]
    {
        let mut clipboard = Clipboard::new().map_err(|e| e.to_string())?;
        match clipboard.get_text() {
            Ok(_) => Ok(true),
            Err(_) => Ok(false),
        }
    }
    #[cfg(not(all(not(any(target_os = "ios", target_os = "android")), feature = "arboard")))]
    {
        Err("Clipboard is not available on this platform or feature is disabled".into())
    }
}
