//! Production dialog ownership with deterministic native substitutes.
#![allow(dead_code, non_camel_case_types)]
extern crate self as log;
extern crate self as ohos_arkui_sys;
#[macro_export]
macro_rules! error {
    ($($args:tt)*) => { eprintln!($($args)*); }
}

use std::{
    cell::RefCell,
    ffi::c_void,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
};
pub type ArkUI_NativeDialogHandle = *mut c_void;
#[derive(Clone, Copy)]
pub struct Alignment;
#[derive(Clone, Copy)]
pub struct DismissReason;
#[derive(Debug)]
pub struct ArkUIError;
pub type ArkUIResult<T> = Result<T, ArkUIError>;
mod arkui_input_binding {
    pub enum ArkUIErrorCode {
        ParamInvalid,
    }
}
impl ArkUIError {
    fn new(_: arkui_input_binding::ArkUIErrorCode, _: &str) -> Self {
        Self
    }
}
#[path = "../src/common/node_retention.rs"]
mod node_retention;
mod common {
    pub(crate) mod node_retention {
        pub(crate) use crate::node_retention::NodeRetention;
    }
}

pub struct ArkUINode {
    retained: node_retention::NativeRetentions,
    disposed: bool,
}
static NODE_DISPOSALS: AtomicUsize = AtomicUsize::new(0);
impl ArkUINode {
    fn new() -> Self {
        Self {
            retained: Default::default(),
            disposed: false,
        }
    }
    fn raw(&self) -> *mut c_void {
        std::ptr::dangling_mut()
    }
    fn is_owned(&self) -> bool {
        !self.disposed
    }
    fn retain_native(&self) -> ArkUIResult<node_retention::NodeRetention> {
        self.retained.retain().ok_or(ArkUIError)
    }
    fn dispose(&mut self) -> ArkUIResult<()> {
        if self.retained.is_retained() {
            return Err(ArkUIError);
        }
        self.disposed = true;
        NODE_DISPOSALS.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
}

static FAIL_REMOVE: AtomicBool = AtomicBool::new(false);
static REMOVED: AtomicUsize = AtomicUsize::new(0);
static TEST_LOCK: Mutex<()> = Mutex::new(());
struct Api;
impl Api {
    fn create(&self) -> ArkUIResult<ArkUI_NativeDialogHandle> {
        Ok(std::ptr::dangling_mut())
    }
    fn set_content(&self, _: ArkUI_NativeDialogHandle, _: *mut c_void) -> ArkUIResult<()> {
        Ok(())
    }
    fn remove_content(&self, _: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        if FAIL_REMOVE.load(Ordering::SeqCst) {
            return Err(ArkUIError);
        }
        REMOVED.fetch_add(1, Ordering::SeqCst);
        Ok(())
    }
    fn register_dismiss(&self, _: ArkUI_NativeDialogHandle, _: *mut c_void) -> ArkUIResult<()> {
        Ok(())
    }
    fn dispose(&self, _: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        Ok(())
    }
    fn close(&self, _: ArkUI_NativeDialogHandle) -> ArkUIResult<()> {
        Ok(())
    }
    fn show(&self, _: ArkUI_NativeDialogHandle, _: bool) -> ArkUIResult<()> {
        Ok(())
    }
    fn custom_style(&self, _: ArkUI_NativeDialogHandle, _: bool) -> ArkUIResult<()> {
        Ok(())
    }
    fn custom_animation(&self, _: ArkUI_NativeDialogHandle, _: bool) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_modal_mode(&self, _: ArkUI_NativeDialogHandle, _: bool) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_auto_cancel(&self, _: ArkUI_NativeDialogHandle, _: bool) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_background_color(&self, _: ArkUI_NativeDialogHandle, _: u32) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_content_alignment(
        &self,
        _: ArkUI_NativeDialogHandle,
        _: Alignment,
        _: f32,
        _: f32,
    ) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_corner_radius(
        &self,
        _: ArkUI_NativeDialogHandle,
        _: f32,
        _: f32,
        _: f32,
        _: f32,
    ) -> ArkUIResult<()> {
        Ok(())
    }
}
struct ApiSlot;
impl ApiSlot {
    fn with<T>(&self, use_api: impl FnOnce(&Api) -> T) -> T {
        use_api(&Api)
    }
}
mod api {
    pub(crate) static ARK_UI_NATIVE_DIALOG_API_1: crate::ApiSlot = crate::ApiSlot;
}
#[path = "../src/dialog/mod.rs"]
mod dialog;

#[test]
fn shared_content_stays_owned_until_successful_native_removal() {
    let _serial = TEST_LOCK.lock().unwrap();
    REMOVED.store(0, Ordering::SeqCst);
    let node = Rc::new(RefCell::new(ArkUINode::new()));
    let dialog = dialog::Dialog::new().unwrap();
    dialog.content_node(&node).unwrap();
    assert!(node.borrow_mut().dispose().is_err());
    FAIL_REMOVE.store(true, Ordering::SeqCst);
    assert!(dialog.remove_content().is_err());
    assert!(node.borrow_mut().dispose().is_err());
    FAIL_REMOVE.store(false, Ordering::SeqCst);
    dialog.remove_content().unwrap();
    dialog.remove_content().unwrap();
    assert_eq!(REMOVED.load(Ordering::SeqCst), 1);
    node.borrow_mut().dispose().unwrap();
}

#[test]
fn transferred_content_disposes_after_its_dialog_edge_retires() {
    let _serial = TEST_LOCK.lock().unwrap();
    let before = NODE_DISPOSALS.load(Ordering::SeqCst);
    let dialog = dialog::Dialog::new().unwrap();
    dialog.content(ArkUINode::new()).unwrap();
    dialog.remove_content().unwrap();
    assert_eq!(NODE_DISPOSALS.load(Ordering::SeqCst), before + 1);
}

#[test]
fn failed_drop_keeps_the_native_contents_guard_quarantined() {
    let _serial = TEST_LOCK.lock().unwrap();
    let node = Rc::new(RefCell::new(ArkUINode::new()));
    let dialog = dialog::Dialog::new().unwrap();
    dialog.content_node(&node).unwrap();
    FAIL_REMOVE.store(true, Ordering::SeqCst);
    drop(dialog);
    FAIL_REMOVE.store(false, Ordering::SeqCst);
    assert!(node.borrow_mut().dispose().is_err());
}
