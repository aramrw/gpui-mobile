//! GPUI element for embedding native platform views.

use crate::platform_view::{PlatformViewBounds, PlatformViewHandle};
use gpui::{div, Pixels, Styled, ParentElement, canvas, Bounds, IntoElement, AnyElement};
use std::sync::Arc;

/// Create a GPUI element that hosts a native platform view.
pub fn platform_view_element(handle: Arc<PlatformViewHandle>) -> impl IntoElement {
    canvas(
        move |bounds, _window, _cx| {
            // Prepaint logic
            bounds
        },
        move |_bounds, prepaint_bounds, _window, _cx| {
            let logical_bounds = PlatformViewBounds {
                x: prepaint_bounds.origin.x.as_f32(),
                y: prepaint_bounds.origin.y.as_f32(),
                width: prepaint_bounds.size.width.as_f32(),
                height: prepaint_bounds.size.height.as_f32(),
            };
            
            log::info!("PlatformViewElement: painting at {:?}", logical_bounds);
            handle.set_bounds(logical_bounds);
            handle.set_visible(true);
        },
    )
    .size_full()
}

/// A higher-level wrapper that creates the platform view from the registry
/// and manages its lifecycle.
pub struct ManagedPlatformView {
    handle: Option<Arc<PlatformViewHandle>>,
    view_type: String,
    creation_params: std::collections::HashMap<String, String>,
}

impl ManagedPlatformView {
    pub fn new(view_type: impl Into<String>) -> Self {
        Self {
            handle: None,
            view_type: view_type.into(),
            creation_params: std::collections::HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.creation_params.insert(key.into(), value.into());
        self
    }

    pub fn ensure_created(&mut self) -> Result<Arc<PlatformViewHandle>, String> {
        if let Some(handle) = &self.handle {
            return Ok(handle.clone());
        }

        let registry = crate::platform_view::PlatformViewRegistry::global();
        let params = crate::platform_view::PlatformViewParams {
            bounds: PlatformViewBounds::default(),
            creation_params: self.creation_params.clone(),
        };

        let handle = Arc::new(registry.create_view(&self.view_type, params)?);
        self.handle = Some(handle.clone());
        Ok(handle)
    }

    pub fn handle(&self) -> Option<&Arc<PlatformViewHandle>> {
        self.handle.as_ref()
    }

    pub fn render(&mut self) -> AnyElement {
        match self.ensure_created() {
            Ok(handle) => platform_view_element(handle).into_any_element(),
            Err(e) => {
                log::error!("Failed to create platform view '{}': {}", self.view_type, e);
                div().into_any_element()
            }
        }
    }
}

impl Drop for ManagedPlatformView {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            handle.dispose();
        }
    }
}
