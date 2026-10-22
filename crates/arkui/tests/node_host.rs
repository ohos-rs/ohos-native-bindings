//! Run production node ownership and registry tests with Rust native API substitutes.
// This harness targets the baseline API without optional native capabilities.
#![cfg(not(any(feature = "napi", feature = "xcomponent", feature = "api-13")))]
#![allow(dead_code, non_snake_case, non_camel_case_types)]
extern crate self as ohos_arkui_input_binding;
extern crate self as ohos_arkui_sys;

use std::{cell::RefCell, ffi::c_void};
pub mod sys {
    pub type ArkUI_NodeHandle = *mut std::ffi::c_void;
}
#[derive(Debug)]
pub enum ArkUIErrorCode {
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
#[derive(Clone, Copy)]
pub enum ArkUINodeType {
    Custom,
}
#[derive(Clone, Default)]
pub struct EventHandle {
    callbacks: Vec<(u32, ())>,
}
pub struct NodeDirtyFlag;
pub trait ArkUIAttributeBasic {
    fn raw(&self) -> &ArkUINode;
    fn borrow_mut(&mut self) -> &mut ArkUINode;
}
pub trait ArkUICommonAttribute: ArkUIAttributeBasic {}
pub trait ArkUIEvent {}
pub trait ArkUIGesture {}
pub mod types {
    pub mod advanced {
        pub struct LengthMetricUnit;
    }
}
#[macro_export]
macro_rules! check_arkui_status {
    ($status:expr) => {{
        let _ = $status;
        Ok::<(), $crate::ArkUIError>(())
    }};
}
pub struct ArkUI_IntOffset {
    pub x: i32,
    pub y: i32,
}
pub struct ArkUI_IntSize {
    pub width: i32,
    pub height: i32,
}
unsafe fn OH_ArkUI_GetContextByNode(_: sys::ArkUI_NodeHandle) -> *mut c_void {
    std::ptr::null_mut()
}
unsafe fn OH_ArkUI_NodeUtils_GetLayoutSize(_: sys::ArkUI_NodeHandle, _: *mut ArkUI_IntSize) {}
unsafe fn OH_ArkUI_NodeUtils_GetLayoutPositionInScreen(
    _: sys::ArkUI_NodeHandle,
    _: *mut ArkUI_IntOffset,
) {
}
unsafe fn OH_ArkUI_NodeUtils_GetLayoutPosition(_: sys::ArkUI_NodeHandle, _: *mut ArkUI_IntOffset) {}
unsafe fn OH_ArkUI_NodeUtils_GetLayoutPositionInWindow(
    _: sys::ArkUI_NodeHandle,
    _: *mut ArkUI_IntOffset,
) {
}
unsafe fn OH_ArkUI_NodeUtils_GetPositionWithTranslateInWindow(
    _: sys::ArkUI_NodeHandle,
    _: *mut ArkUI_IntOffset,
) {
}

pub mod animate {
    pub mod options {
        use super::super::*;
        pub struct Context;
        impl Context {
            pub fn raw(&self) -> *mut c_void {
                std::ptr::null_mut()
            }
        }
        pub struct Animation {
            pub update_ctx: RefCell<Context>,
            pub finish_ctx: RefCell<Context>,
        }
        impl Animation {
            pub fn raw(&self) -> *mut c_void {
                std::ptr::null_mut()
            }
        }
    }
}
pub mod api {
    pub mod node_custom_event {
        pub struct IntOffset;
        pub struct IntSize;
        impl From<crate::ArkUI_IntOffset> for IntOffset {
            fn from(_: crate::ArkUI_IntOffset) -> Self {
                Self
            }
        }
        impl From<crate::ArkUI_IntSize> for IntSize {
            fn from(_: crate::ArkUI_IntSize) -> Self {
                Self
            }
        }
    }
    pub(crate) static ARK_UI_NATIVE_ANIMATE_API_1: crate::ApiSlot = crate::ApiSlot;
}
struct Api;
impl Api {
    fn add_event_receiver(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn remove_event_receiver(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn get_parent(&self, _: &ArkUINode) -> ArkUIResult<Option<sys::ArkUI_NodeHandle>> {
        Ok(None)
    }
    fn remove_child(&self, _: &ArkUINode, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn unregister_node_event(&self, _: &ArkUINode, _: u32) -> ArkUIResult<()> {
        Ok(())
    }
    fn dispose(&self, _: &ArkUINode) -> ArkUIResult<()> {
        Ok(())
    }
    fn set_length_metric_unit(
        &self,
        _: &ArkUINode,
        _: types::advanced::LengthMetricUnit,
    ) -> ArkUIResult<()> {
        Ok(())
    }
    fn mark_dirty(&self, _: &ArkUINode, _: NodeDirtyFlag) -> ArkUIResult<()> {
        Ok(())
    }
    fn animate_to(
        &self,
        _: *mut c_void,
        _: *mut c_void,
        _: *mut c_void,
        _: *mut c_void,
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
static ARK_UI_NATIVE_NODE_API_1: ApiSlot = ApiSlot;
#[allow(unused_imports)]
#[path = "../src/common/node.rs"]
pub mod node;
#[path = "../src/common/node_registry.rs"]
pub mod node_registry;
pub mod common {
    pub use crate::{node, node_registry, ArkUIResult};
}
pub use node::ArkUINode;

#[test]
fn disposal_succeeds_after_the_extra_rc_owner_retires() {
    let node = ArkUINode {
        raw: std::ptr::dangling_mut(),
        owns_raw: true,
        ..Default::default()
    }
    .into_rc();
    let other_owner = node.clone();
    assert!(node.borrow_mut().dispose().is_err());
    assert!(node.borrow().is_owned());
    drop(other_owner);
    node.borrow_mut().dispose().unwrap();
    assert!(node.borrow().raw_handle().is_null());
    assert!(!node.borrow().is_owned());
}

#[test]
fn registry_binds_a_callers_rc_without_retaining_it() {
    let node = std::rc::Rc::new(RefCell::new(ArkUINode {
        raw: std::ptr::dangling_mut(),
        owns_raw: true,
        ..Default::default()
    }));
    let view = node.borrow().borrowed_clone();
    node_registry::register(&view, node.clone()).unwrap();
    assert_eq!(node.borrow().self_weak.strong_count(), 1);
    let other_owner = node.clone();
    assert!(node.borrow_mut().dispose().is_err());
    drop(other_owner);
    node.borrow_mut().dispose().unwrap();
}
