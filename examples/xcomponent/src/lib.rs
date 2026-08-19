#![allow(clippy::all)]
#![allow(dead_code)]

use std::{
    num::NonZeroU32,
    ptr::NonNull,
    sync::{LazyLock, Mutex},
};

use glutin::{
    config::ConfigTemplateBuilder,
    context::{ContextApi, ContextAttributesBuilder, PossiblyCurrentContext},
    display::{Display, DisplayApiPreference, GetGlDisplay},
    prelude::{GlDisplay, NotCurrentGlContext},
    surface::{GlSurface, Surface, SurfaceAttributesBuilder, WindowSurface},
};
use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, Result};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::XComponent;
use raw_window_handle::{
    OhosDisplayHandle, OhosNdkWindowHandle, RawDisplayHandle, RawWindowHandle,
};

static GL_CTX: LazyLock<Mutex<Option<Render>>> = LazyLock::new(|| Mutex::new(None));

/// Raw handle of the live XComponent so napi functions can reach it later
/// (frame-rate control). The framework keeps the native object alive for the
/// component's lifetime; the pointer is only used on the UI thread where the
/// napi entry points run.
#[derive(Clone, Copy)]
struct RawHandle(ohos_xcomponent_binding::XComponentRaw);

// Safety: the wrapped pointer is only dereferenced on the thread that
// received it and the native component outlives the module.
unsafe impl Send for RawHandle {}

static XC_RAW: LazyLock<Mutex<Option<RawHandle>>> = LazyLock::new(|| Mutex::new(None));

struct Render {
    display: Display,
    pub ctx: PossiblyCurrentContext,
    pub surface: Surface<WindowSurface>,
}

unsafe impl Send for Render {}
unsafe impl Sync for Render {}

#[napi(module_exports)]
pub fn init(exports: Object, env: Env) -> Result<()> {
    let xcomponent = XComponent::init(env, exports)?;
    *XC_RAW.lock().unwrap() = Some(RawHandle(ohos_xcomponent_binding::XComponentRaw(
        xcomponent.raw(),
    )));

    xcomponent.on_surface_created(|xcomponent, win| {
        hilog_info!("xcomponent_create");
        let size = xcomponent.size(win)?;

        let raw_handle =
            RawWindowHandle::OhosNdk(OhosNdkWindowHandle::new(NonNull::new(win.0).unwrap()));

        let handle = RawDisplayHandle::Ohos(OhosDisplayHandle::new());

        let display = unsafe { glutin::display::Display::new(handle, DisplayApiPreference::Egl) }
            .map_err(|e| Error::from_reason(e.to_string()))?;

        let config = unsafe {
            display
                .find_configs(ConfigTemplateBuilder::new().build())
                .unwrap()
                .next()
                .unwrap()
        };

        let ctx_attr = ContextAttributesBuilder::new()
            .with_context_api(ContextApi::Gles(None))
            .build(Some(raw_handle));

        let ctx = unsafe { display.create_context(&config, &ctx_attr).unwrap() };

        let surface_attr = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            raw_handle,
            NonZeroU32::new(size.width as u32).unwrap(),
            NonZeroU32::new(size.height as u32).unwrap(),
        );

        let surface = unsafe {
            display
                .create_window_surface(&config, &surface_attr)
                .map_err(|e| Error::from_reason(e.to_string()))?
        };

        let ctx: glutin::context::PossiblyCurrentContext = ctx
            .make_current(&surface)
            .map_err(|e| Error::from_reason(e.to_string()))?;

        gl::load_with(|symbol| {
            let symbol = std::ffi::CString::new(symbol).unwrap();
            let gl_display = ctx.display();
            gl_display.get_proc_address(symbol.as_c_str())
        });

        let gl_display = ctx.display();

        let render = Render {
            ctx,
            surface,
            display: gl_display,
        };

        let mut gl_ctx_guard = GL_CTX.lock().unwrap();
        *gl_ctx_guard = Some(render);

        Ok(())
    });

    xcomponent.on_surface_changed(|xcomponent, win| {
        hilog_info!("xcomponent_changed");
        let size = xcomponent.size(win)?;
        let offset = xcomponent.offset(win)?;
        hilog_info!(format!(
            "xcomponent_changed: size {}x{} offset ({}, {})",
            size.width, size.height, offset.x, offset.y
        ));
        Ok(())
    });

    xcomponent.on_surface_destroyed(|_xcomponent, _win| {
        hilog_info!("xcomponent_destroy");
        Ok(())
    });

    xcomponent.on_touch_event(|_xcomponent, _win, data| {
        hilog_info!("xcomponent_dispatch");
        hilog_info!(format!("xcomponent_dispatch: {:?}", data));
        Ok(())
    });

    xcomponent.on_mouse_event(|_xcomponent, _win, data| {
        hilog_info!(format!("xcomponent_mouse: {:?}", data));
        Ok(())
    })?;

    xcomponent.on_hover_event(|_xcomponent, is_hover| {
        hilog_info!(format!("xcomponent_hover: {}", is_hover));
        Ok(())
    })?;

    xcomponent.register_callback()?;
    xcomponent.register_mouse_event_callback()?;

    xcomponent.on_frame_callback(|_, _, _| {
        hilog_info!("xcomponent_frame");
        Ok(())
    })?;

    // Key events (hardware keyboard / dpad) on the focused component.
    xcomponent.on_key_event(|_xcomponent, _win, data| {
        hilog_info!(format!("xcomponent_key: {:?}", data));
        Ok(())
    })?;

    // UI input events (axis/rotate events) through ArkUI input.
    xcomponent.on_ui_input_event(|_xcomponent, event| {
        hilog_info!(format!("xcomponent_ui_input: {:?}", event));
        Ok(())
    })?;

    Ok(())
}

/// Constrain the expected frame rate range of the surface's frame callbacks.
#[napi]
pub fn set_frame_rate(min: i32, max: i32, expected: i32) -> Result<()> {
    let raw = XC_RAW
        .lock()
        .unwrap()
        .ok_or_else(|| Error::from_reason("xcomponent not initialized"))?;
    let native = ohos_xcomponent_binding::NativeXComponent::new(raw.0);
    native
        .set_frame_rate(min, max, expected)
        .map_err(|e| Error::from_reason(e.to_string()))
}

#[napi]
pub fn draw_xcomponent() {
    let guard = GL_CTX.lock().unwrap();
    match &*guard {
        Some(render) => {
            unsafe {
                gl::ClearColor(0.0, 0.0, 1.0, 1.0);
                gl::Clear(gl::COLOR_BUFFER_BIT);
            }
            render.surface.swap_buffers(&render.ctx).unwrap()
        }
        None => {}
    }
}
