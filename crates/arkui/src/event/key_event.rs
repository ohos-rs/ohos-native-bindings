//! Safe wrappers for ArkUI key input events.

use std::ffi::CStr;

use ohos_arkui_input_binding::ArkUIInputEvent;
#[cfg(feature = "api-20")]
use ohos_arkui_input_binding::UIInputEvent;
use ohos_arkui_sys::*;
use ohos_enum_derive::EnumFrom;

/// ArkUI key-event phase.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeyEventType, "ArkUI_KeyEventType_ARKUI_KEY_EVENT_")]
pub enum KeyEventType {
    Unknown,
    Down,
    Up,
    LongPress,
    Click,
}

/// Lossless ArkUI key code.
///
/// Named constants cover navigation and activation keys commonly consumed by
/// UI frameworks. Unknown and future platform values remain available through
/// [`KeyCode::raw`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct KeyCode(i32);

impl KeyCode {
    pub const ENTER: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_ENTER);
    pub const SPACE: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_SPACE);
    pub const TAB: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_TAB);
    pub const ESCAPE: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_ESCAPE);
    pub const DPAD_UP: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_UP);
    pub const DPAD_DOWN: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_DOWN);
    pub const DPAD_LEFT: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_LEFT);
    pub const DPAD_RIGHT: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_RIGHT);
    pub const MOVE_HOME: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_MOVE_HOME);
    pub const MOVE_END: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_MOVE_END);
    pub const PAGE_UP: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_PAGE_UP);
    pub const PAGE_DOWN: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_PAGE_DOWN);
    pub const BACKSPACE: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_DEL);
    pub const DELETE: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_FORWARD_DEL);
    pub const MENU: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_MENU);
    pub const F10: Self = Self(ArkUI_KeyCode_ARKUI_KEYCODE_F10);

    pub const fn from_raw(value: i32) -> Self {
        Self(value)
    }

    pub const fn raw(self) -> i32 {
        self.0
    }
}

/// Device category that produced a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeySourceType, "ArkUI_KeySourceType_ARKUI_KEY_SOURCE_")]
pub enum KeySource {
    Unknown,
    #[suffix("TYPE_MOUSE")]
    Mouse,
    #[suffix("TYPE_KEYBOARD")]
    Keyboard,
    #[cfg(feature = "api-15")]
    #[suffix("TYPE_JOYSTICK")]
    Joystick,
}

/// Semantic intention derived by ArkUI for a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeyIntension, "ArkUI_KeyIntension_ARKUI_KEY_INTENSION_")]
pub enum KeyIntention {
    Unknown,
    Up,
    Down,
    Left,
    Right,
    Select,
    Escape,
    Back,
    Forward,
    Menu,
    Home,
    PageUp,
    PageDown,
    ZoomOut,
    ZoomIn,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaPlayPause,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaFastForward,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaFastPlayback,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaNext,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaPrevious,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaMute,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    VolumeUp,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    VolumeDown,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    Call,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    Camera,
}

/// Borrow-free view of a callback-scoped ArkUI key input event.
///
/// The wrapper is copyable, but its native event must only be accessed during
/// the callback that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent {
    input: ArkUIInputEvent,
}

impl KeyEvent {
    #[cfg(feature = "api-20")]
    pub(crate) fn from_input(input: ArkUIInputEvent) -> Option<Self> {
        (input.event_type == UIInputEvent::Key).then_some(Self { input })
    }

    pub fn event_type(self) -> KeyEventType {
        KeyEventType::try_from_raw(unsafe { OH_ArkUI_KeyEvent_GetType(self.input.raw()) })
            .unwrap_or(KeyEventType::Unknown)
    }

    pub fn key_code(self) -> KeyCode {
        KeyCode::from_raw(unsafe { OH_ArkUI_KeyEvent_GetKeyCode(self.input.raw()) })
    }

    pub fn key_text(self) -> String {
        let value = unsafe { OH_ArkUI_KeyEvent_GetKeyText(self.input.raw()) };
        if value.is_null() {
            String::new()
        } else {
            unsafe { CStr::from_ptr(value) }
                .to_string_lossy()
                .into_owned()
        }
    }

    pub fn source(self) -> KeySource {
        KeySource::try_from_raw(unsafe { OH_ArkUI_KeyEvent_GetKeySource(self.input.raw()) })
            .unwrap_or(KeySource::Unknown)
    }

    pub fn stop_propagation(self, stop: bool) {
        unsafe { OH_ArkUI_KeyEvent_StopPropagation(self.input.raw(), stop) };
    }

    pub fn intention(self) -> KeyIntention {
        KeyIntention::try_from_raw(unsafe {
            OH_ArkUI_KeyEvent_GetKeyIntensionCode(self.input.raw())
        })
        .unwrap_or(KeyIntention::Unknown)
    }

    pub fn unicode(self) -> u32 {
        unsafe { OH_ArkUI_KeyEvent_GetUnicode(self.input.raw()) }
    }

    pub fn set_consumed(self, consumed: bool) {
        unsafe { OH_ArkUI_KeyEvent_SetConsumed(self.input.raw(), consumed) };
    }

    #[cfg(feature = "api-15")]
    pub fn dispatch_to(self, node: &crate::ArkUINode) {
        unsafe { OH_ArkUI_KeyEvent_Dispatch(node.raw_handle(), self.input.raw()) };
    }

    #[cfg(feature = "api-19")]
    pub fn num_lock_on(self) -> crate::ArkUIResult<bool> {
        let mut state = false;
        unsafe {
            crate::check_arkui_status!(OH_ArkUI_KeyEvent_IsNumLockOn(self.input.raw(), &mut state))
        }?;
        Ok(state)
    }

    #[cfg(feature = "api-19")]
    pub fn caps_lock_on(self) -> crate::ArkUIResult<bool> {
        let mut state = false;
        unsafe {
            crate::check_arkui_status!(OH_ArkUI_KeyEvent_IsCapsLockOn(self.input.raw(), &mut state))
        }?;
        Ok(state)
    }

    #[cfg(feature = "api-19")]
    pub fn scroll_lock_on(self) -> crate::ArkUIResult<bool> {
        let mut state = false;
        unsafe {
            crate::check_arkui_status!(OH_ArkUI_KeyEvent_IsScrollLockOn(
                self.input.raw(),
                &mut state
            ))
        }?;
        Ok(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn preserves_future_key_codes_and_safely_rejects_future_enums() {
        assert_eq!(KeyCode::from_raw(99_999).raw(), 99_999);
        assert_eq!(KeyEventType::try_from_raw(99), None);
        assert_eq!(KeySource::try_from_raw(99), None);
        assert_eq!(KeyIntention::try_from_raw(99), None);
    }
}
