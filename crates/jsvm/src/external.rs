use std::ffi::c_void;
use std::marker::PhantomData;
use std::ptr::NonNull;

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, non_null, type_mismatch, Result};
use crate::{Env, FromJsValue, ToJsValue, Value, ValueType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct External<T> {
    ptr: NonNull<T>,
    _marker: PhantomData<T>,
}

impl<T> External<T> {
    pub fn as_ptr(&self) -> *mut T {
        self.ptr.as_ptr()
    }

    /// # Safety
    ///
    /// The caller must ensure the external value still owns a valid `T` and no mutable alias exists.
    pub unsafe fn as_ref<'a>(self) -> &'a T {
        unsafe { self.ptr.as_ref() }
    }

    /// # Safety
    ///
    /// The caller must ensure the external value still owns a valid `T` and this is the only mutable access.
    pub unsafe fn as_mut<'a>(mut self) -> &'a mut T {
        unsafe { self.ptr.as_mut() }
    }
}

impl<T> FromJsValue for External<T> {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::External => {
                let mut data = std::ptr::null_mut::<c_void>();
                check_status_with_env(env, unsafe {
                    sys::OH_JSVM_GetValueExternal(env.as_raw(), value.as_raw(), &mut data)
                })?;
                Ok(Self {
                    ptr: non_null(data.cast::<T>(), "External")?,
                    _marker: PhantomData,
                })
            }
            other => Err(type_mismatch("external", Some(other))),
        }
    }
}

impl<T> ToJsValue for External<T> {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_external_raw(self.as_ptr().cast(), None, std::ptr::null_mut())
    }
}

pub(crate) unsafe extern "C" fn drop_box_finalize<T>(
    _env: sys::JSVM_Env,
    finalize_data: *mut c_void,
    _finalize_hint: *mut c_void,
) {
    if !finalize_data.is_null() {
        unsafe {
            drop(Box::from_raw(finalize_data.cast::<T>()));
        }
    }
}
