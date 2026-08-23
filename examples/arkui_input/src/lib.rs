use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Result};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::XComponent;

static LAST: LazyLock<Mutex<String>> =
    LazyLock::new(|| Mutex::new("touch the surface to emit ArkUI input events".to_string()));

/// Clone/modify/readback outcome of the most recent UI input event, recorded
/// inside the callback (the ArkUI event is only valid there). This exercises
/// the binding's cloned-event API family: create_cloned_event,
/// set_action_type, set_local_position, set_finger_id_by_index.
static CLONED: LazyLock<Mutex<String>> = LazyLock::new(|| Mutex::new(String::new()));

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

        // Exercise the cloned-event family while the event is live.
        if let Ok(clone) = event.create_cloned_event() {
            let report =
                (|| -> std::result::Result<String, ohos_arkui_input_binding::ArkUIInputError> {
                    clone.set_action_type(event.action)?;
                    clone.set_local_position(10.5, 20.5)?;
                    clone.set_finger_id_by_index(7, 0)?;
                    // Read the clone back through ArkUIInputEvent::from_raw —
                    // the cloned event only exposes setters.
                    let read = ohos_arkui_input_binding::ArkUIInputEvent::from_raw(clone.raw());
                    let x = read.pointer_x_by_index(0);
                    let y = read.pointer_y_by_index(0);
                    Ok(format!(
                        "clone ok action_set={:?} pos=({x},{y}) finger_id={}",
                        event.action,
                        read.pointer_id(0)
                    ))
                })()
                .unwrap_or_else(|e| format!("clone ERR {e}"));
            *CLONED.lock().unwrap() = report.clone();
            hilog_info!(format!("arkui_input cloned: {report}"));
        }
        Ok(())
    })?;
    Ok(())
}

#[napi]
pub fn last_event() -> String {
    LAST.lock().unwrap().clone()
}

/// Clone/modify/readback report of the most recent UI input event (see
/// CLONED). Empty until the first event arrives.
#[napi]
pub fn last_cloned_event() -> String {
    CLONED.lock().unwrap().clone()
}
