use std::{
    mem::MaybeUninit,
    sync::{LazyLock, Mutex},
};

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, Result};
use ohos_hilog_binding::hilog_info;
use ohos_native_window_binding::NativeWindow;
use ohos_xcomponent_binding::{WindowRaw, XComponent, XComponentRaw};

static LAST: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new("(no surface yet)".to_string()));

fn set_last(msg: impl Into<String>) {
    let msg = msg.into();
    hilog_info!(format!("native_window: {msg}"));
    *LAST.lock().unwrap() = msg;
}

fn fill_window(xc: XComponentRaw, win: WindowRaw) -> Result<()> {
    let size = xc.size(win)?;
    let window = NativeWindow::clone_from_ptr(win.0);
    let surface_id = window
        .surface_id()
        .map_err(|e| Error::from_reason(e.to_string()))?;
    window
        .set_buffer_geometry(size.width as i32, size.height as i32)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let mut buffer = window
        .request_buffer(None)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let width = buffer.width();
    let height = buffer.height();
    let stride = buffer.stride();
    let format = format!("{:?}", buffer.format());
    if let Some(bytes) = buffer.bytes() {
        for pixel in bytes.as_chunks_mut::<4>().0 {
            pixel[0] = MaybeUninit::new(40);
            pixel[1] = MaybeUninit::new(120);
            pixel[2] = MaybeUninit::new(220);
            pixel[3] = MaybeUninit::new(255);
        }
    }
    drop(buffer);
    set_last(format!(
        "surface id={surface_id} {}x{} buffer {width}x{height} stride={stride} format={format} filled",
        size.width, size.height
    ));
    Ok(())
}

#[napi(module_exports)]
pub fn init(exports: Object, env: Env) -> Result<()> {
    // Outside an XComponent host (e.g. imported by a test runner without a
    // native surface) there is no __NATIVE_XCOMPONENT_OBJ__ in exports; skip
    // binding instead of failing module registration.
    let xcomponent = match XComponent::init(env, exports) {
        Ok(xc) => xc,
        Err(e) => {
            hilog_info!("no XComponent surface, skip init: {e}");
            return Ok(());
        }
    };
    xcomponent.on_surface_created(fill_window);
    xcomponent.on_surface_changed(fill_window);
    xcomponent.on_surface_destroyed(|_, _| {
        set_last("surface destroyed");
        Ok(())
    });
    xcomponent.register_callback()?;
    Ok(())
}

#[napi]
pub fn last_result() -> String {
    LAST.lock().unwrap().clone()
}

#[napi]
pub fn redraw() -> Result<String> {
    Ok(last_result())
}
