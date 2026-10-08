//! Safe wrappers for ArkUI key input events.

use std::{ffi::CStr, marker::PhantomData, sync::OnceLock};

use ohos_arkui_input_binding::{sys::ArkUI_UIInputEvent, ArkUIInputEvent};
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

/// ArkUI key code.
///
/// This is the complete `ArkUI_KeyCode` set exposed by the API 26 headers.
/// Unknown values returned by a newer runtime are normalized to
/// [`KeyCode::Unknown`]; use [`KeyEvent::key_code_raw`] when the original value
/// is required.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, EnumFrom)]
#[config(ArkUI_KeyCode, "ArkUI_KeyCode_ARKUI_KEYCODE_")]
pub enum KeyCode {
    Unknown,
    Fn,
    VolumeUp,
    VolumeDown,
    Power,
    Camera,
    VolumeMute,
    Mute,
    BrightnessUp,
    BrightnessDown,
    #[suffix("0")]
    Digit0,
    #[suffix("1")]
    Digit1,
    #[suffix("2")]
    Digit2,
    #[suffix("3")]
    Digit3,
    #[suffix("4")]
    Digit4,
    #[suffix("5")]
    Digit5,
    #[suffix("6")]
    Digit6,
    #[suffix("7")]
    Digit7,
    #[suffix("8")]
    Digit8,
    #[suffix("9")]
    Digit9,
    Star,
    Pound,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    DpadCenter,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Comma,
    Period,
    AltLeft,
    AltRight,
    ShiftLeft,
    ShiftRight,
    Tab,
    Space,
    Sym,
    Explorer,
    Envelope,
    Enter,
    Del,
    Grave,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Slash,
    At,
    Plus,
    Menu,
    PageUp,
    PageDown,
    Escape,
    ForwardDel,
    CtrlLeft,
    CtrlRight,
    CapsLock,
    ScrollLock,
    MetaLeft,
    MetaRight,
    Function,
    Sysrq,
    Break,
    MoveHome,
    MoveEnd,
    Insert,
    Forward,
    MediaPlay,
    MediaPause,
    MediaClose,
    MediaEject,
    MediaRecord,
    #[suffix("F1")]
    F1,
    #[suffix("F2")]
    F2,
    #[suffix("F3")]
    F3,
    #[suffix("F4")]
    F4,
    #[suffix("F5")]
    F5,
    #[suffix("F6")]
    F6,
    #[suffix("F7")]
    F7,
    #[suffix("F8")]
    F8,
    #[suffix("F9")]
    F9,
    #[suffix("F10")]
    F10,
    #[suffix("F11")]
    F11,
    #[suffix("F12")]
    F12,
    NumLock,
    Numpad0,
    Numpad1,
    Numpad2,
    Numpad3,
    Numpad4,
    Numpad5,
    Numpad6,
    Numpad7,
    Numpad8,
    Numpad9,
    NumpadDivide,
    NumpadMultiply,
    NumpadSubtract,
    NumpadAdd,
    NumpadDot,
    NumpadComma,
    NumpadEnter,
    NumpadEquals,
    NumpadLeftParen,
    NumpadRightParen,
    #[cfg(feature = "api-15")]
    ButtonA,
    #[cfg(feature = "api-15")]
    ButtonB,
    #[cfg(feature = "api-15")]
    ButtonX,
    #[cfg(feature = "api-15")]
    ButtonY,
    #[cfg(feature = "api-15")]
    #[suffix("BUTTON_L1")]
    ButtonL1,
    #[cfg(feature = "api-15")]
    #[suffix("BUTTON_R1")]
    ButtonR1,
    #[cfg(feature = "api-15")]
    #[suffix("BUTTON_L2")]
    ButtonL2,
    #[cfg(feature = "api-15")]
    #[suffix("BUTTON_R2")]
    ButtonR2,
    #[cfg(feature = "api-15")]
    ButtonSelect,
    #[cfg(feature = "api-15")]
    ButtonStart,
    #[cfg(feature = "api-15")]
    ButtonMode,
    #[cfg(feature = "api-15")]
    ButtonThumbl,
    #[cfg(feature = "api-15")]
    ButtonThumbr,
}

impl KeyCode {
    pub fn raw(self) -> ArkUI_KeyCode {
        self.into()
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

/// Borrowed view of a callback-scoped ArkUI key input event.
///
/// The wrapper is copyable, but its native event must only be accessed during
/// the callback that produced it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct KeyEvent<'event> {
    input: ArkUIInputEvent,
    _event: PhantomData<&'event super::inner_event::Event>,
}

impl<'event> KeyEvent<'event> {
    pub(crate) fn from_input(
        input: ArkUIInputEvent,
        _event: &'event super::inner_event::Event,
    ) -> Self {
        Self {
            input,
            _event: PhantomData,
        }
    }

    pub fn device_id(self) -> i32 {
        self.input.device_id()
    }

    pub fn event_time(self) -> i64 {
        self.input.event_time()
    }

    pub fn pressed_keys(
        self,
        keys: &mut [i32],
    ) -> Result<usize, ohos_arkui_input_binding::ArkUIInputError> {
        self.input.pressed_keys(keys)
    }

    /// Queries API-19 lock state when supported by the running system.
    /// Missing symbols and failed queries return `None`, not an unlocked state.
    /// This does not introduce a load-time dependency on API-19 symbols.
    pub fn lock_state(self) -> KeyLockState {
        let queries = LockQueries::get();
        KeyLockState {
            caps_lock: queries.query(queries.caps, self.input.raw()),
            num_lock: queries.query(queries.num, self.input.raw()),
            scroll_lock: queries.query(queries.scroll, self.input.raw()),
        }
    }

    pub fn event_type(self) -> KeyEventType {
        KeyEventType::try_from_raw(unsafe { OH_ArkUI_KeyEvent_GetType(self.input.raw()) })
            .unwrap_or(KeyEventType::Unknown)
    }

    pub fn key_code(self) -> KeyCode {
        KeyCode::try_from_raw(self.key_code_raw()).unwrap_or(KeyCode::Unknown)
    }

    /// Returns the unmodified platform key code.
    ///
    /// Prefer [`KeyEvent::key_code`] for normal matching. This method preserves
    /// codes introduced by a runtime newer than the binding headers.
    pub fn key_code_raw(self) -> ArkUI_KeyCode {
        unsafe { OH_ArkUI_KeyEvent_GetKeyCode(self.input.raw()) }
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

/// Lock-key state, available independently for each optional runtime query.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct KeyLockState {
    pub caps_lock: Option<bool>,
    pub num_lock: Option<bool>,
    pub scroll_lock: Option<bool>,
}

type LockQuery = unsafe extern "C" fn(*const ArkUI_UIInputEvent, *mut bool) -> u32;

struct LockQueries {
    _library: libloading::os::unix::Library,
    caps: Option<LockQuery>,
    num: Option<LockQuery>,
    scroll: Option<LockQuery>,
}

impl LockQueries {
    fn get() -> &'static Self {
        static QUERIES: OnceLock<LockQueries> = OnceLock::new();
        QUERIES.get_or_init(|| {
            let library = libloading::os::unix::Library::this();
            // SAFETY: the SDK defines these API-19 symbols with this signature.
            // The process library is retained with the resolved function pointers.
            unsafe {
                Self {
                    caps: library
                        .get::<LockQuery>(b"OH_ArkUI_KeyEvent_IsCapsLockOn\0")
                        .ok()
                        .map(|s| *s),
                    num: library
                        .get::<LockQuery>(b"OH_ArkUI_KeyEvent_IsNumLockOn\0")
                        .ok()
                        .map(|s| *s),
                    scroll: library
                        .get::<LockQuery>(b"OH_ArkUI_KeyEvent_IsScrollLockOn\0")
                        .ok()
                        .map(|s| *s),
                    _library: library,
                }
            }
        })
    }

    fn query(&self, query: Option<LockQuery>, event: *const ArkUI_UIInputEvent) -> Option<bool> {
        let mut state = false;
        // SAFETY: KeyEvent is borrowed from the still-active node callback.
        (unsafe { query?(event, &mut state) } == 0).then_some(state)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn converts_key_codes_and_safely_rejects_future_enums() {
        assert_eq!(KeyCode::Enter.raw(), ArkUI_KeyCode_ARKUI_KEYCODE_ENTER);
        #[cfg(feature = "api-15")]
        assert_eq!(
            KeyCode::try_from_raw(ArkUI_KeyCode_ARKUI_KEYCODE_BUTTON_THUMBR),
            Some(KeyCode::ButtonThumbr)
        );
        assert_eq!(KeyCode::try_from_raw(99_999), None);
        assert_eq!(KeyEventType::try_from_raw(99), None);
        assert_eq!(KeySource::try_from_raw(99), None);
        assert_eq!(KeyIntention::try_from_raw(99), None);
    }
}
