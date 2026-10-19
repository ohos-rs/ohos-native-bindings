#![allow(
    dead_code,
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types
)]
extern crate self as ohos_input_method_sys;
use std::sync::{Condvar, Mutex, MutexGuard};
use std::time::Duration;
pub const InputMethod_ErrorCode_IME_ERR_DETACHED: u32 = 12800009;
pub const InputMethod_ErrorCode_IME_ERR_NULL_POINTER: u32 = 12802000;
pub const InputMethod_ErrorCode_IME_ERR_IMCLIENT: u32 = 12800003;
pub type InputMethod_InputMethodProxy = u8;
pub type InputMethod_TextEditorProxy = u8;
#[derive(Clone, Copy)]
pub enum EnterKey {
    Unspecified,
    Newline,
}
impl From<EnterKey> for u32 {
    fn from(value: EnterKey) -> Self {
        match value {
            EnterKey::Unspecified => 0,
            EnterKey::Newline => 1,
        }
    }
}
#[derive(Clone, Copy)]
pub enum InputType {
    Text,
    Multiline,
}
impl From<InputType> for u32 {
    fn from(value: InputType) -> Self {
        match value {
            InputType::Text => 0,
            InputType::Multiline => 1,
        }
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
pub struct TextConfig;
impl TextConfig {
    fn set_window_id(&self, _: i32) {}
    fn set_enter_key(&self, _: EnterKey) {}
    fn set_input_type(&self, _: InputType) {}
    fn set_preview_text_supported(&self, _: bool) {}
    fn set_selection(&self, _: Selection) {}
    fn set_consume_key_events(&self, _: bool) -> ImeResult<bool> {
        Ok(true)
    }
    fn supports_key_event_configuration() -> bool {
        true
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
    fn is_showing_keyboard(&self) -> bool {
        self.raw
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
        let mut state = NATIVE.lock().expect("native state");
        assert_ne!(
            state.active, self.raw as usize,
            "freed an editor still retained by native code"
        );
        state.destroyed.push(self.raw as usize);
        unsafe {
            drop(Box::from_raw(self.raw));
        }
    }
}
#[derive(Default)]
struct Native {
    active: usize,
    attaches: usize,
    detaches: usize,
    shows: usize,
    block: bool,
    entered: bool,
    release: bool,
    fail: bool,
    destroyed: Vec<usize>,
    configuration: Option<(u32, u32)>,
    cursor_notifications: usize,
    selection_notifications: Vec<(String, i32, i32)>,
    selection_error: u32,
}
static NATIVE: Mutex<Native> = Mutex::new(Native {
    active: 0,
    attaches: 0,
    detaches: 0,
    shows: 0,
    block: false,
    entered: false,
    release: false,
    fail: false,
    destroyed: Vec::new(),
    configuration: None,
    cursor_notifications: 0,
    selection_notifications: Vec::new(),
    selection_error: 0,
});
static CHANGED: Condvar = Condvar::new();
static DETACH_CHANGED: Condvar = Condvar::new();
static DETACH_CONTROL: Mutex<(bool, bool, bool)> = Mutex::new((false, false, false));
static DETACH_THREAD: Mutex<Option<std::thread::ThreadId>> = Mutex::new(None);
unsafe fn OH_InputMethodController_Attach(editor: *mut u8, _: bool, raw: *mut *mut u8) -> u32 {
    let mut state = NATIVE.lock().expect("native state");
    state.attaches += 1;
    state.entered = true;
    CHANGED.notify_all();
    while state.block && !state.release {
        state = CHANGED.wait(state).expect("release attach");
    }
    state.active = editor as usize;
    if state.fail {
        return 12800008;
    }
    *raw = editor;
    0
}
unsafe fn OH_InputMethodController_Detach(raw: *mut u8) -> u32 {
    *DETACH_THREAD.lock().expect("detach thread") = Some(std::thread::current().id());
    let mut control = DETACH_CONTROL.lock().expect("detach control");
    control.1 = true;
    DETACH_CHANGED.notify_all();
    while control.0 && !control.2 {
        control = DETACH_CHANGED.wait(control).expect("release detach");
    }
    drop(control);
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, raw as usize);
    state.active = 0;
    state.detaches += 1;
    0
}
unsafe fn OH_InputMethodProxy_ShowKeyboard(raw: *mut u8) -> u32 {
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, raw as usize);
    state.shows += 1;
    0
}
unsafe fn OH_InputMethodProxy_HideKeyboard(_: *mut u8) -> u32 {
    0
}
unsafe fn OH_InputMethodProxy_NotifyConfigurationChange(
    raw: *mut u8,
    enter_key: u32,
    input_type: u32,
) -> u32 {
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, raw as usize);
    state.configuration = Some((enter_key, input_type));
    0
}
unsafe fn OH_InputMethodProxy_NotifyCursorUpdate(raw: *mut u8, _: *mut u8) -> u32 {
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, raw as usize);
    state.cursor_notifications += 1;
    0
}
unsafe fn OH_InputMethodProxy_NotifySelectionChange(
    raw: *mut u8,
    text: *mut u16,
    length: usize,
    start: i32,
    end: i32,
) -> u32 {
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, raw as usize);
    if state.selection_error != 0 {
        return state.selection_error;
    }
    let text = String::from_utf16_lossy(std::slice::from_raw_parts(text, length));
    state.selection_notifications.push((text, start, end));
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
#[path = "../../src/session.rs"]
mod session;
pub use ime::*;
static TEST_LOCK: Mutex<()> = Mutex::new(());
static ATTACHMENT_WORKERS: Mutex<Vec<std::thread::JoinHandle<()>>> = Mutex::new(Vec::new());
struct TestGuard {
    _lock: MutexGuard<'static, ()>,
}
impl Drop for TestGuard {
    fn drop(&mut self) {
        finish_attachments();
    }
}
fn finish_attachments() {
    let workers = std::mem::take(&mut *ATTACHMENT_WORKERS.lock().expect("workers"));
    for worker in workers {
        worker.join().expect("attachment worker");
    }
}
fn run_attachment(attachment: ImeAttachment, show: bool) -> ImeResult<bool> {
    let (sender, receiver) = std::sync::mpsc::channel();
    let worker = std::thread::spawn(move || {
        attachment.run(show, |result| {
            sender.send(result).expect("attachment result receiver");
        })
    });
    ATTACHMENT_WORKERS.lock().expect("workers").push(worker);
    receiver.recv().expect("attachment result")
}
fn reset() -> TestGuard {
    let test = TEST_LOCK.lock().expect("test lock");
    *DETACH_CONTROL.lock().expect("detach control") = (false, false, false);
    *DETACH_THREAD.lock().expect("detach thread") = None;
    let mut state = NATIVE.lock().expect("native state");
    assert_eq!(state.active, 0);
    *state = Native::default();
    TestGuard { _lock: test }
}
fn editor() -> IME {
    IME::new_for_window(AttachOptions::new(false), 7)
}
fn block_attach() {
    NATIVE.lock().expect("native state").block = true;
}
fn wait_entered() {
    let state = NATIVE.lock().expect("native state");
    let (state, timeout) = CHANGED
        .wait_timeout_while(state, Duration::from_secs(5), |state| !state.entered)
        .expect("attach wait");
    assert!(state.entered && !timeout.timed_out());
}
fn release_attach() {
    NATIVE.lock().expect("native state").release = true;
    CHANGED.notify_all();
}
#[test]
fn cancellation_before_run_never_attaches() {
    let _test = reset();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    drop(cancel);
    assert!(!run_attachment(task, true).expect("cancelled"));
    assert_eq!(NATIVE.lock().expect("state").attaches, 0);
}
#[test]
fn committed_connection_survives_handle_drop() {
    let _test = reset();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    assert!(std::thread::spawn(move || run_attachment(task, true))
        .join()
        .expect("worker")
        .expect("attach"));
    assert!(cancel.commit());
    drop(cancel);
    assert_eq!(NATIVE.lock().expect("state").shows, 1);
    ime.try_detach().expect("detach");
}
#[test]
fn cancel_in_flight_cleans_up_without_show() {
    let _test = reset();
    block_attach();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    let worker = std::thread::spawn(move || run_attachment(task, true));
    wait_entered();
    cancel.cancel();
    release_attach();
    assert!(!worker.join().expect("worker").expect("cancelled"));
    assert_eq!(NATIVE.lock().expect("state").shows, 0);
    assert_eq!(NATIVE.lock().expect("state").active, 0);
}
#[test]
fn newer_request_waits_for_cancelled_request_cleanup() {
    let _test = reset();
    block_attach();
    let ime = editor();
    let (first, first_cancel) = ime.prepare_background_attachment().expect("prepare");
    let first_worker = std::thread::spawn(move || run_attachment(first, true));
    wait_entered();
    let (second, second_cancel) = ime
        .prepare_background_attachment()
        .expect("prepare replacement");
    let second_worker = std::thread::spawn(move || run_attachment(second, true));
    release_attach();
    assert!(!first_worker.join().expect("first").expect("cancelled"));
    assert!(second_worker.join().expect("second").expect("attach"));
    assert!(second_cancel.commit());
    drop(first_cancel);
    assert_ne!(NATIVE.lock().expect("state").active, 0);
    assert_eq!(NATIVE.lock().expect("state").shows, 1);
    ime.try_detach().expect("detach");
}
#[test]
fn dropping_window_during_attach_keeps_native_editor_alive() {
    let _test = reset();
    block_attach();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    let worker = std::thread::spawn(move || run_attachment(task, true));
    wait_entered();
    drop(ime);
    drop(cancel);
    assert!(NATIVE.lock().expect("state").destroyed.is_empty());
    release_attach();
    assert!(!worker.join().expect("worker").expect("cancelled"));
    assert_eq!(NATIVE.lock().expect("state").active, 0);
    assert_eq!(NATIVE.lock().expect("state").destroyed.len(), 1);
}
#[test]
fn text_queries_observe_updates_during_blocked_attach() {
    let _test = reset();
    block_attach();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    let worker = std::thread::spawn(move || run_attachment(task, false));
    wait_entered();
    assert!(ime
        .try_update_text_state(TextState {
            text: "new text".into(),
            offset: 0,
            selection: 8..8,
            reversed: false
        })
        .is_err());
    release_attach();
    assert!(worker.join().expect("worker").expect("attach"));
    assert!(cancel.commit());
    let pointer = NATIVE.lock().expect("state").active as *mut u8;
    let callbacks = proxy::callbacks_for(pointer).expect("callbacks");
    let query = callbacks
        .read()
        .expect("callbacks")
        .get_left_text_of_cursor
        .clone()
        .expect("query");
    assert_eq!(query(8), "new text");
    ime.try_detach().expect("detach");
}

#[test]
fn selection_notification_retries_unchanged_snapshot_after_busy_session() -> ImeResult<()> {
    let _test = reset();
    let ime = editor();
    ime.try_attach()?;
    ime.try_update_text_state(TextState::default())?;
    let changed = TextState {
        text: "next".into(),
        selection: 4..4,
        ..Default::default()
    };
    let busy_session = session::NativeSessions::acquire("test-busy")?;
    assert!(ime.try_update_text_state(changed.clone()).is_err());
    drop(busy_session);
    ime.try_update_text_state(changed.clone())?;
    ime.try_update_text_state(changed)?;
    let notifications = NATIVE
        .lock()
        .expect("native state")
        .selection_notifications
        .clone();
    assert_eq!(
        notifications,
        vec![(String::new(), 0, 0), ("next".into(), 4, 4)]
    );
    ime.try_detach()
}

#[test]
fn selection_notification_retries_unchanged_snapshot_after_native_failure() -> ImeResult<()> {
    let _test = reset();
    let ime = editor();
    ime.try_attach()?;
    ime.try_update_text_state(TextState::default())?;
    let changed = TextState {
        text: "中文".into(),
        selection: 2..2,
        ..Default::default()
    };
    NATIVE.lock().expect("native state").selection_error = InputMethod_ErrorCode_IME_ERR_IMCLIENT;
    assert!(ime.try_update_text_state(changed.clone()).is_err());
    NATIVE.lock().expect("native state").selection_error = 0;
    ime.try_update_text_state(changed)?;
    let notifications = NATIVE
        .lock()
        .expect("native state")
        .selection_notifications
        .clone();
    assert_eq!(
        notifications,
        vec![(String::new(), 0, 0), ("中文".into(), 2, 2)]
    );
    ime.try_detach()
}
#[test]
fn old_window_cancellation_does_not_detach_new_window() {
    let _test = reset();
    let first = editor();
    let (task, cancel) = first.prepare_background_attachment().expect("prepare");
    assert!(run_attachment(task, false).expect("attach"));
    let second = editor();
    second.try_attach().expect("second attach");
    let active = NATIVE.lock().expect("state").active;
    drop(cancel);
    assert_eq!(NATIVE.lock().expect("state").active, active);
    second.try_detach().expect("detach");
}
#[test]
fn failed_attach_retains_callback_allocation_until_next_transition() {
    let _test = reset();
    NATIVE.lock().expect("state").fail = true;
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    assert!(run_attachment(task, false).is_err());
    drop(cancel);
    drop(ime);
    assert!(NATIVE.lock().expect("state").destroyed.is_empty());
    NATIVE.lock().expect("state").fail = false;
    let next = editor();
    next.try_attach().expect("next attach");
    assert_eq!(NATIVE.lock().expect("state").destroyed.len(), 1);
    next.try_detach().expect("detach");
}
#[test]
fn canceled_completed_result_cannot_commit() {
    let _test = reset();
    let ime = editor();
    let (task, cancel) = ime.prepare_background_attachment().expect("prepare");
    assert!(run_attachment(task, true).expect("attach"));
    cancel.cancel();
    finish_attachments();
    assert!(!cancel.commit());
    assert_eq!(NATIVE.lock().expect("state").active, 0);
}
#[cfg(feature = "api-22")]
#[test]
fn main_thread_callbacks_cannot_move_to_background() {
    let _test = reset();
    let ime = IME::new_with_main_thread_callbacks(AttachOptions::new(false));
    assert!(ime.prepare_background_attachment().is_err());
}

#[test]
fn accepted_connection_refreshes_attributes_changed_during_attach() {
    let _test = reset();
    block_attach();
    let ime = editor();
    let (task, cancellation) = ime.prepare_background_attachment().expect("prepare");
    let worker = std::thread::spawn(move || run_attachment(task, false));
    wait_entered();
    assert!(ime
        .try_update_configuration(EnterKey::Newline, InputType::Multiline)
        .is_err());
    assert!(ime.try_update_cursor(Rect).is_err());
    release_attach();
    assert!(worker.join().expect("worker").expect("attach"));
    assert!(cancellation.commit());
    assert_eq!(NATIVE.lock().expect("native state").configuration, None);
    ime.try_refresh_configuration().expect("refresh");
    assert_eq!(
        NATIVE.lock().expect("native state").configuration,
        Some((1, 1))
    );
    assert_eq!(NATIVE.lock().expect("native state").cursor_notifications, 1);
    let second = editor();
    second.try_attach().expect("replacement");
    ime.try_refresh_configuration().expect("inactive refresh");
    assert_eq!(NATIVE.lock().expect("native state").cursor_notifications, 1);
    second.try_detach().expect("detach");
}

#[test]
fn completed_attachment_cancellation_keeps_blocking_detach_on_its_worker() {
    let _test = reset();
    let caller = std::thread::current().id();
    let ime = editor();
    let (attachment, cancellation) = ime.prepare_background_attachment().expect("prepare");
    assert!(run_attachment(attachment, false).expect("attached"));
    DETACH_CONTROL.lock().expect("detach control").0 = true;
    cancellation.cancel();
    let control = DETACH_CONTROL.lock().expect("detach control");
    let (mut control, timeout) = DETACH_CHANGED
        .wait_timeout_while(control, Duration::from_secs(5), |control| !control.1)
        .expect("detach entered");
    let entered = control.1 && !timeout.timed_out();
    control.2 = true;
    DETACH_CHANGED.notify_all();
    drop(control);
    finish_attachments();
    assert!(entered);
    assert_ne!(*DETACH_THREAD.lock().expect("detach thread"), Some(caller));
    assert!(!cancellation.commit());
    assert_eq!(NATIVE.lock().expect("native state").active, 0);
}
