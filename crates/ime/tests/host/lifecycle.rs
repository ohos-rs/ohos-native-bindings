#![allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
extern crate self as ohos_input_method_sys;

use std::cell::RefCell;

pub const InputMethod_ErrorCode_IME_ERR_DETACHED: u32 = 12800009;
pub const InputMethod_ErrorCode_IME_ERR_NULL_POINTER: u32 = 12802000;
pub type InputMethod_InputMethodProxy = u8;
pub type InputMethod_TextEditorProxy = u8;
#[derive(Clone, Copy)]
pub enum EnterKey {
    Unspecified,
}
impl From<EnterKey> for u32 {
    fn from(_: EnterKey) -> Self {
        0
    }
}
#[derive(Clone, Copy)]
pub enum InputType {
    Text,
}
impl From<InputType> for u32 {
    fn from(_: InputType) -> Self {
        0
    }
}
#[derive(Clone, Copy)]
pub struct Rect;
#[derive(Clone, Copy)]
pub struct Selection {
    pub start: i32,
    pub end: i32,
}
#[derive(Clone, Copy)]
pub struct Action;
#[derive(Clone, Copy)]
pub struct Direction;
#[derive(Clone, Copy)]
pub struct KeyboardStatus;
#[derive(Clone, Copy)]
pub struct PrivateCommand;
pub mod private_command {
    pub use crate::PrivateCommand;
}

#[derive(Default)]
struct Native {
    active: usize,
    attaches: usize,
    shows: usize,
    destroyed: usize,
    detach_error: u32,
    attach_error: u32,
    window_id: i32,
    consume: bool,
}
thread_local! { static NATIVE: RefCell<Native> = RefCell::default(); }
fn native<T>(f: impl FnOnce(&mut Native) -> T) -> T {
    NATIVE.with(|state| f(&mut state.borrow_mut()))
}

pub struct TextConfig;
impl TextConfig {
    fn set_window_id(&self, id: i32) {
        native(|state| state.window_id = id);
    }
    fn set_enter_key(&self, _: EnterKey) {}
    fn set_input_type(&self, _: InputType) {}
    fn set_preview_text_supported(&self, _: bool) {}
    fn set_selection(&self, _: Selection) {}
    #[cfg(feature = "api-26")]
    fn set_consume_key_events(&self, enabled: bool) -> ImeResult<()> {
        native(|state| state.consume = enabled);
        Ok(())
    }
    fn cursor(&self) -> Cursor {
        Cursor::new(Rect)
    }
}
pub struct Cursor {
    raw: *mut u8,
}
impl Cursor {
    fn new(_: Rect) -> Self {
        Self {
            raw: std::ptr::null_mut(),
        }
    }
    fn set_rect(&self, _: Rect) {}
}
pub struct AttachOptions {
    raw: bool,
}
impl AttachOptions {
    pub fn new(show: bool) -> Self {
        Self { raw: show }
    }
}
pub struct TextEditor {
    raw: *mut u8,
}
impl TextEditor {
    fn new() -> Self {
        Self {
            raw: Box::into_raw(Box::new(0)),
        }
    }
    fn set_callback_in_main_thread(&self, _: bool) -> ImeResult<()> {
        Ok(())
    }
}
impl Drop for TextEditor {
    fn drop(&mut self) {
        native(|state| {
            assert_ne!(state.active, self.raw as usize, "freed a retained editor");
            state.destroyed += 1;
        });
        // SAFETY: new allocated this table; successful detach released native use.
        unsafe {
            drop(Box::from_raw(self.raw));
        }
    }
}
unsafe fn OH_InputMethodController_Attach(editor: *mut u8, _: bool, raw: *mut *mut u8) -> u32 {
    let status = native(|state| {
        state.attaches += 1;
        state.active = editor as usize;
        state.attach_error
    });
    let callbacks = proxy::callbacks_for(editor).unwrap();
    let callback = callbacks.read().unwrap().get_text_config.clone().unwrap();
    callback(TextConfig);
    if status == 0 {
        // SAFETY: production passes a live output slot.
        unsafe {
            *raw = editor;
        }
    }
    status
}
unsafe fn OH_InputMethodController_Detach(_: *mut u8) -> u32 {
    native(|state| {
        if state.detach_error == 0 {
            state.active = 0;
        }
        state.detach_error
    })
}
unsafe fn OH_InputMethodProxy_ShowKeyboard(_: *mut u8) -> u32 {
    native(|state| state.shows += 1);
    0
}
unsafe fn OH_InputMethodProxy_HideKeyboard(_: *mut u8) -> u32 {
    0
}
unsafe fn OH_InputMethodProxy_NotifyConfigurationChange(_: *mut u8, _: u32, _: u32) -> u32 {
    0
}
unsafe fn OH_InputMethodProxy_NotifyCursorUpdate(_: *mut u8, _: *mut u8) -> u32 {
    0
}
unsafe fn OH_InputMethodProxy_NotifySelectionChange(
    _: *mut u8,
    _: *mut u16,
    _: usize,
    _: i32,
    _: i32,
) -> u32 {
    0
}

#[path = "../../src/error.rs"]
mod error;
pub use error::*;
#[path = "../../src/text_state.rs"]
mod text_state;
pub use text_state::*;
#[path = "../../src/ime.rs"]
mod ime;
#[path = "../../src/proxy/callbacks.rs"]
mod proxy;
pub use ime::*;

#[test]
fn native_window_configuration_is_used_on_each_attach() {
    let ime = IME::new_for_window(AttachOptions::new(false), 42);
    ime.try_attach().unwrap();
    assert_eq!(native(|state| state.window_id), 42);
    ime.try_detach().unwrap();
    ime.set_window_id(57);
    ime.try_attach().unwrap();
    assert_eq!(native(|state| state.window_id), 57);
    ime.try_detach().unwrap();
}

#[test]
fn show_attached_does_not_implicitly_attach() {
    let ime = IME::new(AttachOptions::new(false));
    assert_eq!(
        ime.show_attached().unwrap_err().code(),
        InputMethod_ErrorCode_IME_ERR_DETACHED
    );
    assert_eq!(native(|state| state.attaches), 0);
    ime.try_attach().unwrap();
    ime.show_attached().unwrap();
    assert_eq!(native(|state| (state.attaches, state.shows)), (1, 1));
}

#[test]
fn failed_detach_keeps_the_table_for_an_explicit_retry() {
    let ime = IME::new(AttachOptions::new(false));
    ime.try_attach().unwrap();
    native(|state| state.detach_error = 12800003);
    assert!(ime.try_detach().is_err());
    assert_eq!(native(|state| state.destroyed), 0);
    native(|state| state.detach_error = 0);
    ime.try_detach().unwrap();
    assert_eq!(native(|state| state.destroyed), 1);
}

#[test]
fn failed_attach_drop_unregisters_callbacks_without_freeing_a_native_table() {
    native(|state| state.attach_error = 12800008);
    let ime = IME::new(AttachOptions::new(false));
    assert!(ime.try_attach().is_err());
    let raw = native(|state| state.active) as *mut u8;
    drop(ime);
    assert_eq!(native(|state| state.destroyed), 0);
    assert!(proxy::callbacks_for(raw).is_none());
    native(|state| state.active = 0);
    // SAFETY: the mock native owner is released; reclaim the deliberately leaked table.
    unsafe {
        drop(Box::from_raw(raw));
    }
}

#[cfg(feature = "api-26")]
#[test]
fn api_26_consumption_is_configured_without_runtime_feature_probing() {
    let ime = IME::new(AttachOptions::new(false));
    ime.set_consume_key_events(true);
    ime.try_attach().unwrap();
    assert!(native(|state| state.consume));
}
