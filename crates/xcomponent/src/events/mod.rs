use crate::raw::{WindowRaw, XComponentRaw};
use napi_ohos::Result;
use std::cell::RefCell;

#[cfg(feature = "multi_mode")]
use std::collections::HashMap;


mod key_event;
mod native_callbacks;
mod raw_window;
mod touch_event;

pub use key_event::*;
pub use native_callbacks::*;
pub use raw_window::*;
pub use touch_event::*;

pub struct XComponentCallbacks {
    pub on_surface_created: Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>,
    pub on_surface_changed: Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>,
    pub on_surface_destroyed: Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>,
    pub dispatch_touch_event:
        Option<Box<dyn Fn(XComponentRaw, WindowRaw, TouchEventData) -> Result<()>>>,
    pub on_frame_change: Option<Box<dyn Fn(XComponentRaw, u64, u64) -> Result<()>>>,
    pub on_key_event: Option<Box<dyn Fn(XComponentRaw, WindowRaw, KeyEventData) -> Result<()>>>,
}

impl Default for XComponentCallbacks {
    fn default() -> Self {
        XComponentCallbacks {
            on_surface_changed: None,
            on_surface_created: None,
            on_surface_destroyed: None,
            dispatch_touch_event: None,
            on_frame_change: None,
            on_key_event: None,
        }
    }
}

thread_local! {
    #[cfg(not(feature = "multi_mode"))]
    pub static X_COMPONENT_CALLBACKS: RefCell<XComponentCallbacks> = RefCell::new(XComponentCallbacks::default());

    #[cfg(feature = "multi_mode")]
    pub static X_COMPONENT_CALLBACKS_MAP: RefCell<HashMap<String, XComponentCallbacks>> = RefCell::new(HashMap::default());
}
