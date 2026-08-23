//! Same-library multi-instance XComponent demo with the binding's
//! `multi_mode` feature: callbacks dispatch by XComponent id
//! (X_COMPONENT_CALLBACKS_MAP), so two simultaneously mounted instances keep
//! independent callback state instead of sharing one global slot.
#![allow(clippy::all)]
#![allow(dead_code)]

use std::collections::BTreeMap;
use std::sync::{LazyLock, Mutex};

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Result};
use ohos_hilog_binding::hilog_info;
use ohos_xcomponent_binding::{NativeXComponent, XComponent, XComponentRaw};

/// Per-id surface lifecycle counters, observable from the E2E suite. In
/// multi_mode the dispatch table keys by XComponent id, so both instances
/// must show up here with their own counts.
static EVENTS_BY_ID: LazyLock<Mutex<BTreeMap<String, (u32, u32)>>> =
    LazyLock::new(|| Mutex::new(BTreeMap::new()));

fn bump(id: &str, created: bool) {
    let mut map = EVENTS_BY_ID.lock().unwrap();
    let entry = map.entry(id.to_string()).or_insert((0, 0));
    if created {
        entry.0 += 1;
    } else {
        entry.1 += 1;
    }
}

fn id_of(raw: XComponentRaw) -> String {
    NativeXComponent::new(raw).id().unwrap_or_default()
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
    let id = xcomponent.id().unwrap_or_default();
    hilog_info!("xcomponent_multi init id={id}");

    xcomponent.on_surface_created(|xc, _win| {
        let id = id_of(xc);
        bump(&id, true);
        hilog_info!("multi surface created id={id}");
        Ok(())
    });

    xcomponent.on_surface_destroyed(|xc, _win| {
        let id = id_of(xc);
        bump(&id, false);
        hilog_info!("multi surface destroyed id={id}");
        Ok(())
    });

    xcomponent.on_surface_changed(|xc, win| {
        let size = xc.size(win)?;
        let id = id_of(xc);
        hilog_info!(
            "multi surface changed id={id} {}x{}",
            size.width,
            size.height
        );
        Ok(())
    });

    xcomponent.register_callback()?;
    Ok(())
}

/// Per-id `created/destroyed` counters joined by ';', ids sorted. Empty when
/// no surface has been created yet.
#[napi]
pub fn events_by_id() -> String {
    let map = EVENTS_BY_ID.lock().unwrap();
    map.iter()
        .map(|(id, (created, destroyed))| format!("{id}:{created}/{destroyed}"))
        .collect::<Vec<_>>()
        .join(";")
}

#[napi]
pub fn reset_events() -> Result<()> {
    EVENTS_BY_ID.lock().unwrap().clear();
    Ok(())
}
