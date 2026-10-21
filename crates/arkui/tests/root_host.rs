//! Run the production root lifecycle with deterministic native substitutes.
#![allow(dead_code, non_upper_case_globals)]

extern crate self as ohos_arkui_input_binding;

use std::{
    cell::RefCell,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicUsize, Ordering},
        Mutex,
    },
};

#[derive(Debug)]
pub enum ArkUIErrorCode {
    ChildNodeExist,
    ParamInvalid,
}
#[derive(Debug)]
pub struct ArkUIError;
impl ArkUIError {
    fn new(_: ArkUIErrorCode, _: &str) -> Self {
        Self
    }
}
pub type ArkUIResult<T> = Result<T, ArkUIError>;

#[path = "../src/common/node_retention.rs"]
mod node_retention;

pub struct ArkUINode {
    owns_raw: bool,
    retained: node_retention::NativeRetentions,
    disposed: bool,
}
impl ArkUINode {
    fn retain_native(&self) -> ArkUIResult<node_retention::NodeRetention> {
        self.retained.retain().ok_or(ArkUIError)
    }
    fn dispose(&mut self) -> ArkUIResult<()> {
        if self.retained.is_retained() {
            return Err(ArkUIError);
        }
        self.disposed = true;
        Ok(())
    }
}

pub struct ArkUIHandle;
static REMOVALS: AtomicUsize = AtomicUsize::new(0);
static FAIL_REMOVAL: AtomicBool = AtomicBool::new(false);
static TEST_LOCK: Mutex<()> = Mutex::new(());
impl ArkUIHandle {
    fn add_node(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn remove_node(&self, _: &ArkUINode) -> ArkUIResult<()> {
        if FAIL_REMOVAL.load(Ordering::SeqCst) {
            REMOVALS.fetch_add(1, Ordering::SeqCst);
            return Err(ArkUIError);
        }
        // Repeating a successful native removal is an error, as on the device.
        if REMOVALS.fetch_add(1, Ordering::SeqCst) != 0 {
            return Err(ArkUIError);
        }
        Ok(())
    }
}
struct Api;
impl Api {
    fn add_event_receiver(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn remove_event_receiver(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
}
struct ApiSlot;
impl ApiSlot {
    fn with<T>(&self, use_api: impl FnOnce(&Api) -> T) -> T {
        use_api(&Api)
    }
}
static ARK_UI_NATIVE_NODE_API_1: ApiSlot = ApiSlot;
mod common {
    pub(crate) mod node_retention {
        pub(crate) use crate::node_retention::NodeRetention;
    }
    pub(crate) mod node_registry {
        pub(crate) fn register(
            _: &crate::ArkUINode,
            _: std::rc::Rc<std::cell::RefCell<crate::ArkUINode>>,
        ) -> crate::ArkUIResult<()> {
            Ok(())
        }
        pub(crate) fn unregister(_: &crate::ArkUINode) {}
    }
}
#[path = "../src/component/root.rs"]
mod root;

#[test]
fn detached_roots_retry_disposal_without_repeating_native_removal() {
    let _serial = TEST_LOCK.lock().unwrap();
    REMOVALS.store(0, Ordering::SeqCst);
    let node = Rc::new(RefCell::new(ArkUINode {
        owns_raw: true,
        retained: Default::default(),
        disposed: false,
    }));
    let other_owner = node.borrow().retain_native().unwrap();
    let mut root = root::RootNode::new(ArkUIHandle);
    root.mount_node(node.clone()).unwrap();
    assert!(root.unmount().is_err());
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert!(!node.borrow().disposed);
    drop(other_owner);
    root.unmount().unwrap();
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert!(node.borrow().disposed);
}

#[test]
fn dropping_a_root_quarantines_its_guard_when_the_native_edge_survives() {
    let _serial = TEST_LOCK.lock().unwrap();
    REMOVALS.store(0, Ordering::SeqCst);
    let node = Rc::new(RefCell::new(ArkUINode {
        owns_raw: true,
        retained: Default::default(),
        disposed: false,
    }));
    let mut root = root::RootNode::new(ArkUIHandle);
    root.mount_node(node.clone()).unwrap();
    FAIL_REMOVAL.store(true, Ordering::SeqCst);
    drop(root);
    FAIL_REMOVAL.store(false, Ordering::SeqCst);
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert!(node.borrow().retained.is_retained());
    assert!(node.borrow_mut().dispose().is_err());
}
