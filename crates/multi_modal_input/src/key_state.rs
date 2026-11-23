use std::ptr::NonNull;

use ohos_multi_modal_input_sys as sys;

use crate::error::{check_status, InputError, Result};

/// Press or toggle state reported for a key (API 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum KeyStateAction {
    Default,
    Pressed,
    Released,
    SwitchOn,
    SwitchOff,
    Unknown(i32),
}

impl KeyStateAction {
    #[allow(non_upper_case_globals)]
    pub const fn from_raw(value: sys::Input_KeyStateAction) -> Self {
        match value {
            sys::Input_KeyStateAction_KEY_DEFAULT => Self::Default,
            sys::Input_KeyStateAction_KEY_PRESSED => Self::Pressed,
            sys::Input_KeyStateAction_KEY_RELEASED => Self::Released,
            sys::Input_KeyStateAction_KEY_SWITCH_ON => Self::SwitchOn,
            sys::Input_KeyStateAction_KEY_SWITCH_OFF => Self::SwitchOff,
            other => Self::Unknown(other),
        }
    }
}

/// Snapshot of a queried key's state (API 12).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct KeyStateInfo {
    pub key_code: i32,
    pub pressed: KeyStateAction,
    pub switch: KeyStateAction,
}

struct OwnedKeyState(NonNull<sys::Input_KeyState>);

impl OwnedKeyState {
    fn new() -> Result<Self> {
        NonNull::new(unsafe { sys::OH_Input_CreateKeyState() })
            .map(Self)
            .ok_or(InputError::AllocationFailed)
    }

    fn as_ptr(&self) -> *mut sys::Input_KeyState {
        self.0.as_ptr()
    }
}

impl Drop for OwnedKeyState {
    fn drop(&mut self) {
        let mut raw = self.0.as_ptr();
        unsafe { sys::OH_Input_DestroyKeyState(&mut raw) };
    }
}

/// Queries the current press and toggle state of a key code (API 12).
pub fn query_key_state(key_code: i32) -> Result<KeyStateInfo> {
    let state = OwnedKeyState::new()?;
    unsafe {
        sys::OH_Input_SetKeyCode(state.as_ptr(), key_code);
        check_status(sys::OH_Input_GetKeyState(state.as_ptr()))?;
        let key_code = sys::OH_Input_GetKeyCode(state.as_ptr());
        let pressed = KeyStateAction::from_raw(sys::OH_Input_GetKeyPressed(state.as_ptr()));
        let switch = KeyStateAction::from_raw(sys::OH_Input_GetKeySwitch(state.as_ptr()));
        Ok(KeyStateInfo {
            key_code,
            pressed,
            switch,
        })
    }
}

/// Queries the interval in microseconds since the last system input event (API 14).
#[cfg(feature = "api-14")]
pub fn interval_since_last_input_us() -> Result<i64> {
    let mut interval = 0;
    check_status(unsafe { sys::OH_Input_GetIntervalSinceLastInput(&mut interval) })?;
    Ok(interval)
}

/// Queries the enabled state of a function key such as CapsLock (`key_code = 1`) (API 15).
#[cfg(feature = "api-15")]
pub fn function_key_state(key_code: i32) -> Result<bool> {
    let mut state = 0;
    check_status(unsafe { sys::OH_Input_GetFunctionKeyState(key_code, &mut state) })?;
    Ok(state != 0)
}

/// Queries the maximum number of touch points supported by the device (API 20).
#[cfg(feature = "api-20")]
pub fn max_touch_points() -> Result<i32> {
    let mut count = 0;
    check_status(unsafe { sys::OH_Input_QueryMaxTouchPoints(&mut count) })?;
    Ok(count)
}
