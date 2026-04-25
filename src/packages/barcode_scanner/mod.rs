//! Native Barcode Scanner.

#[cfg(target_os = "ios")]
mod ios;

#[cfg(target_os = "macos")]
mod macos;

#[cfg(target_os = "macos")]
pub fn macos_get_session() -> Option<*mut objc::runtime::Object> {
    macos::macos_get_session()
}

/// Start scanning for barcodes.
pub fn scan<F>(on_scan: F) -> Result<(), String>
where
    F: Fn(String) + Send + Sync + 'static,
{
    #[cfg(target_os = "ios")]
    {
        ios::scan(on_scan)
    }
    #[cfg(target_os = "macos")]
    {
        macos::scan(on_scan)
    }
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    {
        let _ = on_scan;
        Err("Barcode scanning is currently only supported on iOS and macOS".into())
    }
}

/// Stop the barcode scanner.
pub fn stop() -> Result<(), String> {
    #[cfg(target_os = "ios")]
    {
        ios::stop()
    }
    #[cfg(target_os = "macos")]
    {
        macos::stop()
    }
    #[cfg(not(any(target_os = "ios", target_os = "macos")))]
    {
        Ok(())
    }
}
