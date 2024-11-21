use crate::raw::{WindowRaw, XComponentRaw};
use napi_ohos::Result;

#[cfg(feature = "single_mode")]
use std::cell::RefCell;

#[cfg(feature = "multi_mode")]
use std::collections::HashMap;

#[cfg(feature = "multi_mode")]
use std::cell::RefCell;

mod native_callbacks;

pub use native_callbacks::*;

pub struct XComponentCallbacks {
    pub on_surface_created: Option<fn(XComponentRaw, WindowRaw) -> Result<()>>,
    pub on_surface_changed: Option<fn(XComponentRaw, WindowRaw) -> Result<()>>,
    pub on_surface_destroyed: Option<fn(XComponentRaw, WindowRaw) -> Result<()>>,
    pub dispatch_touch_event: Option<fn(XComponentRaw, WindowRaw) -> Result<()>>,
    pub on_frame_change: Option<fn(XComponentRaw, u64, u64) -> Result<()>>,
}

impl Default for XComponentCallbacks {
    fn default() -> Self {
        XComponentCallbacks {
            on_surface_changed: None,
            on_surface_created: None,
            on_surface_destroyed: None,
            dispatch_touch_event: None,
            on_frame_change: None,
        }
    }
}

thread_local! {
    #[cfg(feature = "single_mode")]
    pub static X_COMPONENT_CALLBACKS: RefCell<XComponentCallbacks> = RefCell::new(XComponentCallbacks::default());

    #[cfg(feature = "multi_mode")]
    pub static X_COMPONENT_CALLBACKS_MAP: RefCell<HashMap<String, XComponentCallbacks>> = RefCell::new(HashMap::default());
}
