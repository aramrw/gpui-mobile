use std::sync::OnceLock;
use std::cell::UnsafeCell;

struct BarcodeCallbackCell(UnsafeCell<Option<Box<dyn Fn(String) + Send + Sync + 'static>>>);
unsafe impl Sync for BarcodeCallbackCell {}

static BARCODE_CALLBACK: OnceLock<BarcodeCallbackCell> = OnceLock::new();

pub fn scan<F>(on_scan: F) -> Result<(), String>
where
    F: Fn(String) + Send + Sync + 'static,
{
    let cell = BARCODE_CALLBACK.get_or_init(|| BarcodeCallbackCell(UnsafeCell::new(None)));
    unsafe {
        *cell.0.get() = Some(Box::new(on_scan));
    }
    
    // Trigger native iOS barcode scanning setup
    // This would typically involve starting an AVCaptureSession 
    // with a MetadataOutput.
    log::info!("iOS Barcode Scanner: Started (Native implementation placeholder)");
    
    Ok(())
}

pub fn stop() -> Result<(), String> {
    if let Some(cell) = BARCODE_CALLBACK.get() {
        unsafe {
            *cell.0.get() = None;
        }
    }
    Ok(())
}

/// FFI entry point called from Objective-C when a barcode is detected.
#[unsafe(no_mangle)]
pub extern "C" fn gpui_ios_on_barcode_scanned(barcode_ptr: *mut objc::runtime::Object) {
    if barcode_ptr.is_null() { return; }
    
    unsafe {
        let ns_string = barcode_ptr;
        let bytes: *const std::os::raw::c_char = objc::msg_send![ns_string, UTF8String];
        let barcode = std::ffi::CStr::from_ptr(bytes).to_string_lossy().into_owned();
        
        if let Some(cell) = BARCODE_CALLBACK.get() {
            if let Some(callback) = &*cell.0.get() {
                callback(barcode);
            }
        }
    }
}
