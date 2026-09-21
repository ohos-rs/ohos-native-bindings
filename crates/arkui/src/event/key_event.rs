//! Safe wrappers for ArkUI key input events.

use std::ffi::CStr;

use ohos_arkui_input_binding::ArkUIInputEvent;
#[cfg(feature = "api-20")]
use ohos_arkui_input_binding::UIInputEvent;
use ohos_arkui_sys::{
    ArkUI_KeyCode_ARKUI_KEYCODE_DEL, ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_DOWN,
    ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_LEFT, ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_RIGHT,
    ArkUI_KeyCode_ARKUI_KEYCODE_DPAD_UP, ArkUI_KeyCode_ARKUI_KEYCODE_ENTER,
    ArkUI_KeyCode_ARKUI_KEYCODE_ESCAPE, ArkUI_KeyCode_ARKUI_KEYCODE_F10,
    ArkUI_KeyCode_ARKUI_KEYCODE_FORWARD_DEL, ArkUI_KeyCode_ARKUI_KEYCODE_MENU,
    ArkUI_KeyCode_ARKUI_KEYCODE_MOVE_END, ArkUI_KeyCode_ARKUI_KEYCODE_MOVE_HOME,
    ArkUI_KeyCode_ARKUI_KEYCODE_PAGE_DOWN, ArkUI_KeyCode_ARKUI_KEYCODE_PAGE_UP,
    ArkUI_KeyCode_ARKUI_KEYCODE_SPACE, ArkUI_KeyCode_ARKUI_KEYCODE_TAB,
    ArkUI_KeyEventType_ARKUI_KEY_EVENT_CLICK, ArkUI_KeyEventType_ARKUI_KEY_EVENT_DOWN,
    ArkUI_KeyEventType_ARKUI_KEY_EVENT_LONG_PRESS, ArkUI_KeyEventType_ARKUI_KEY_EVENT_UNKNOWN,
    ArkUI_KeyEventType_ARKUI_KEY_EVENT_UP, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_BACK,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_DOWN, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ESCAPE,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_FORWARD, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_HOME,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_LEFT, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_MENU,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_PAGE_DOWN,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_PAGE_UP, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_RIGHT,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_SELECT, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_UNKNOWN,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_UP, ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ZOOM_IN,
    ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ZOOM_OUT, ArkUI_KeyIntension_ARKUI_KEY_INTENTION_CALL,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_CAMERA,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_FAST_FORWARD,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_FAST_PLAYBACK,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_MUTE,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_NEXT,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_PLAY_PAUSE,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_PREVIOUS,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_VOLUME_DOWN,
    ArkUI_KeyIntension_ARKUI_KEY_INTENTION_VOLUME_UP,
    ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_KEYBOARD,
    ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_MOUSE, ArkUI_KeySourceType_ARKUI_KEY_SOURCE_UNKNOWN,
    OH_ArkUI_KeyEvent_GetKeyCode, OH_ArkUI_KeyEvent_GetKeyIntensionCode,
    OH_ArkUI_KeyEvent_GetKeySource, OH_ArkUI_KeyEvent_GetKeyText, OH_ArkUI_KeyEvent_GetType,
    OH_ArkUI_KeyEvent_GetUnicode, OH_ArkUI_KeyEvent_SetConsumed, OH_ArkUI_KeyEvent_StopPropagation,
};
#[cfg(feature = "api-15")]
use ohos_arkui_sys::{
    ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_JOYSTICK, OH_ArkUI_KeyEvent_Dispatch,
};
#[cfg(feature = "api-19")]
use ohos_arkui_sys::{
    OH_ArkUI_KeyEvent_IsCapsLockOn, OH_ArkUI_KeyEvent_IsNumLockOn, OH_ArkUI_KeyEvent_IsScrollLockOn,
};

/// ArkUI key-event phase with a forward-compatible fallback.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyEventType {
    Unknown,
    Down,
    Up,
    LongPress,
    Click,
    Other(i32),
}

impl KeyEventType {
    fn from_raw(value: i32) -> Self {
        if value == ArkUI_KeyEventType_ARKUI_KEY_EVENT_UNKNOWN {
            Self::Unknown
        } else if value == ArkUI_KeyEventType_ARKUI_KEY_EVENT_DOWN {
            Self::Down
        } else if value == ArkUI_KeyEventType_ARKUI_KEY_EVENT_UP {
            Self::Up
        } else if value == ArkUI_KeyEventType_ARKUI_KEY_EVENT_LONG_PRESS {
            Self::LongPress
        } else if value == ArkUI_KeyEventType_ARKUI_KEY_EVENT_CLICK {
            Self::Click
        } else {
            Self::Other(value)
        }
    }
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySource {
    Unknown,
    Mouse,
    Keyboard,
    #[cfg(feature = "api-15")]
    Joystick,
    Other(u32),
}

impl KeySource {
    fn from_raw(value: u32) -> Self {
        if value == ArkUI_KeySourceType_ARKUI_KEY_SOURCE_UNKNOWN {
            Self::Unknown
        } else if value == ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_MOUSE {
            Self::Mouse
        } else if value == ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_KEYBOARD {
            Self::Keyboard
        } else {
            #[cfg(feature = "api-15")]
            if value == ArkUI_KeySourceType_ARKUI_KEY_SOURCE_TYPE_JOYSTICK {
                return Self::Joystick;
            }
            Self::Other(value)
        }
    }
}

/// Semantic intention derived by ArkUI for a key event.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
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
    MediaPlayPause,
    MediaFastForward,
    MediaFastPlayback,
    MediaNext,
    MediaPrevious,
    MediaMute,
    VolumeUp,
    VolumeDown,
    Call,
    Camera,
    Other(i32),
}

impl KeyIntention {
    fn from_raw(value: i32) -> Self {
        let known = [
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENSION_UNKNOWN,
                Self::Unknown,
            ),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_UP, Self::Up),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_DOWN, Self::Down),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_LEFT, Self::Left),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_RIGHT, Self::Right),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_SELECT, Self::Select),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ESCAPE, Self::Escape),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_BACK, Self::Back),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENSION_FORWARD,
                Self::Forward,
            ),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_MENU, Self::Menu),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_HOME, Self::Home),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_PAGE_UP, Self::PageUp),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENSION_PAGE_DOWN,
                Self::PageDown,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ZOOM_OUT,
                Self::ZoomOut,
            ),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENSION_ZOOM_IN, Self::ZoomIn),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_PLAY_PAUSE,
                Self::MediaPlayPause,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_FAST_FORWARD,
                Self::MediaFastForward,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_FAST_PLAYBACK,
                Self::MediaFastPlayback,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_NEXT,
                Self::MediaNext,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_PREVIOUS,
                Self::MediaPrevious,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_MEDIA_MUTE,
                Self::MediaMute,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_VOLUME_UP,
                Self::VolumeUp,
            ),
            (
                ArkUI_KeyIntension_ARKUI_KEY_INTENTION_VOLUME_DOWN,
                Self::VolumeDown,
            ),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENTION_CALL, Self::Call),
            (ArkUI_KeyIntension_ARKUI_KEY_INTENTION_CAMERA, Self::Camera),
        ];
        known
            .into_iter()
            .find_map(|(raw, intention)| (raw == value).then_some(intention))
            .unwrap_or(Self::Other(value))
    }
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
        KeyEventType::from_raw(unsafe { OH_ArkUI_KeyEvent_GetType(self.input.raw()) })
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
        KeySource::from_raw(unsafe { OH_ArkUI_KeyEvent_GetKeySource(self.input.raw()) })
    }

    pub fn stop_propagation(self, stop: bool) {
        unsafe { OH_ArkUI_KeyEvent_StopPropagation(self.input.raw(), stop) };
    }

    pub fn intention(self) -> KeyIntention {
        KeyIntention::from_raw(unsafe { OH_ArkUI_KeyEvent_GetKeyIntensionCode(self.input.raw()) })
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
    fn preserves_future_key_values() {
        assert_eq!(KeyCode::from_raw(99_999).raw(), 99_999);
        assert_eq!(KeyEventType::from_raw(99), KeyEventType::Other(99));
        assert_eq!(KeySource::from_raw(99), KeySource::Other(99));
        assert_eq!(KeyIntention::from_raw(99), KeyIntention::Other(99));
    }
}
