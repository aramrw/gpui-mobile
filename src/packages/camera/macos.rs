use std::sync::{Mutex, OnceLock};
use std::collections::HashMap;
use objc::{class, msg_send, sel, sel_impl, runtime::Object};
use crate::packages::camera::{CameraDescription, CameraLensDirection, ResolutionPreset, CameraHandle, FlashMode, FocusMode, ExposureMode, CapturedImage, RecordedVideo};

struct CameraSession {
    session: *mut Object,
}
unsafe impl Send for CameraSession {}
unsafe impl Sync for CameraSession {}

static SESSIONS: OnceLock<Mutex<HashMap<usize, CameraSession>>> = OnceLock::new();

fn get_sessions() -> &'static Mutex<HashMap<usize, CameraSession>> {
    SESSIONS.get_or_init(|| Mutex::new(HashMap::new()))
}

pub fn available_cameras() -> Result<Vec<CameraDescription>, String> {
    log::info!("Camera(macOS): available_cameras() called");
    unsafe {
        let types: *mut Object = msg_send![class!(NSArray), arrayWithObject: make_nsstring("AVCaptureDeviceTypeBuiltInWideAngleCamera")];
        let discovery_session: *mut Object = msg_send![class!(AVCaptureDeviceDiscoverySession), 
            discoverySessionWithDeviceTypes:types 
            mediaType:make_nsstring("vide") 
            position:0isize];
        
        let devices: *mut Object = msg_send![discovery_session, devices];
        let count: usize = msg_send![devices, count];
        
        let mut result = Vec::new();
        for i in 0..count {
            let device: *mut Object = msg_send![devices, objectAtIndex: i];
            let name_obj: *mut Object = msg_send![device, localizedName];
            let name: *const std::os::raw::c_char = msg_send![name_obj, UTF8String];
            let name_str = std::ffi::CStr::from_ptr(name).to_string_lossy().into_owned();
            
            result.push(CameraDescription {
                name: name_str,
                lens_direction: CameraLensDirection::Back, // macOS usually just has one
                sensor_orientation: 0,
            });
        }
        Ok(result)
    }
}

pub fn create_camera(
    camera: &CameraDescription,
    _resolution: ResolutionPreset,
    _enable_audio: bool,
) -> Result<usize, String> {
    unsafe {
        let session: *mut Object = msg_send![class!(AVCaptureSession), alloc];
        let session: *mut Object = msg_send![session, init];
        
        let device: *mut Object = msg_send![class!(AVCaptureDevice), defaultDeviceWithMediaType: make_nsstring("vide")];
        if device.is_null() {
            return Err("No camera found".into());
        }

        let input: *mut Object = msg_send![class!(AVCaptureDeviceInput), deviceInputWithDevice:device error:std::ptr::null_mut::<*mut Object>()];
        if msg_send![session, canAddInput: input] {
            let _: () = msg_send![session, addInput: input];
        }

        let _: () = msg_send![session, startRunning];
        
        let id = session as usize;
        get_sessions().lock().unwrap().insert(id, CameraSession { session });
        Ok(id)
    }
}

pub fn stop_preview_session(_handle: &CameraHandle) -> Result<(), String> {
    Ok(())
}

pub fn take_picture(_handle: &CameraHandle) -> Result<CapturedImage, String> {
    Err("Not implemented".into())
}

pub fn start_video_recording(_handle: &CameraHandle) -> Result<(), String> {
    Err("Not implemented".into())
}

pub fn stop_video_recording(_handle: &CameraHandle) -> Result<RecordedVideo, String> {
    Err("Not implemented".into())
}

pub fn set_flash_mode(_handle: &CameraHandle, _mode: FlashMode) -> Result<(), String> { Ok(()) }
pub fn set_focus_mode(_handle: &CameraHandle, _mode: FocusMode) -> Result<(), String> { Ok(()) }
pub fn set_exposure_mode(_handle: &CameraHandle, _mode: ExposureMode) -> Result<(), String> { Ok(()) }
pub fn get_min_zoom(_handle: &CameraHandle) -> Result<f64, String> { Ok(1.0) }
pub fn get_max_zoom(_handle: &CameraHandle) -> Result<f64, String> { Ok(1.0) }
pub fn set_zoom(_handle: &CameraHandle, _zoom: f64) -> Result<(), String> { Ok(()) }
pub fn set_camera(_handle: &CameraHandle, _camera: &CameraDescription) -> Result<(), String> { Ok(()) }

pub fn dispose(handle: CameraHandle) -> Result<(), String> {
    if let Some(session) = get_sessions().lock().unwrap().remove(&handle.id) {
        unsafe {
            let _: () = msg_send![session.session, stopRunning];
            let _: () = msg_send![session.session, release];
        }
    }
    Ok(())
}

pub fn macos_get_session(id: usize) -> Option<*mut Object> {
    get_sessions().lock().unwrap().get(&id).map(|s| s.session)
}

fn make_nsstring(s: &str) -> *mut Object {
    unsafe {
        let ns_string: *mut Object = msg_send![class!(NSString), alloc];
        let ns_string: *mut Object = msg_send![ns_string, initWithBytes:s.as_ptr() length:s.len() encoding:4u64];
        ns_string
    }
}
