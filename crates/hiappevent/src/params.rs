use crate::array_len;
use crate::error::{HiAppEventError, Result};
use ohos_hiappevent_sys as sys;
use std::any::Any;
use std::ffi::{c_char, CString};

/// A built, owned event parameter list. Frees the native list on drop.
///
/// Build one with [`EventParams::builder`], then pass it to
/// [`write`](crate::write).
pub struct EventParams {
    raw: sys::ParamList,
    // The native list stores the pointers it was handed; the payloads they point
    // at are owned here and outlive the list.
    _keep: Vec<Box<dyn Any>>,
}

impl EventParams {
    /// Start building a parameter list.
    pub fn builder() -> EventParamsBuilder {
        EventParamsBuilder::default()
    }

    pub(crate) fn as_raw(&self) -> sys::ParamList {
        self.raw
    }
}

impl Drop for EventParams {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            // SAFETY: raw came from OH_HiAppEvent_CreateParamList and is freed once.
            unsafe { sys::OH_HiAppEvent_DestroyParamList(self.raw) };
        }
    }
}

enum Value {
    Bool(bool),
    I8(i8),
    I16(i16),
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    Str(CString),
    BoolArray(Vec<bool>),
    I8Array(Vec<i8>),
    I16Array(Vec<i16>),
    I32Array(Vec<i32>),
    I64Array(Vec<i64>),
    F32Array(Vec<f32>),
    F64Array(Vec<f64>),
    StrArray(Vec<CString>),
}

/// Opaque carrier for a converted parameter value, produced by
/// [`IntoEventValue`]. It has no public constructor, which is what seals the
/// trait: the private [`Value`] cannot be named outside this crate.
#[doc(hidden)]
pub struct EventValue(Value);

/// A value that can be added to an [`EventParamsBuilder`], so a single
/// [`add`](EventParamsBuilder::add) covers every parameter type.
///
/// Implemented for `bool`, the signed integers `i8`/`i16`/`i32`/`i64`,
/// `f32`/`f64`, `&str`, a slice of any of those scalar types, and `&[&str]`.
/// A `&str` value (or a string in a `&[&str]`) with an interior NUL byte is
/// reported by [`build`](EventParamsBuilder::build). Sealed: it cannot be
/// implemented outside this crate.
pub trait IntoEventValue {
    #[doc(hidden)]
    fn into_event_value(self) -> Result<EventValue>;
}

macro_rules! scalar_value {
    ($($ty:ty => $variant:ident),* $(,)?) => {$(
        impl IntoEventValue for $ty {
            fn into_event_value(self) -> Result<EventValue> {
                Ok(EventValue(Value::$variant(self)))
            }
        }
    )*};
}

scalar_value! {
    bool => Bool,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    f32 => F32,
    f64 => F64,
}

macro_rules! array_value {
    ($($ty:ty => $variant:ident),* $(,)?) => {$(
        impl IntoEventValue for &[$ty] {
            fn into_event_value(self) -> Result<EventValue> {
                Ok(EventValue(Value::$variant(self.to_vec())))
            }
        }
    )*};
}

array_value! {
    bool => BoolArray,
    i8 => I8Array,
    i16 => I16Array,
    i32 => I32Array,
    i64 => I64Array,
    f32 => F32Array,
    f64 => F64Array,
}

impl IntoEventValue for &str {
    fn into_event_value(self) -> Result<EventValue> {
        Ok(EventValue(Value::Str(CString::new(self)?)))
    }
}

impl IntoEventValue for &[&str] {
    fn into_event_value(self) -> Result<EventValue> {
        let mut owned = Vec::with_capacity(self.len());
        for value in self {
            owned.push(CString::new(*value)?);
        }
        Ok(EventValue(Value::StrArray(owned)))
    }
}

/// Builder for an [`EventParams`]. [`add`](Self::add) names a parameter and gives
/// its value; a name or string value with an interior NUL byte is reported by
/// [`build`](Self::build).
#[derive(Default)]
pub struct EventParamsBuilder {
    params: Vec<(CString, Value)>,
    error: Option<HiAppEventError>,
}

impl EventParamsBuilder {
    /// Add a parameter named `name` with the given value.
    ///
    /// The value may be a scalar (`bool`, `i8`/`i16`/`i32`/`i64`, `f32`/`f64`),
    /// a `&str`, a slice of any scalar type, or `&[&str]` — see [`IntoEventValue`].
    pub fn add(mut self, name: &str, value: impl IntoEventValue) -> Self {
        match value.into_event_value() {
            Ok(EventValue(value)) => self.push(name, value),
            Err(error) => {
                self.error.get_or_insert(error);
                self
            }
        }
    }

    fn push(mut self, name: &str, value: Value) -> Self {
        match CString::new(name) {
            Ok(name) => self.params.push((name, value)),
            Err(_) => {
                self.error.get_or_insert(HiAppEventError::Nul);
            }
        }
        self
    }

    /// Create the native list and add every parameter to it.
    pub fn build(self) -> Result<EventParams> {
        if let Some(error) = self.error {
            return Err(error);
        }
        // SAFETY: the call takes no arguments and yields an owned list or null.
        let mut raw = unsafe { sys::OH_HiAppEvent_CreateParamList() };
        if raw.is_null() {
            return Err(HiAppEventError::Alloc);
        }

        // `params` is moved into the returned EventParams below. Moving a Vec
        // leaves its heap buffer — and therefore every pointer taken here — in
        // place.
        let params = self.params;
        let mut keep: Vec<Box<dyn Any>> = Vec::new();
        for (name, value) in &params {
            let name = name.as_ptr();
            // SAFETY: `name` and the payload behind every pointer below are owned
            // by `params`/`keep` and outlive the list.
            let next = unsafe {
                match value {
                    Value::Bool(v) => sys::OH_HiAppEvent_AddBoolParam(raw, name, *v),
                    Value::I8(v) => sys::OH_HiAppEvent_AddInt8Param(raw, name, *v),
                    Value::I16(v) => sys::OH_HiAppEvent_AddInt16Param(raw, name, *v),
                    Value::I32(v) => sys::OH_HiAppEvent_AddInt32Param(raw, name, *v),
                    Value::I64(v) => sys::OH_HiAppEvent_AddInt64Param(raw, name, *v),
                    Value::F32(v) => sys::OH_HiAppEvent_AddFloatParam(raw, name, *v),
                    Value::F64(v) => sys::OH_HiAppEvent_AddDoubleParam(raw, name, *v),
                    Value::Str(v) => sys::OH_HiAppEvent_AddStringParam(raw, name, v.as_ptr()),
                    Value::BoolArray(v) => sys::OH_HiAppEvent_AddBoolArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::I8Array(v) => sys::OH_HiAppEvent_AddInt8ArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::I16Array(v) => sys::OH_HiAppEvent_AddInt16ArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::I32Array(v) => sys::OH_HiAppEvent_AddInt32ArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::I64Array(v) => sys::OH_HiAppEvent_AddInt64ArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::F32Array(v) => sys::OH_HiAppEvent_AddFloatArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::F64Array(v) => sys::OH_HiAppEvent_AddDoubleArrayParam(
                        raw,
                        name,
                        v.as_ptr(),
                        array_len(v.len()),
                    ),
                    Value::StrArray(v) => {
                        // Two levels of storage: the strings and the array of
                        // pointers into them. Both must outlive the list.
                        let ptrs: Vec<*const c_char> = v.iter().map(|s| s.as_ptr()).collect();
                        let len = array_len(ptrs.len());
                        let next =
                            sys::OH_HiAppEvent_AddStringArrayParam(raw, name, ptrs.as_ptr(), len);
                        keep.push(Box::new(ptrs));
                        next
                    }
                }
            };
            // The native list is a cons list: a successful add returns the new
            // head. A null return means the parameter was not added, so the
            // previous head is kept rather than leaked.
            if !next.is_null() {
                raw = next;
            }
        }

        keep.push(Box::new(params));
        Ok(EventParams { raw, _keep: keep })
    }
}
