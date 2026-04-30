use std::ffi::c_void;
use std::ptr::{self, NonNull};

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, non_null, Result};
use crate::{Env, FromJsValue, Value};

#[derive(Debug)]
pub struct Callback {
    raw: sys::JSVM_CallbackStruct,
}

// Callback descriptors are immutable after construction and are commonly stored as `static`
// values before being registered with JSVM.
unsafe impl Sync for Callback {}

impl Callback {
    pub const fn new(
        callback: unsafe extern "C" fn(sys::JSVM_Env, sys::JSVM_CallbackInfo) -> sys::JSVM_Value,
        data: *mut c_void,
    ) -> Self {
        Self {
            raw: sys::JSVM_CallbackStruct {
                callback: Some(callback),
                data,
            },
        }
    }

    pub(crate) fn as_raw(&'static self) -> sys::JSVM_Callback {
        (&self.raw as *const sys::JSVM_CallbackStruct).cast_mut()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CallbackInfo {
    raw: NonNull<sys::JSVM_CallbackInfo__>,
}

impl CallbackInfo {
    pub fn from_raw(raw: sys::JSVM_CallbackInfo) -> Result<Self> {
        Ok(Self {
            raw: non_null(raw, "JSVM_CallbackInfo")?,
        })
    }

    pub fn as_raw(self) -> sys::JSVM_CallbackInfo {
        self.raw.as_ptr()
    }

    pub fn args(self, env: &Env) -> Result<Vec<Value>> {
        let mut argc = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetCbInfo(
                env.as_raw(),
                self.as_raw(),
                &mut argc,
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        })?;

        let mut argv = vec![ptr::null_mut(); argc];
        let mut actual_argc = argc;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetCbInfo(
                env.as_raw(),
                self.as_raw(),
                &mut actual_argc,
                argv.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        })?;

        argv.into_iter()
            .take(actual_argc)
            .map(Value::from_raw)
            .collect()
    }

    pub fn arg<T>(self, env: &Env, index: usize) -> Result<T>
    where
        T: FromJsValue,
    {
        let args = self.args(env)?;
        let value = args.get(index).copied().unwrap_or(env.undefined()?);
        T::from_js_value(env, value)
    }

    pub fn this_arg(self, env: &Env) -> Result<Value> {
        let mut this_arg = ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetCbInfo(
                env.as_raw(),
                self.as_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut this_arg,
                ptr::null_mut(),
            )
        })?;

        Value::from_raw(this_arg)
    }

    pub fn data<T>(self, env: &Env) -> Result<Option<*mut T>> {
        let mut data = ptr::null_mut::<c_void>();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetCbInfo(
                env.as_raw(),
                self.as_raw(),
                ptr::null_mut(),
                ptr::null_mut(),
                ptr::null_mut(),
                &mut data,
            )
        })?;

        Ok(NonNull::new(data).map(|data| data.cast::<T>().as_ptr()))
    }

    pub fn new_target(self, env: &Env) -> Result<Option<Value>> {
        let mut raw = ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetNewTarget(env.as_raw(), self.as_raw(), &mut raw)
        })?;
        if raw.is_null() {
            Ok(None)
        } else {
            Value::from_raw(raw).map(Some)
        }
    }
}
