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

    xcomponent.on_surface_changed(|_xcomponent, _win| {
        hilog_info!("xcomponent_changed");
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

    xcomponent.register_callback()?;

    xcomponent.on_frame_callback(|_, _, _| {
        hilog_info!("xcomponent_frame");
        Ok(())
    })?;

    Ok(())
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
