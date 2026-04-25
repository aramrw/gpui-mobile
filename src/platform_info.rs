use std::path::PathBuf;
use crate::packages::{path_provider, device_info, vibration, barcode_scanner};

/// Standard directory paths for the current platform.
pub struct Paths;

impl Paths {
    /// The directory where the app can store persistent data (e.g., SQLite DBs).
    /// On iOS, this is the Documents directory.
    pub fn data() -> Result<PathBuf, String> {
        path_provider::documents_directory()
    }

    /// The directory where the app can store temporary cache files.
    pub fn cache() -> Result<PathBuf, String> {
        path_provider::cache_directory()
    }

    /// The directory for application support files.
    pub fn support() -> Result<PathBuf, String> {
        path_provider::support_directory()
    }
}

/// Information about the current device.
pub fn device() -> Result<device_info::DeviceInfo, String> {
    device_info::get_device_info()
}

/// Haptic feedback and vibration.
pub struct Haptics;

impl Haptics {
    /// Trigger a specific haptic feedback pattern.
    pub fn feedback(feedback: vibration::HapticFeedback) -> Result<(), String> {
        vibration::haptic_feedback(feedback)
    }

    /// Vibrate the device for a given duration.
    pub fn vibrate(duration_ms: u32) -> Result<(), String> {
        vibration::vibrate(duration_ms)
    }
}

/// Native barcode scanner.
pub struct Barcode;

impl Barcode {
    /// Get the raw AVCaptureSession pointer (macOS only).
    #[cfg(target_os = "macos")]
    pub fn macos_get_session() -> Option<*mut objc::runtime::Object> {
        barcode_scanner::macos_get_session()
    }

    /// Start scanning for barcodes.
    pub fn scan<F>(on_scan: F) -> Result<(), String>
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        barcode_scanner::scan(on_scan)
    }

    /// Stop the barcode scanner.
    pub fn stop() -> Result<(), String> {
        barcode_scanner::stop()
    }
}

/// Returns whether the app is running on a mobile platform (iOS or Android).
pub fn is_mobile() -> bool {
    cfg!(any(target_os = "ios", target_os = "android"))
}
