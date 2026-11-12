//! Host regression tests exercise `Controller::run_javascript` against a
//! deterministic ArkWeb controller test double. No OHOS library or device is needed:
//! crates/web/tests/run-javascript.sh
#![allow(
    non_snake_case,
    non_camel_case_types,
    non_upper_case_globals,
    dead_code,
    clippy::missing_safety_doc
)]

extern crate self as ohos_web_sys;

#[path = "../../src/error.rs"]
mod error;
pub use error::ArkWebError;
#[path = "../../src/macro.rs"]
mod r#macro;
#[path = "../../src/api/controller.rs"]
mod controller;
pub use controller::Controller;

use std::{
    cell::{Cell, RefCell},
    collections::VecDeque,
    ffi::{c_char, c_void, CStr},
    rc::Rc,
};

pub const ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER: u32 = 1;

pub type ArkWeb_OnJavaScriptCallback = Option<
    unsafe extern "C" fn(
        webTag: *const c_char,
        data: *const ArkWeb_JavaScriptBridgeData,
        userData: *mut c_void,
    ),
>;

#[repr(C)]
pub struct ArkWeb_JavaScriptBridgeData {
    pub buffer: *const u8,
    pub size: usize,
}

#[repr(C)]
pub struct ArkWeb_JavaScriptObject {
    pub buffer: *const u8,
    pub size: usize,
    pub callback: ArkWeb_OnJavaScriptCallback,
    pub userData: *mut c_void,
}

#[repr(C)]
pub struct ArkWeb_ProxyObject {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct ArkWeb_ControllerAPI {
    pub size: usize,
    pub runJavaScript:
        Option<unsafe extern "C" fn(webTag: *const c_char, javascriptObject: *const ArkWeb_JavaScriptObject)>,
    pub registerJavaScriptProxy:
        Option<unsafe extern "C" fn(webTag: *const c_char, proxyObject: *const ArkWeb_ProxyObject)>,
    pub deleteJavaScriptRegister:
        Option<unsafe extern "C" fn(webTag: *const c_char, objName: *const c_char)>,
    pub refresh: Option<unsafe extern "C" fn(webTag: *const c_char)>,
    pub registerAsyncJavaScriptProxy:
        Option<unsafe extern "C" fn(webTag: *const c_char, proxyObject: *const ArkWeb_ProxyObject)>,
    pub createWebMessagePorts:
        Option<unsafe extern "C" fn(webTag: *const c_char, size: *mut usize) -> *mut c_void>,
    pub destroyWebMessagePorts: Option<unsafe extern "C" fn(ports: *mut *mut c_void, size: usize)>,
    pub postWebMessage: Option<
        unsafe extern "C" fn(
            webTag: *const c_char,
            name: *const c_char,
            ports: *mut *mut c_void,
            size: usize,
            url: *const c_char,
        ) -> i32,
    >,
}

struct PendingCall {
    web_tag: String,
    script: Vec<u8>,
    callback: ArkWeb_OnJavaScriptCallback,
    user_data: *mut c_void,
}

struct Harness {
    api: Box<ArkWeb_ControllerAPI>,
    calls: RefCell<VecDeque<PendingCall>>,
}

thread_local! {
    static CURRENT_HARNESS: RefCell<Option<Rc<Harness>>> = const { RefCell::new(None) };
}

impl Harness {
    fn install(size: usize, run_js_present: bool) -> (Rc<Self>, Controller) {
        let harness = Rc::new(Self {
            api: Box::new(ArkWeb_ControllerAPI {
                size,
                runJavaScript: run_js_present.then_some(mock_run_javascript),
                registerJavaScriptProxy: None,
                deleteJavaScriptRegister: None,
                refresh: None,
                registerAsyncJavaScriptProxy: None,
                createWebMessagePorts: None,
                destroyWebMessagePorts: None,
                postWebMessage: None,
            }),
            calls: RefCell::new(VecDeque::new()),
        });
        CURRENT_HARNESS.with(|slot| {
            *slot.borrow_mut() = Some(harness.clone());
        });
        let controller = Controller::new();
        (harness, controller)
    }

    fn complete_next(&self, payload: Option<Option<&[u8]>>) {
        let call = self
            .calls
            .borrow_mut()
            .pop_front()
            .expect("expected pending runJavaScript call");
        let cb = call.callback.expect("callback must be present");
        let tag = std::ffi::CString::new(call.web_tag).unwrap();
        match payload {
            None => unsafe {
                cb(tag.as_ptr(), std::ptr::null(), call.user_data);
            },
            Some(None) => {
                let bridge = ArkWeb_JavaScriptBridgeData {
                    buffer: std::ptr::null(),
                    size: 4,
                };
                unsafe {
                    cb(tag.as_ptr(), &bridge, call.user_data);
                }
            }
            Some(Some(bytes)) => {
                let bridge = ArkWeb_JavaScriptBridgeData {
                    buffer: bytes.as_ptr(),
                    size: bytes.len(),
                };
                unsafe {
                    cb(tag.as_ptr(), &bridge, call.user_data);
                }
            }
        }
    }
}

unsafe extern "C" fn mock_run_javascript(
    web_tag: *const c_char,
    js_object: *const ArkWeb_JavaScriptObject,
) {
    assert!(!web_tag.is_null(), "webTag must be non-null");
    assert!(!js_object.is_null(), "javascriptObject must be non-null");
    let obj = &*js_object;
    assert!(!obj.buffer.is_null(), "script buffer must be non-null");
    // Verify NUL termination past `obj.size`.
    assert_eq!(
        *obj.buffer.add(obj.size),
        0,
        "script buffer must be NUL-terminated at buffer[size]"
    );
    let script = std::slice::from_raw_parts(obj.buffer, obj.size).to_vec();
    let tag = CStr::from_ptr(web_tag).to_str().unwrap().to_owned();
    CURRENT_HARNESS.with(|slot| {
        let harness = slot.borrow();
        harness
            .as_ref()
            .expect("harness installed")
            .calls
            .borrow_mut()
            .push_back(PendingCall {
                web_tag: tag,
                script,
                callback: obj.callback,
                user_data: obj.userData,
            });
    });
}

pub unsafe fn OH_ArkWeb_GetNativeAPI(kind: u32) -> *mut c_void {
    assert_eq!(kind, ArkWeb_NativeAPIVariantKind_ARKWEB_NATIVE_CONTROLLER);
    CURRENT_HARNESS.with(|slot| {
        let borrow = slot.borrow();
        let harness = borrow.as_ref().expect("harness installed");
        (&*harness.api as *const ArkWeb_ControllerAPI)
            .cast_mut()
            .cast::<c_void>()
    })
}

struct DropCounter(Rc<Cell<usize>>);
impl Drop for DropCounter {
    fn drop(&mut self) {
        self.0.set(self.0.get() + 1);
    }
}

#[test]
fn delivers_utf8_empty_and_null_bridge_payloads_and_drops_closure_once() {
    let (harness, controller) =
        Harness::install(std::mem::size_of::<ArkWeb_ControllerAPI>(), true);

    let drops = Rc::new(Cell::new(0));
    let guard = DropCounter(drops.clone());
    let received = Rc::new(RefCell::new(None));
    let slot = received.clone();

    controller
        .run_javascript("web-1", "1 + 1", move |bytes| {
            let _keep = guard;
            *slot.borrow_mut() = Some(bytes.map(<[u8]>::to_vec));
        })
        .unwrap();

    assert_eq!(drops.get(), 0, "closure retained while in flight");
    assert_eq!(harness.calls.borrow().len(), 1);
    assert_eq!(harness.calls.borrow()[0].web_tag, "web-1");
    assert_eq!(harness.calls.borrow()[0].script, b"1 + 1");

    harness.complete_next(Some(Some(b"2")));
    assert_eq!(*received.borrow(), Some(Some(b"2".to_vec())));
    assert_eq!(drops.get(), 1, "closure dropped immediately after completion");

    // Null bridge pointer -> None
    let null_out = Rc::new(RefCell::new(Some(Some(vec![1_u8]))));
    let slot = null_out.clone();
    controller
        .run_javascript("web-1", "void 0", move |bytes| {
            *slot.borrow_mut() = Some(bytes.map(<[u8]>::to_vec));
        })
        .unwrap();
    harness.complete_next(None);
    assert_eq!(*null_out.borrow(), Some(None));

    // Null buffer inside non-null bridge struct -> Some(&[])
    let null_buf_out = Rc::new(RefCell::new(None));
    let slot = null_buf_out.clone();
    controller
        .run_javascript("web-1", "''", move |bytes| {
            *slot.borrow_mut() = Some(bytes.map(<[u8]>::to_vec));
        })
        .unwrap();
    harness.complete_next(Some(None));
    assert_eq!(*null_buf_out.borrow(), Some(Some(vec![])));
}

#[test]
fn supports_zero_sized_closures_and_catches_panics_across_ffi() {
    let (harness, controller) =
        Harness::install(std::mem::size_of::<ArkWeb_ControllerAPI>(), true);

    // Non-capturing ZST closure (`size_of_val(&cb) == 0`) must still execute and clean up safely.
    controller.run_javascript("web-zst", "0", |_| {}).unwrap();
    harness.complete_next(Some(Some(b"0")));

    // Panicking callback must not unwind across `extern "C"`.
    controller
        .run_javascript("web-panic", "throw 1", |_| {
            panic!("boom inside callback");
        })
        .unwrap();
    harness.complete_next(Some(Some(b"null")));
}

#[test]
fn rejects_missing_member_struct_truncation_and_interior_nul_web_tag() {
    let (_harness, missing_fn) =
        Harness::install(std::mem::size_of::<ArkWeb_ControllerAPI>(), false);
    assert!(matches!(
        missing_fn.run_javascript("web-1", "1", |_| {}),
        Err(ArkWebError::ArkWebApiMemberMissing(member)) if member == "runJavaScript"
    ));

    let (_harness, truncated) = Harness::install(std::mem::size_of::<usize>(), true);
    assert!(matches!(
        truncated.run_javascript("web-1", "1", |_| {}),
        Err(ArkWebError::ArkWebApiMemberMissing(member)) if member == "runJavaScript"
    ));

    let (_harness, valid) = Harness::install(std::mem::size_of::<ArkWeb_ControllerAPI>(), true);
    let drops = Rc::new(Cell::new(0));
    let guard = DropCounter(drops.clone());
    assert!(matches!(
        valid.run_javascript("bad\0tag", "1", move |_| {
            let _keep = guard;
        }),
        Err(ArkWebError::WebviewCreateFailed(_))
    ));
    assert_eq!(
        drops.get(),
        1,
        "early validation error drops callback without leaking"
    );
}
