use crate::raw::{WindowRaw, XComponentRaw};
use napi_ohos::Result;

#[cfg(feature = "multi_mode")]
use std::collections::HashMap;

use std::cell::RefCell;

mod key_event;
mod native_callbacks;
mod raw_window;
mod touch_event;

pub use key_event::*;
pub use native_callbacks::*;
pub use raw_window::*;
pub use touch_event::*;

pub type OnSurfaceCreated = Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type OnSurfaceChanged = Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type OnSurfaceDestroyed = Option<Box<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type DispatchTouchEvent =
    Option<Box<dyn Fn(XComponentRaw, WindowRaw, TouchEventData) -> Result<()>>>;
pub type OnFrameChange = Option<Box<dyn Fn(XComponentRaw, u64, u64) -> Result<()>>>;
pub type OnKeyEvent = Option<Box<dyn Fn(XComponentRaw, WindowRaw, KeyEventData) -> Result<()>>>;

#[derive(Default)]
pub struct XComponentCallbacks {
    pub on_surface_created: OnSurfaceCreated,
    pub on_surface_changed: OnSurfaceChanged,
    pub on_surface_destroyed: OnSurfaceDestroyed,
    pub dispatch_touch_event: DispatchTouchEvent,
    pub on_frame_change: OnFrameChange,
    pub on_key_event: OnKeyEvent,
}

thread_local! {
    #[cfg(feature = "single_mode")]
    pub static X_COMPONENT_CALLBACKS: RefCell<XComponentCallbacks> = RefCell::new(XComponentCallbacks::default());

    #[cfg(feature = "multi_mode")]
    pub static X_COMPONENT_CALLBACKS_MAP: RefCell<HashMap<String, XComponentCallbacks>> = RefCell::new(HashMap::default());
}
