//! Execute the production native-copy helpers with owner-backed static substitutes.
#![allow(non_camel_case_types, non_snake_case, dead_code)]

extern crate self as ohos_native_pasteboard_sys;
extern crate self as ohos_udmf_sys;

use std::{
    collections::BTreeMap,
    ffi::{CStr, CString},
    os::raw::c_char,
    ptr::NonNull,
    sync::atomic::{AtomicUsize, Ordering},
};

#[path = "../src/error.rs"]
mod error;
#[path = "../../udmf/src/error.rs"]
#[allow(clippy::enum_variant_names)]
mod udmf_error;
use udmf_error::UdmfError;
#[path = "../src/native_data.rs"]
mod native_data;
#[path = "../../udmf/src/udmf/native_data.rs"]
mod udmf_native_data;

#[derive(Default)]
struct NativeTypes {
    strings: Vec<CString>,
    pointers: Vec<*mut c_char>,
    mode: u8,
}

impl NativeTypes {
    fn new(values: &[&str]) -> Self {
        Self {
            strings: values.iter().map(|s| CString::new(*s).unwrap()).collect(),
            ..Self::default()
        }
    }

    unsafe fn get(&mut self, count: *mut u32) -> *mut *mut c_char {
        self.pointers = self.strings.iter().map(|s| s.as_ptr().cast_mut()).collect();
        // SAFETY: each production helper passes a writable count out-pointer.
        unsafe { *count = self.pointers.len() as u32 };
        if self.mode == 1 {
            // SAFETY: same out-pointer; simulate malformed native output.
            unsafe { *count = 1 };
            return std::ptr::null_mut();
        }
        if self.mode == 2 {
            self.pointers = vec![std::ptr::null_mut()];
            // SAFETY: same out-pointer; simulate a null string in a valid array.
            unsafe { *count = 1 };
        }
        if self.pointers.is_empty() {
            std::ptr::null_mut()
        } else {
            self.pointers.as_mut_ptr()
        }
    }
}

#[derive(Default)]
pub struct OH_UdmfRecord {
    entries: BTreeMap<String, Vec<u8>>,
    read: Vec<u8>,
    types: NativeTypes,
    status: i32,
    calls: usize,
    bad_buffer: bool,
    empty_buffer: bool,
    plain_text: String,
    html: String,
}

static LIVE_TYPED_UDS: AtomicUsize = AtomicUsize::new(0);

pub struct OH_UdsPlainText(String);
pub struct OH_UdsHtml(String);

pub struct UdsPlainText {
    raw: NonNull<OH_UdsPlainText>,
}

impl UdsPlainText {
    fn new() -> Self {
        LIVE_TYPED_UDS.fetch_add(1, Ordering::SeqCst);
        Self {
            raw: NonNull::from(Box::leak(Box::new(OH_UdsPlainText(String::new())))),
        }
    }
}

impl Drop for UdsPlainText {
    fn drop(&mut self) {
        // SAFETY: this test owner uniquely owns its boxed native substitute.
        unsafe { drop(Box::from_raw(self.raw.as_ptr())) };
        LIVE_TYPED_UDS.fetch_sub(1, Ordering::SeqCst);
    }
}

pub struct UdsHtml {
    raw: NonNull<OH_UdsHtml>,
}

impl UdsHtml {
    fn new() -> Self {
        LIVE_TYPED_UDS.fetch_add(1, Ordering::SeqCst);
        Self {
            raw: NonNull::from(Box::leak(Box::new(OH_UdsHtml(String::new())))),
        }
    }
}

impl Drop for UdsHtml {
    fn drop(&mut self) {
        // SAFETY: this test owner uniquely owns its boxed native substitute.
        unsafe { drop(Box::from_raw(self.raw.as_ptr())) };
        LIVE_TYPED_UDS.fetch_sub(1, Ordering::SeqCst);
    }
}

#[derive(Default)]
pub struct OH_UdmfData {
    types: NativeTypes,
    queries: usize,
}

#[derive(Default)]
pub struct OH_Pasteboard {
    types: NativeTypes,
    clear_status: i32,
    clears: usize,
    revision: u32,
}

unsafe fn OH_UdmfRecord_AddGeneralEntry(
    record: *mut OH_UdmfRecord,
    id: *const c_char,
    bytes: *mut u8,
    count: u32,
) -> i32 {
    // SAFETY: helper arguments point into live test-owned storage.
    let record = unsafe { &mut *record };
    record.calls += 1;
    if record.status != 0 {
        return record.status;
    }
    // SAFETY: helper provides a NUL-terminated identifier and count readable bytes.
    let id = unsafe { CStr::from_ptr(id) }.to_str().unwrap().to_owned();
    // SAFETY: the slice passed by the helper is readable for count bytes.
    let bytes = unsafe { std::slice::from_raw_parts(bytes, count as usize) }.to_vec();
    record.entries.insert(id, bytes);
    0
}

unsafe fn OH_UdmfRecord_GetGeneralEntry(
    record: *mut OH_UdmfRecord,
    id: *const c_char,
    bytes: *mut *mut u8,
    count: *mut u32,
) -> i32 {
    // SAFETY: the record and both out-pointers are live helper arguments.
    let record = unsafe { &mut *record };
    record.calls += 1;
    if record.status != 0 {
        return record.status;
    }
    if record.bad_buffer || record.empty_buffer {
        // SAFETY: helper supplies writable out-pointers.
        unsafe {
            *bytes = std::ptr::null_mut();
            *count = if record.empty_buffer { 0 } else { 5 };
        }
        return 0;
    }
    // SAFETY: helper supplies a live NUL-terminated identifier.
    let id = unsafe { CStr::from_ptr(id) }.to_str().unwrap();
    let Some(value) = record.entries.get(id) else {
        return 401;
    };
    record.read = value.clone();
    // SAFETY: helper supplies writable out-pointers. The record keeps the allocation.
    unsafe {
        *bytes = record.read.as_mut_ptr();
        *count = record.read.len() as u32;
    }
    0
}

unsafe fn OH_UdmfRecord_GetTypes(record: *mut OH_UdmfRecord, count: *mut u32) -> *mut *mut c_char {
    // SAFETY: live owner and out-pointer supplied by the production helper.
    unsafe { (*record).types.get(count) }
}

unsafe fn OH_UdmfRecord_GetPlainText(
    record: *mut OH_UdmfRecord,
    text: *mut OH_UdsPlainText,
) -> i32 {
    // SAFETY: the helper supplies live record and independently allocated output.
    let record = unsafe { &*record };
    if record.status == 0 {
        // SAFETY: this writable output is uniquely owned by the helper.
        unsafe { (*text).0 = record.plain_text.clone() };
    }
    record.status
}

unsafe fn OH_UdmfRecord_GetHtml(record: *mut OH_UdmfRecord, html: *mut OH_UdsHtml) -> i32 {
    // SAFETY: the helper supplies live record and independently allocated output.
    let record = unsafe { &*record };
    if record.status == 0 {
        // SAFETY: this writable output is uniquely owned by the helper.
        unsafe { (*html).0 = record.html.clone() };
    }
    record.status
}

unsafe fn OH_UdmfData_GetTypes(data: *mut OH_UdmfData, count: *mut u32) -> *mut *mut c_char {
    // SAFETY: live owner and out-pointer supplied by the production helper.
    unsafe { (*data).types.get(count) }
}

unsafe fn OH_UdmfData_HasType(data: *mut OH_UdmfData, id: *const c_char) -> bool {
    // SAFETY: live data and NUL-terminated identifier supplied by the helper.
    let data = unsafe { &mut *data };
    data.queries += 1;
    // SAFETY: helper CString lives until this call returns.
    let id = unsafe { CStr::from_ptr(id) };
    data.types
        .strings
        .iter()
        .any(|value| value.as_c_str() == id)
}

unsafe fn OH_Pasteboard_ClearData(board: *mut OH_Pasteboard) -> i32 {
    // SAFETY: the helper supplies a live pasteboard.
    let board = unsafe { &mut *board };
    board.clears += 1;
    board.clear_status
}

#[cfg(feature = "api-14")]
unsafe fn OH_Pasteboard_GetMimeTypes(
    board: *mut OH_Pasteboard,
    count: *mut u32,
) -> *mut *mut c_char {
    // SAFETY: live owner and out-pointer supplied by the production helper.
    unsafe { (*board).types.get(count) }
}

#[cfg(feature = "api-18")]
unsafe fn OH_Pasteboard_GetChangeCount(board: *mut OH_Pasteboard) -> u32 {
    // SAFETY: the helper supplies a live pasteboard.
    unsafe { (*board).revision }
}

#[test]
fn binary_entries_copy_inputs_and_outputs_without_taking_native_ownership() {
    let mut record = OH_UdmfRecord::default();
    let raw = NonNull::from(&mut record);
    let input = vec![0, 1, 255, 0, 2];
    udmf_native_data::add_general_entry(raw, "application/x-binary", &input).unwrap();
    drop(input);
    let first = udmf_native_data::general_entry(raw, "application/x-binary").unwrap();
    record
        .entries
        .insert("application/x-binary".into(), vec![9]);
    let second = udmf_native_data::general_entry(raw, "application/x-binary").unwrap();
    assert_eq!(first, [0, 1, 255, 0, 2]);
    assert_eq!(second, [9]);
    assert_eq!(record.read, [9]);
    drop(record);
    assert_eq!(first, [0, 1, 255, 0, 2]);
}

#[test]
fn native_errors_and_malformed_entry_buffers_are_not_silently_rewritten() {
    let mut record = OH_UdmfRecord {
        status: -7,
        ..Default::default()
    };
    let raw = NonNull::from(&mut record);
    assert!(matches!(
        udmf_native_data::add_general_entry(raw, "x", &[1]),
        Err(UdmfError::InternalError(-7))
    ));
    assert!(matches!(
        udmf_native_data::general_entry(raw, "x"),
        Err(UdmfError::InternalError(-7))
    ));
    record.status = 0;
    record.bad_buffer = true;
    assert!(udmf_native_data::general_entry(raw, "x").is_err());
    record.empty_buffer = true;
    assert_eq!(udmf_native_data::general_entry(raw, "x").unwrap(), []);
}

#[test]
fn interior_nul_type_ids_are_rejected_before_ffi() {
    let mut record = OH_UdmfRecord::default();
    let raw = NonNull::from(&mut record);
    assert!(udmf_native_data::add_general_entry(raw, "x\0y", &[1]).is_err());
    assert!(udmf_native_data::general_entry(raw, "x\0y").is_err());
    assert_eq!(record.calls, 0);
    let mut data = OH_UdmfData::default();
    assert!(udmf_native_data::has_type(NonNull::from(&mut data), "x\0y").is_err());
    assert_eq!(data.queries, 0);
}

#[test]
fn empty_general_entries_return_a_typed_error_without_calling_native() {
    let mut record = OH_UdmfRecord::default();
    assert!(matches!(
        udmf_native_data::add_general_entry(NonNull::from(&mut record), "application/x-empty", &[]),
        Err(UdmfError::EmptyEntry)
    ));
    assert_eq!(record.calls, 0);
}

#[test]
fn typed_plain_text_and_html_are_owned_and_failed_outputs_are_released() {
    let mut record = OH_UdmfRecord {
        plain_text: "Other app's text 😀".into(),
        html: "<b>Other app</b>".into(),
        ..Default::default()
    };
    let raw = NonNull::from(&mut record);
    let text = udmf_native_data::plain_text(raw).unwrap();
    let html = udmf_native_data::html(raw).unwrap();
    assert_eq!(LIVE_TYPED_UDS.load(Ordering::SeqCst), 2);
    // Standard UDS values have no custom ARRAY_BUFFER representation.
    assert!(udmf_native_data::general_entry(raw, "general.plain-text").is_err());
    record.status = -9;
    assert!(matches!(
        udmf_native_data::plain_text(raw),
        Err(UdmfError::InternalError(-9))
    ));
    assert!(matches!(
        udmf_native_data::html(raw),
        Err(UdmfError::InternalError(-9))
    ));
    assert_eq!(LIVE_TYPED_UDS.load(Ordering::SeqCst), 2);
    drop(record);
    // SAFETY: each independently returned UDS owner remains live after the record.
    assert_eq!(unsafe { &text.raw.as_ref().0 }, "Other app's text 😀");
    // SAFETY: same independent ownership guarantee for HTML.
    assert_eq!(unsafe { &html.raw.as_ref().0 }, "<b>Other app</b>");
    drop((text, html));
    assert_eq!(LIVE_TYPED_UDS.load(Ordering::SeqCst), 0);
}

#[test]
fn record_and_data_type_snapshots_outlive_their_native_owners() {
    let mut record = OH_UdmfRecord {
        types: NativeTypes::new(&["text/plain", "application/x-a"]),
        ..Default::default()
    };
    let values = udmf_native_data::record_types(NonNull::from(&mut record)).unwrap();
    assert_eq!(values, ["text/plain", "application/x-a"]);
    record.types = NativeTypes::default();
    assert!(udmf_native_data::record_types(NonNull::from(&mut record))
        .unwrap()
        .is_empty());
    drop(record);
    assert_eq!(values[1], "application/x-a");
    let mut data = OH_UdmfData {
        types: NativeTypes::new(&["a", "b"]),
        ..Default::default()
    };
    let raw = NonNull::from(&mut data);
    assert!(udmf_native_data::has_type(raw, "b").unwrap());
    assert!(!udmf_native_data::has_type(raw, "c").unwrap());
    let values = udmf_native_data::data_types(raw).unwrap();
    drop(data);
    assert_eq!(values, ["a", "b"]);
}

#[test]
fn invalid_native_type_arrays_and_utf8_are_errors() {
    let mut record = OH_UdmfRecord::default();
    for mode in [1, 2] {
        record.types.mode = mode;
        assert!(udmf_native_data::record_types(NonNull::from(&mut record)).is_err());
    }
    record.types = NativeTypes {
        strings: vec![CString::new(vec![255]).unwrap()],
        ..Default::default()
    };
    assert!(udmf_native_data::record_types(NonNull::from(&mut record)).is_err());
}

#[test]
fn clear_forwards_native_success_and_failure_without_a_subscription() {
    let mut board = OH_Pasteboard::default();
    let raw = NonNull::from(&mut board);
    native_data::clear(raw).unwrap();
    board.clear_status = -8;
    assert!(matches!(
        native_data::clear(raw),
        Err(error::PasteboardError::InternalError(-8))
    ));
    assert_eq!(board.clears, 2);
}

#[cfg(feature = "api-14")]
#[test]
fn mime_snapshots_copy_owner_memory_and_handle_empty_invalid_and_utf8() {
    let mut board = OH_Pasteboard {
        types: NativeTypes::new(&["text/html", "text/plain"]),
        ..Default::default()
    };
    let raw = NonNull::from(&mut board);
    let values = native_data::mime_types(raw).unwrap();
    board.types = NativeTypes::default();
    assert!(native_data::mime_types(raw).unwrap().is_empty());
    for mode in [1, 2] {
        board.types.mode = mode;
        assert!(native_data::mime_types(raw).is_err());
    }
    board.types = NativeTypes {
        strings: vec![CString::new(vec![255]).unwrap()],
        ..Default::default()
    };
    assert!(native_data::mime_types(raw).is_err());
    drop(board);
    assert_eq!(values, ["text/html", "text/plain"]);
}

#[cfg(feature = "api-18")]
#[test]
fn revision_preserves_full_u32_and_native_zero_sentinel() {
    let mut board = OH_Pasteboard {
        revision: u32::MAX,
        ..Default::default()
    };
    let raw = NonNull::from(&mut board);
    assert_eq!(native_data::change_count(raw), u32::MAX);
    board.revision = 0;
    assert_eq!(native_data::change_count(raw), 0);
}
