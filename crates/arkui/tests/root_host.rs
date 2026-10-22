//! Run the production root lifecycle with deterministic native substitutes.
#![allow(dead_code, non_upper_case_globals)]

extern crate self as log;
extern crate self as ohos_arkui_input_binding;
#[macro_export]
macro_rules! error {
    ($($args:tt)*) => { eprintln!($($args)*); }
}

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

pub struct ArkUINode {
    owns_raw: bool,
    self_weak: std::rc::Weak<RefCell<ArkUINode>>,
    disposed: bool,
}
impl ArkUINode {
    fn new() -> Self {
        Self {
            owns_raw: true,
            self_weak: Default::default(),
            disposed: false,
        }
    }
    fn into_rc(self) -> Rc<RefCell<Self>> {
        Rc::new_cyclic(|weak| {
            let mut node = self;
            node.self_weak = weak.clone();
            RefCell::new(node)
        })
    }
    fn bind_rc(rc: &Rc<RefCell<Self>>) {
        if rc.borrow().self_weak.strong_count() == 0 {
            rc.borrow_mut().self_weak = Rc::downgrade(rc);
        }
    }
    fn raw_handle(&self) -> *mut std::ffi::c_void {
        if self.disposed {
            std::ptr::null_mut()
        } else {
            std::ptr::dangling_mut()
        }
    }
    fn dispose(&mut self) -> ArkUIResult<()> {
        if self.self_weak.strong_count() > 1 {
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
    let node = ArkUINode::new().into_rc();
    let other_owner = node.clone();
    let weak = Rc::downgrade(&node);
    let mut root = root::RootNode::new(ArkUIHandle);
    root.mount_node(node).unwrap();
    assert_eq!(other_owner.borrow().self_weak.strong_count(), 2);
    assert!(root.unmount().is_err());
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert!(!other_owner.borrow().disposed);
    drop(other_owner);
    root.unmount().unwrap();
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert!(weak.upgrade().is_none());
}

#[test]
fn dropping_a_root_retains_its_rc_when_the_native_edge_survives() {
    let _serial = TEST_LOCK.lock().unwrap();
    REMOVALS.store(0, Ordering::SeqCst);
    // Exercise binding of a caller-created Rc as well as into_rc above.
    let node = Rc::new(RefCell::new(ArkUINode::new()));
    let mut root = root::RootNode::new(ArkUIHandle);
    root.mount_node(node.clone()).unwrap();
    FAIL_REMOVAL.store(true, Ordering::SeqCst);
    drop(root);
    FAIL_REMOVAL.store(false, Ordering::SeqCst);
    assert_eq!(REMOVALS.load(Ordering::SeqCst), 1);
    assert_eq!(Rc::strong_count(&node), 2);
    assert!(node.borrow_mut().dispose().is_err());
}
