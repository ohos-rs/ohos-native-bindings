use crate::raw::{WindowRaw, XComponentRaw};
use napi_ohos::Result;
use ohos_arkui_input_binding::ArkUIInputEvent;
#[cfg(feature = "multi_mode")]
use ohos_xcomponent_sys::OH_NativeXComponent;
use std::{cell::RefCell, rc::Rc};

#[cfg(feature = "multi_mode")]
use std::collections::HashMap;

mod key_event;
mod mouse_event;
mod native_callbacks;
mod raw_window;
mod touch_event;

pub use key_event::*;
pub use mouse_event::*;
pub use native_callbacks::*;
pub use raw_window::*;
pub use touch_event::*;

pub type OnSurfaceCreated = Option<Rc<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type OnSurfaceChanged = Option<Rc<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type OnSurfaceDestroyed = Option<Rc<dyn Fn(XComponentRaw, WindowRaw) -> Result<()>>>;
pub type DispatchTouchEvent =
    Option<Rc<dyn Fn(XComponentRaw, WindowRaw, TouchEventData) -> Result<()>>>;
pub type OnFrameChange = Option<Rc<dyn Fn(XComponentRaw, u64, u64) -> Result<()>>>;
pub type OnKeyEvent = Option<Rc<dyn Fn(XComponentRaw, WindowRaw, KeyEventData) -> Result<()>>>;
pub type OnMouseEvent = Option<Rc<dyn Fn(XComponentRaw, WindowRaw, MouseEventData) -> Result<()>>>;
pub type OnHoverEvent = Option<Rc<dyn Fn(XComponentRaw, bool) -> Result<()>>>;
pub type OnUIInputEvent = Option<Rc<dyn Fn(XComponentRaw, ArkUIInputEvent) -> Result<()>>>;

#[derive(Default, Clone)]
pub struct XComponentCallbacks {
    pub on_surface_created: OnSurfaceCreated,
    pub on_surface_changed: OnSurfaceChanged,
    pub on_surface_destroyed: OnSurfaceDestroyed,
    pub dispatch_touch_event: DispatchTouchEvent,
    pub on_frame_change: OnFrameChange,
    pub on_key_event: OnKeyEvent,
    pub on_mouse_event: OnMouseEvent,
    pub on_hover_event: OnHoverEvent,
    pub on_ui_input_event: OnUIInputEvent,
}

#[cfg(feature = "multi_mode")]
pub(crate) fn callback_key(component: *mut OH_NativeXComponent) -> usize {
    component as usize
}

thread_local! {
    #[cfg(not(feature = "multi_mode"))]
    pub static X_COMPONENT_CALLBACKS: RefCell<XComponentCallbacks> = RefCell::new(XComponentCallbacks::default());

    #[cfg(feature = "multi_mode")]
    pub static X_COMPONENT_CALLBACKS_MAP: RefCell<HashMap<usize, XComponentCallbacks>> = RefCell::new(HashMap::default());
}

#[cfg(all(test, feature = "multi_mode"))]
mod tests {
    use super::callback_key;
    use ohos_xcomponent_sys::OH_NativeXComponent;

    #[test]
    fn callback_keys_are_per_native_instance() {
        let first = 1_usize as *mut OH_NativeXComponent;
        let second = 2_usize as *mut OH_NativeXComponent;
        assert_ne!(callback_key(first), callback_key(second));
        assert_eq!(callback_key(first), callback_key(first));
    }
}
