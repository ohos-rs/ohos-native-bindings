//! Production resource algorithms with native configuration effects substituted.
#![allow(
    non_snake_case,
    non_upper_case_globals,
    non_camel_case_types,
    dead_code
)]
extern crate self as ohos_resource_manager_sys;
use std::{
    ffi::{c_char, CString},
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

#[path = "../src/error.rs"]
mod error;
pub use error::RawFileError;
#[path = "../src/bounded_read.rs"]
mod bounded_read;
#[path = "../src/configuration.rs"]
mod configuration;

pub struct NativeResourceManager;
pub struct ResourceManager {
    resource_manager: NonNull<NativeResourceManager>,
}
pub struct ResourceManager_Configuration {
    locale: *mut c_char,
    deviceType: u32,
    colorMode: u32,
}
const ResourceManager_ErrorCode_SUCCESS: i32 = 0;
const ResourceManager_ColorMode_COLOR_MODE_DARK: u32 = 0;
static MODE: AtomicUsize = AtomicUsize::new(0);
static RELEASED: AtomicUsize = AtomicUsize::new(0);
unsafe fn OH_ResourceManager_GetConfiguration(
    _: *mut NativeResourceManager,
    output: *mut ResourceManager_Configuration,
) -> i32 {
    let mode = MODE.load(Ordering::SeqCst);
    let text = if mode == 1 {
        vec![0xff]
    } else {
        b"en-US".to_vec()
    };
    // SAFETY: the production method provides an initialized writable output;
    // this substitute transfers exactly one locale allocation to it.
    unsafe {
        (*output).locale = CString::new(text).unwrap().into_raw();
        (*output).deviceType = 7;
        (*output).colorMode = 0;
    }
    if mode == 2 {
        -1
    } else {
        0
    }
}
unsafe fn OH_ResourceManager_ReleaseConfiguration(
    output: *mut ResourceManager_Configuration,
) -> i32 {
    // SAFETY: GetConfiguration allocated this one terminated locale string.
    unsafe {
        drop(CString::from_raw((*output).locale));
        (*output).locale = std::ptr::null_mut();
    }
    RELEASED.fetch_add(1, Ordering::SeqCst);
    0
}

#[test]
fn configuration_releases_native_strings_on_success_decode_error_and_partial_failure() {
    let manager = ResourceManager {
        resource_manager: NonNull::dangling(),
    };
    for mode in 0..3 {
        MODE.store(mode, Ordering::SeqCst);
        let before = RELEASED.load(Ordering::SeqCst);
        let result = manager.configuration();
        if mode == 0 {
            let snapshot = result.unwrap();
            assert_eq!(snapshot.locale, "en-US");
            assert_eq!(snapshot.device_type, 7);
            assert!(snapshot.dark);
        } else {
            assert!(result.is_err());
        }
        assert_eq!(RELEASED.load(Ordering::SeqCst), before + 1);
    }
}
