//! Safe Rust bindings for OpenHarmony multimodal InputKit.
//!
//! Provides owned RAII wrappers for key state queries (API 12+), global hotkey
//! monitoring (API 14+), pointer location (API 20+), and window pointer styles /
//! custom cursors (API 22+).

pub use ohos_multi_modal_input_sys as sys;

mod error;
#[cfg(feature = "api-14")]
mod hotkey;
mod key_state;
#[cfg(feature = "api-20")]
mod pointer;

pub use error::{InputError, Result};
#[cfg(feature = "api-14")]
pub use hotkey::{system_hotkeys, HotkeyMonitor, HotkeySpec};
#[cfg(feature = "api-15")]
pub use key_state::function_key_state;
#[cfg(feature = "api-14")]
pub use key_state::interval_since_last_input_us;
#[cfg(feature = "api-20")]
pub use key_state::max_touch_points;
pub use key_state::{query_key_state, KeyStateAction, KeyStateInfo};
#[cfg(all(feature = "api-22", feature = "image"))]
pub use pointer::set_custom_cursor;
#[cfg(feature = "api-22")]
pub use pointer::{
    cursor_info, pointer_style, set_pointer_style, set_pointer_visible, CursorInfo, PointerStyle,
};
#[cfg(feature = "api-20")]
pub use pointer::{pointer_location, PointerLocation};
