use std::sync::mpsc::Sender;

use ohos_native_window_sys::{OHNativeWindow, OH_NativeWindow_GetSurfaceId};
use ohos_xcomponent_binding::{NativeXComponent, WindowRaw, XComponentRaw};

use crate::{CameraError, CameraResult, CameraSize, CameraSurface};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CameraXComponentEvent {
    Surface(CameraSurface),
    SurfaceLost,
}

/// Camera-specific surface routing built on `ohos-xcomponent-binding`.
pub struct CameraXComponentAttachment {
    _component: NativeXComponent,
}

impl CameraXComponentAttachment {
    pub fn attach(
        component: NativeXComponent,
        sender: Sender<CameraXComponentEvent>,
    ) -> CameraResult<Self> {
        let created = sender.clone();
        component.on_surface_created(move |component, window| {
            Self::send_surface(component, window, &created);
            Ok(())
        });
        let changed = sender.clone();
        component.on_surface_changed(move |component, window| {
            Self::send_surface(component, window, &changed);
            Ok(())
        });
        component.on_surface_destroyed(move |_component, _window| {
            let _ = sender.send(CameraXComponentEvent::SurfaceLost);
            Ok(())
        });
        component.register_callback().map_err(|error| {
            CameraError::surface("NativeXComponent::register_callback", error.to_string())
        })?;
        Ok(Self {
            _component: component,
        })
    }

    fn send_surface(
        component: XComponentRaw,
        window: WindowRaw,
        sender: &Sender<CameraXComponentEvent>,
    ) {
        if component.0.is_null() || window.0.is_null() {
            return;
        }
        let mut surface_id = 0_u64;
        // SAFETY: XComponent supplies the live matching native window for the
        // duration of this surface callback.
        if unsafe {
            OH_NativeWindow_GetSurfaceId(window.0.cast::<OHNativeWindow>(), &mut surface_id)
        } != 0
            || surface_id == 0
        {
            return;
        }
        let Ok(size) = component.size(window) else {
            return;
        };
        let _ = sender.send(CameraXComponentEvent::Surface(CameraSurface {
            id: surface_id,
            size: CameraSize::new(
                u32::try_from(size.width).unwrap_or(u32::MAX),
                u32::try_from(size.height).unwrap_or(u32::MAX),
            ),
        }));
    }
}
