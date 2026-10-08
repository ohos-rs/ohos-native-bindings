#![cfg(not(target_env = "ohos"))]

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, Mutex, MutexGuard};

use ohos_ime_binding::{AttachOptions, EnterKey, InputType, IME};

unsafe extern "C" {
    fn ime_mock_reset();
    fn ime_mock_fail(operation: u32, code: u32);
    fn ime_mock_null_attach();
    fn ime_mock_stop() -> u32;
    fn ime_mock_count(counter: u32) -> u32;
}

const NOT_EDITABLE: u32 = 12800016;
const DETACHED: u32 = 12800009;
const SERVICE_ERROR: u32 = 12800004;
static TEST_LOCK: Mutex<()> = Mutex::new(());

struct NativeFixture(MutexGuard<'static, ()>);

impl NativeFixture {
    fn new() -> Self {
        let guard = TEST_LOCK.lock().unwrap_or_else(|error| error.into_inner());
        // Finish any retained failed session from the preceding case.
        unsafe { ime_mock_reset() };
        let ime = IME::new(AttachOptions::new(false));
        ime.try_attach().unwrap();
        ime.try_detach().unwrap();
        unsafe { ime_mock_reset() };
        Self(guard)
    }

    fn fail(&self, operation: u32, code: u32) {
        unsafe { ime_mock_fail(operation, code) };
    }

    fn count(&self, counter: u32) -> u32 {
        unsafe { ime_mock_count(counter) }
    }

    fn stop(&self) {
        assert_eq!(
            unsafe { ime_mock_stop() },
            0,
            "native listener used a destroyed editor"
        );
    }

    fn assert_safe(&self) {
        let _guard = &self.0;
        assert_eq!(
            self.count(2),
            0,
            "destroyed an editor still retained by the NDK"
        );
        assert_eq!(self.count(3), 0, "used an invalidated native proxy");
    }
}

fn editor() -> IME {
    IME::new(AttachOptions::new(false))
}

#[test]
fn delayed_stop_after_hide_error_keeps_the_editor_alive_until_detach() {
    let native = NativeFixture::new();
    for error in [NOT_EDITABLE, DETACHED] {
        let ime = editor();
        let callbacks = Arc::new(AtomicUsize::new(0));
        let observed = callbacks.clone();
        ime.on_status_change(move |_| {
            observed.fetch_add(1, Ordering::Relaxed);
        });
        ime.try_attach().unwrap();
        native.fail(1, error);
        assert_eq!(ime.try_hide_keyboard().unwrap_err().code(), error);
        native.stop();
        assert_eq!(callbacks.load(Ordering::Relaxed), 1);
        ime.try_detach().unwrap();
    }
    assert_eq!(native.count(0), native.count(1));
    native.assert_safe();
}

#[test]
fn show_recovery_reuses_one_editor_for_repeated_stale_sessions() {
    let native = NativeFixture::new();
    let ime = editor();
    ime.try_attach().unwrap();
    for _ in 0..100 {
        native.fail(0, NOT_EDITABLE);
        ime.try_show_keyboard().unwrap();
        native.stop();
    }
    assert_eq!(native.count(0), 1);
    assert_eq!(native.count(1), 0);
    ime.try_detach().unwrap();
    assert_eq!(native.count(1), 1);
    native.assert_safe();
}

#[test]
fn failed_recovery_keeps_the_published_editor_until_successful_replacement() {
    let native = NativeFixture::new();
    let ime = editor();
    ime.try_attach().unwrap();
    native.fail(0, NOT_EDITABLE);
    native.fail(2, SERVICE_ERROR);
    assert_eq!(ime.try_show_keyboard().unwrap_err().code(), SERVICE_ERROR);
    native.stop();
    drop(ime);
    assert_eq!(native.count(1), 0);
    native.stop();
    let successor = editor();
    successor.try_attach().unwrap();
    assert_eq!(native.count(1), 1);
    successor.try_detach().unwrap();
    native.assert_safe();
}

#[test]
fn failed_detach_disables_rust_callbacks_but_preserves_the_native_table() {
    let native = NativeFixture::new();
    let ime = editor();
    let callbacks = Arc::new(AtomicUsize::new(0));
    let observed = callbacks.clone();
    ime.on_status_change(move |_| {
        observed.fetch_add(1, Ordering::Relaxed);
    });
    ime.try_attach().unwrap();
    native.fail(3, SERVICE_ERROR);
    assert_eq!(ime.try_detach().unwrap_err().code(), SERVICE_ERROR);
    native.stop();
    assert_eq!(callbacks.load(Ordering::Relaxed), 0);
    assert_eq!(native.count(1), 0);
    ime.try_attach().unwrap();
    assert_eq!(native.count(1), 1);
    ime.try_detach().unwrap();
    native.assert_safe();
}

#[test]
fn failed_initial_attach_and_null_success_do_not_free_published_editors() {
    let native = NativeFixture::new();
    for null_success in [false, true] {
        let ime = editor();
        if null_success {
            unsafe { ime_mock_null_attach() };
        } else {
            native.fail(2, SERVICE_ERROR);
        }
        assert!(ime.try_attach().is_err());
        drop(ime);
        native.stop();
    }
    let successor = editor();
    successor.try_attach().unwrap();
    successor.try_detach().unwrap();
    assert_eq!(native.count(0), native.count(1));
    native.assert_safe();
}

#[test]
fn inactive_editor_cannot_hide_or_detach_its_successor() {
    let native = NativeFixture::new();
    let old = editor();
    let successor = editor();
    old.try_attach().unwrap();
    successor.try_attach().unwrap();
    old.try_hide_keyboard().unwrap();
    old.try_detach().unwrap();
    assert_eq!(native.count(5), 0);
    successor.try_show_keyboard().unwrap();
    successor.try_detach().unwrap();
    native.assert_safe();
}

#[test]
fn failed_replacement_invalidates_old_proxy_without_freeing_either_editor() {
    let native = NativeFixture::new();
    let old = editor();
    let successor = editor();
    old.try_attach().unwrap();
    native.fail(2, SERVICE_ERROR);
    assert!(successor.try_attach().is_err());
    old.try_hide_keyboard().unwrap();
    drop(old);
    native.stop();
    successor.try_attach().unwrap();
    successor.try_detach().unwrap();
    assert_eq!(native.count(0), native.count(1));
    native.assert_safe();
}

#[test]
fn stale_notification_and_drop_preserve_lifetime_through_native_stop() {
    let native = NativeFixture::new();
    let ime = editor();
    ime.try_attach().unwrap();
    native.fail(4, NOT_EDITABLE);
    assert!(ime
        .try_update_configuration(EnterKey::Done, InputType::Text)
        .is_err());
    native.stop();
    drop(ime);
    assert_eq!(native.count(1), 1);
    assert_eq!(native.count(5), 1);
    native.assert_safe();
}

#[test]
fn native_callback_reentry_returns_an_error_without_deadlocking() {
    let native = NativeFixture::new();
    let ime = editor();
    let reentries = Arc::new(AtomicUsize::new(0));
    let observed = reentries.clone();
    ime.on_status_change(move |_| {
        let other = editor();
        assert!(other.try_attach().is_err());
        observed.fetch_add(1, Ordering::Relaxed);
    });
    ime.try_attach().unwrap();
    ime.try_detach().unwrap();
    assert_eq!(reentries.load(Ordering::Relaxed), 1);
    native.assert_safe();
}
