use objc::declare::ClassDecl;
use objc::runtime::{Object, Sel, BOOL, YES, NO};
use objc::{class, msg_send, sel, sel_impl};
use std::sync::{OnceLock, Once};
use std::cell::UnsafeCell;
use std::ffi::{c_void, CString};
use std::os::raw::c_char;

#[link(name = "AVFoundation", kind = "framework")]
extern "C" {}

#[link(name = "CoreMedia", kind = "framework")]
extern "C" {}

#[link(name = "CoreVideo", kind = "framework")]
extern "C" {}

#[link(name = "Vision", kind = "framework")]
extern "C" {}

#[link(name = "System", kind = "framework")]
extern "C" {
    fn dispatch_queue_create(label: *const c_char, attr: *const c_void) -> *mut c_void;
}

static mut SESSION: Option<*mut Object> = None;
static mut BARCODE_CALLBACK: OnceLock<BarcodeCallback> = OnceLock::new();
static mut SCANNER_INIT: Once = Once::new();

struct BarcodeCallback(UnsafeCell<Option<Box<dyn Fn(String) + Send + Sync + 'static>>>);
unsafe impl Send for BarcodeCallback {}
unsafe impl Sync for BarcodeCallback {}

pub fn macos_get_session() -> Option<*mut Object> {
    unsafe { SESSION }
}

pub fn scan<F>(callback: F) -> Result<(), String>
where
    F: Fn(String) + Send + Sync + 'static,
{
    unsafe {
        let cell = BARCODE_CALLBACK.get_or_init(|| BarcodeCallback(UnsafeCell::new(None)));
        *cell.0.get() = Some(Box::new(callback));

        SCANNER_INIT.call_once(|| {
            std::thread::spawn(|| {
                init_scanner_internal();
            });
        });

        Ok(())
    }
}

unsafe fn init_scanner_internal() {
    log::info!("BarcodeScanner(macOS): Initializing session with Vision Engine...");
    let session: *mut Object = msg_send![class!(AVCaptureSession), alloc];
    let session: *mut Object = msg_send![session, init];
    unsafe { SESSION = Some(session); }

    let _: () = msg_send![session, beginConfiguration];
    
    let preset = make_nsstring("AVCaptureSessionPresetHigh");
    let _: () = msg_send![session, setSessionPreset: preset];

    let device: *mut Object = msg_send![class!(AVCaptureDevice), defaultDeviceWithMediaType: make_nsstring("vide")];
    if device.is_null() {
        log::error!("BarcodeScanner(macOS): No video device found");
        return;
    }

    let mut error: *mut Object = std::ptr::null_mut();
    let input: *mut Object = msg_send![class!(AVCaptureDeviceInput), deviceInputWithDevice:device error:&mut error];
    if input.is_null() {
        log::error!("BarcodeScanner(macOS): Failed to create device input");
        return;
    }

    if msg_send![session, canAddInput: input] {
        let _: () = msg_send![session, addInput: input];
    }

    // Use VideoDataOutput instead of MetadataOutput for Vision compatibility
    let output: *mut Object = msg_send![class!(AVCaptureVideoDataOutput), alloc];
    let output: *mut Object = msg_send![output, init];
    
    if msg_send![session, canAddOutput: output] {
        let _: () = msg_send![session, addOutput: output];
        
        let delegate = create_vision_delegate();
        let label = CString::new("com.gpui.vision.queue").unwrap();
        let queue = dispatch_queue_create(label.as_ptr(), std::ptr::null());
        let _: () = msg_send![output, setSampleBufferDelegate:delegate queue:queue];
        
        let _: () = msg_send![session, commitConfiguration];
        let _: () = msg_send![session, startRunning];
        
        log::info!("BarcodeScanner(macOS): VISION SCANNER ARMED AND ACTIVE");
    }
}

pub fn stop() -> Result<(), String> {
    unsafe {
        if let Some(session) = SESSION {
            let _: () = msg_send![session, stopRunning];
        }
        Ok(())
    }
}

unsafe fn make_nsstring(s: &str) -> *mut Object {
    let ns_string: *mut Object = msg_send![class!(NSString), alloc];
    let ns_string: *mut Object = msg_send![ns_string, initWithBytes:s.as_ptr() length:s.len() encoding:4u64];
    ns_string
}

fn create_vision_delegate() -> *mut Object {
    unsafe {
        let mut decl = ClassDecl::new("VisionBarcodeDelegate", class!(NSObject)).unwrap();
        decl.add_method(
            sel!(captureOutput:didOutputSampleBuffer:fromConnection:),
            did_output_sample_buffer as extern "C" fn(&Object, Sel, *mut Object, *mut Object, *mut Object),
        );
        let cls = decl.register();
        let delegate: *mut Object = msg_send![cls, alloc];
        let delegate: *mut Object = msg_send![delegate, init];
        delegate
    }
}

extern "C" {
    fn CMSampleBufferGetImageBuffer(sbuf: *mut Object) -> *mut Object;
}

extern "C" fn did_output_sample_buffer(
    _this: &Object,
    _sel: Sel,
    _output: *mut Object,
    sample_buffer: *mut Object,
    _connection: *mut Object,
) {
    unsafe {
        let pixel_buffer = CMSampleBufferGetImageBuffer(sample_buffer);
        if pixel_buffer.is_null() { return; }

        // 1. Create Barcode Request
        let request: *mut Object = msg_send![class!(VNDetectBarcodesRequest), alloc];
        let request: *mut Object = msg_send![request, init];
        
        // 2. Create Request Handler
        let handler: *mut Object = msg_send![class!(VNImageRequestHandler), alloc];
        let handler: *mut Object = msg_send![handler, initWithCVPixelBuffer:pixel_buffer options:std::ptr::null_mut::<Object>()];
        
        // 3. Perform Request
        let requests: *mut Object = msg_send![class!(NSArray), arrayWithObject: request];
        let mut error: *mut Object = std::ptr::null_mut::<Object>();
        let success: BOOL = msg_send![handler, performRequests:requests error:&mut error];
        
        if success == YES {
            let results: *mut Object = msg_send![request, results];
            if !results.is_null() {
                let count: usize = msg_send![results, count];
                
                for i in 0..count {
                    let result: *mut Object = msg_send![results, objectAtIndex: i];
                    let string_value: *mut Object = msg_send![result, payloadStringValue];
                    
                    if !string_value.is_null() {
                        let bytes: *const c_char = msg_send![string_value, UTF8String];
                        let barcode = std::ffi::CStr::from_ptr(bytes).to_string_lossy().into_owned();
                        
                        log::info!("BarcodeScanner(macOS): Found barcode via Vision! -> {}", barcode);
                        
                        if let Some(cell) = BARCODE_CALLBACK.get() {
                            if let Some(callback) = &*cell.0.get() {
                                callback(barcode);
                            }
                        }
                    }
                }
            }
        }
        
        // Cleanup
        let _: () = msg_send![request, release];
        let _: () = msg_send![handler, release];
    }
}
