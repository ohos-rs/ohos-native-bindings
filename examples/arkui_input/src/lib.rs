use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Result};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::XComponent;

static LAST: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new("touch the surface to emit ArkUI input events".to_string()));

fn set_last(msg: impl Into<String>) {
    let msg = msg.into();
    hilog_info!(format!("arkui_input: {msg}"));
    *LAST.lock().unwrap() = msg;
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
    xcomponent.on_surface_created(|_, _| {
        set_last("surface created — interact to dump ArkUIInputEvent");
        Ok(())
    });
    xcomponent.on_surface_destroyed(|_, _| Ok(()));
    xcomponent.on_touch_event(|_, _, data| {
        set_last(format!("xcomponent touch {data:?}"));
        Ok(())
    });
    xcomponent.register_callback()?;
    xcomponent.on_ui_input_event(|_, event| {
        set_last(format!(
            "type={:?} action={:?} source={:?} tool={:?} time={} pointers={} id0={}",
            event.event_type,
            event.action,
            event.source_type,
            event.tool_type,
            event.event_time(),
            event.pointer_count(),
            event.pointer_id(0)
        ));
        Ok(())
    })?;
    Ok(())
}

#[napi]
pub fn last_event() -> String {
    LAST.lock().unwrap().clone()
}
