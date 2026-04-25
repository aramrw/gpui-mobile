//! macOS platform view implementation.

use crate::platform_view::{
    PlatformView, PlatformViewBounds, PlatformViewFactory, PlatformViewId, PlatformViewParams,
};
use gpui::PlatformWindow;
use std::sync::atomic::{AtomicBool, Ordering};
use objc::{class, msg_send, runtime::Object, sel, sel_impl};
use cocoa::base::{id, nil};
use cocoa::appkit::NSView;
use std::ffi::c_void;

pub struct MacPlatformView {
    id: PlatformViewId,
    view_type: String,
    native_view: std::sync::Mutex<id>,
    disposed: AtomicBool,
    inserted: AtomicBool,
    bounds: std::sync::Mutex<PlatformViewBounds>,
}

unsafe impl Send for MacPlatformView {}
unsafe impl Sync for MacPlatformView {}

impl MacPlatformView {
    pub fn new(view_type: &str, params: &PlatformViewParams) -> Result<Self, String> {
        let id = PlatformViewId::next();
        let native_view = Self::create_native_view(view_type, &params.bounds, params)?;

        Ok(Self {
            id,
            view_type: view_type.to_string(),
            native_view: std::sync::Mutex::new(native_view),
            disposed: AtomicBool::new(false),
            inserted: AtomicBool::new(false),
            bounds: std::sync::Mutex::new(params.bounds),
        })
    }

    fn create_native_view(
        view_type: &str,
        bounds: &PlatformViewBounds,
        params: &PlatformViewParams,
    ) -> Result<id, String> {
        unsafe {
            let frame = cocoa::foundation::NSRect::new(
                cocoa::foundation::NSPoint::new(bounds.x as f64, bounds.y as f64),
                cocoa::foundation::NSSize::new(bounds.width as f64, bounds.height as f64),
            );

            let view: id = match view_type {
                "camera_preview" => Self::create_camera_preview_view(frame, params)?,
                _ => Self::create_generic_view(frame)?,
            };

            if view.is_null() {
                return Err(format!("Failed to create NSView for type '{}'", view_type));
            }

            log::info!("MacPlatformView: created native NSView for type '{}'", view_type);
            Ok(view)
        }
    }

    unsafe fn create_generic_view(frame: cocoa::foundation::NSRect) -> Result<id, String> {
        let view: id = msg_send![class!(NSView), alloc];
        let view: id = msg_send![view, initWithFrame: frame];
        Ok(view)
    }

    unsafe fn create_camera_preview_view(
        frame: cocoa::foundation::NSRect,
        params: &PlatformViewParams,
    ) -> Result<id, String> {
        let view: id = msg_send![class!(NSView), alloc];
        let view: id = msg_send![view, initWithFrame: frame];
        
        let _: () = msg_send![view, setWantsLayer: true];

        if let Some(session_id_str) = params.creation_params.get("session_id") {
            if let Ok(session_id) = session_id_str.parse::<usize>() {
                // Try camera package session
                let mut session_ptr = crate::packages::camera::macos_get_session(session_id);
                
                // If not found, try barcode scanner session
                if session_ptr.is_none() {
                    if let Some(barcode_ptr) = crate::platform_info::Barcode::macos_get_session() {
                        if barcode_ptr as usize == session_id {
                            session_ptr = Some(barcode_ptr);
                        }
                    }
                }

                if let Some(session_ptr) = session_ptr {
                    log::info!("MacPlatformView: initializing preview with session {:p}", session_ptr);
                    let layer: id = msg_send![class!(AVCaptureVideoPreviewLayer), alloc];
                    let layer: id = msg_send![layer, initWithSession: session_ptr];
                    if !layer.is_null() {
                        let rect = core_graphics::geometry::CGRect::new(
                            &core_graphics::geometry::CGPoint::new(0.0, 0.0),
                            &core_graphics::geometry::CGSize::new(frame.size.width, frame.size.height),
                        );
                        let _: () = msg_send![layer, setFrame: rect];
                        let gravity = Self::make_nsstring("AVLayerVideoGravityResizeAspectFill");
                        let _: () = msg_send![layer, setVideoGravity: gravity];
                        let name = Self::make_nsstring("preview_layer");
                        let _: () = msg_send![layer, setName: name];
                        
                        let view_layer: id = msg_send![view, layer];
                        let _: () = msg_send![view_layer, addSublayer: layer];
                        log::info!("MacPlatformView: preview layer added to view");
                    } else {
                        log::error!("MacPlatformView: failed to create AVCaptureVideoPreviewLayer");
                    }
                } else {
                    log::warn!("MacPlatformView: could not find session for id {}", session_id);
                }
            }
        }

        Ok(view)
    }

    unsafe fn make_nsstring(s: &str) -> id {
        let ns_string: id = msg_send![class!(NSString), alloc];
        let ns_string: id = msg_send![ns_string, initWithBytes:s.as_ptr() length:s.len() encoding:4u64];
        ns_string
    }

    pub fn insert_into_window(&self) -> Result<(), String> {
        if self.inserted.swap(true, Ordering::Relaxed) {
            return Ok(());
        }

        let native_view = *self.native_view.lock().unwrap();
        let native_view_addr = native_view as usize;
        unsafe {
            if let Some(list) = gpui_macos::window::MAC_WINDOW_LIST.get() {
                let windows = list.0.lock();
                if let Some(&window_addr) = windows.last() {
                    let window_addr = window_addr;
                    log::info!("MacPlatformView: Inserting view, on main thread? {}", is_main_thread());
                    dispatch_on_main(move || {
                        let native_view = native_view_addr as id;
                        let window = &*(window_addr as *const gpui_macos::window::MacWindow);
                        let content_view: id = msg_send![window.platform_window(), contentView];
                        
                        log::info!("MacPlatformView: Performing addSubview:positioned:relativeTo: on main thread");
                        // Use NSWindowAbove (-1 is below, 1 is above)
                        let _: () = msg_send![content_view, addSubview: native_view positioned: 1 relativeTo: nil];
                        log::info!("MacPlatformView: inserted view into window hierarchy (Above)");
                    });
                    return Ok(());
                }
            }
        }
        self.inserted.store(false, Ordering::Relaxed);
        Err("No GPUI window available".to_string())
    }
}

fn is_main_thread() -> bool {
    unsafe {
        let is_main: objc::runtime::BOOL = msg_send![class!(NSThread), isMainThread];
        is_main == objc::runtime::YES
    }
}

extern "C" fn trampoline<F: FnOnce()>(context: *mut c_void) {
    let f = unsafe { Box::from_raw(context as *mut F) };
    f();
}

fn dispatch_on_main<F>(f: F)
where
    F: FnOnce() + Send + 'static,
{
    if is_main_thread() {
        f();
    } else {
        let context = Box::into_raw(Box::new(f));
        unsafe {
            gpui_macos::dispatch2::DispatchQueue::main().exec_async_f(context as *mut _, trampoline::<F>);
        }
    }
}

impl PlatformView for MacPlatformView {
    fn id(&self) -> PlatformViewId { self.id }
    fn view_type(&self) -> &str { &self.view_type }

    fn set_bounds(&self, bounds: PlatformViewBounds) {
        if self.disposed.load(Ordering::Relaxed) { return; }
        let _ = self.insert_into_window();
        
        *self.bounds.lock().unwrap() = bounds;
        let view = *self.native_view.lock().unwrap();
        let view_addr = view as usize;
        unsafe {
            // Need to flip Y for macOS
            if let Some(list) = gpui_macos::window::MAC_WINDOW_LIST.get() {
                let windows = list.0.lock();
                if let Some(&window_addr) = windows.last() {
                    let window = &*(window_addr as *const gpui_macos::window::MacWindow);
                    let content_height = window.content_size().height.as_f32();
                    let frame = cocoa::foundation::NSRect::new(
                        cocoa::foundation::NSPoint::new(bounds.x as f64, (content_height - bounds.y - bounds.height) as f64),
                        cocoa::foundation::NSSize::new(bounds.width as f64, bounds.height as f64),
                    );
                    dispatch_on_main(move || {
                        let view = view_addr as id;
                        let _: () = msg_send![view, setFrame: frame];
                        
                        unsafe {
                            let layer: id = msg_send![view, layer];
                            
                            let sublayers: id = msg_send![layer, sublayers];
                            let count: usize = msg_send![sublayers, count];
                            for i in 0..count {
                                let sublayer: id = msg_send![sublayers, objectAtIndex: i];
                                let name_obj: id = msg_send![sublayer, name];
                                if !name_obj.is_null() {
                                    let name_bytes: *const std::os::raw::c_char = msg_send![name_obj, UTF8String];
                                    let name = std::ffi::CStr::from_ptr(name_bytes).to_string_lossy();
                                    if name == "preview_layer" {
                                        let rect = core_graphics::geometry::CGRect::new(
                                            &core_graphics::geometry::CGPoint::new(0.0, 0.0),
                                            &core_graphics::geometry::CGSize::new(frame.size.width, frame.size.height),
                                        );
                                        let _: () = msg_send![sublayer, setFrame: rect];
                                    }
                                }
                            }
                        }
                    });
                }
            }
        }
    }

    fn set_visible(&self, visible: bool) {
        let view = *self.native_view.lock().unwrap();
        let view_addr = view as usize;
        dispatch_on_main(move || {
            let view = view_addr as id;
            unsafe { let _: () = msg_send![view, setHidden: if visible { 0 } else { 1 }]; }
        });
    }

    fn set_z_index(&self, _z: i32) {}

    fn dispose(&self) {
        if self.disposed.swap(true, Ordering::Relaxed) { return; }
        let view = *self.native_view.lock().unwrap();
        let view_addr = view as usize;
        dispatch_on_main(move || {
            let view = view_addr as id;
            unsafe { let _: () = msg_send![view, removeFromSuperview]; }
        });
    }

    fn is_disposed(&self) -> bool { self.disposed.load(Ordering::Relaxed) }
}

pub struct MacPlatformViewFactory {
    view_type: String,
}

impl MacPlatformViewFactory {
    pub fn new(view_type: &str) -> Self {
        Self { view_type: view_type.to_string() }
    }
}

impl PlatformViewFactory for MacPlatformViewFactory {
    fn create(&self, params: &PlatformViewParams) -> Result<Box<dyn PlatformView>, String> {
        let view = MacPlatformView::new(&self.view_type, params)?;
        Ok(Box::new(view))
    }
    fn view_type(&self) -> &str { &self.view_type }
}
