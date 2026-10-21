//! Owned snapshots and scoped API-12 window key filtering.
mod registration;
use ohos_enum_derive::EnumFrom;
use ohos_multi_modal_input_sys::*;
pub use registration::WindowKeyFilter;

/// Physical key identity, independent of the active text layout.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(Input_KeyCode, "Input_KeyCode_KEYCODE_")]
pub enum KeyCode {
    Unknown,
    DpadUp,
    DpadDown,
    DpadLeft,
    DpadRight,
    Enter,
    Del,
    Tab,
    Space,
    Escape,
    ForwardDel,
    Insert,
    MoveHome,
    MoveEnd,
    PageUp,
    PageDown,
    CtrlLeft,
    CtrlRight,
    ShiftLeft,
    ShiftRight,
    AltLeft,
    AltRight,
    MetaLeft,
    MetaRight,
    Comma,
    Period,
    Grave,
    Minus,
    Equals,
    LeftBracket,
    RightBracket,
    Backslash,
    Semicolon,
    Apostrophe,
    Slash,
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
}
/// Key phase delivered by the API-12 window filter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeyAction {
    Down,
    Up,
    Cancel,
    Unknown,
}
/// Callback-owned primitive data; no native event pointer escapes the callback.
#[derive(Debug, Clone, Copy)]
pub struct KeyEvent {
    pub code: KeyCode,
    pub raw_code: i32,
    pub action: KeyAction,
    /// Monotonic event time in microseconds.
    pub time: i64,
}
const CALLBACKS: [crate::KeyEventFilter; 32] = [
    dispatch::<0>,
    dispatch::<1>,
    dispatch::<2>,
    dispatch::<3>,
    dispatch::<4>,
    dispatch::<5>,
    dispatch::<6>,
    dispatch::<7>,
    dispatch::<8>,
    dispatch::<9>,
    dispatch::<10>,
    dispatch::<11>,
    dispatch::<12>,
    dispatch::<13>,
    dispatch::<14>,
    dispatch::<15>,
    dispatch::<16>,
    dispatch::<17>,
    dispatch::<18>,
    dispatch::<19>,
    dispatch::<20>,
    dispatch::<21>,
    dispatch::<22>,
    dispatch::<23>,
    dispatch::<24>,
    dispatch::<25>,
    dispatch::<26>,
    dispatch::<27>,
    dispatch::<28>,
    dispatch::<29>,
    dispatch::<30>,
    dispatch::<31>,
];
unsafe extern "C" fn dispatch<const SLOT: usize>(event: *mut Input_KeyEvent) -> bool {
    if event.is_null() {
        return false;
    }
    // SAFETY: the window manager retains the non-null event for this callback.
    let (raw_code, action, time) = unsafe {
        (
            OH_Input_GetKeyEventKeyCode(event),
            OH_Input_GetKeyEventAction(event),
            OH_Input_GetKeyEventActionTime(event),
        )
    };
    let action = match action as u32 {
        x if x == Input_KeyEventAction_KEY_ACTION_DOWN => KeyAction::Down,
        x if x == Input_KeyEventAction_KEY_ACTION_UP => KeyAction::Up,
        x if x == Input_KeyEventAction_KEY_ACTION_CANCEL => KeyAction::Cancel,
        _ => KeyAction::Unknown,
    };
    let event = KeyEvent {
        code: KeyCode::try_from_raw(raw_code).unwrap_or(KeyCode::Unknown),
        raw_code,
        action,
        time,
    };
    registration::dispatch(SLOT, event)
}
