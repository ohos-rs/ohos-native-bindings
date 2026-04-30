use ohos_jsvm_sys as sys;

use crate::convert::ToJsValue;
use crate::error::{check_status_with_env, non_null, type_mismatch, Result};
use crate::{Env, Value};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Promise {
    value: Value,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Deferred {
    raw: std::ptr::NonNull<sys::JSVM_Deferred__>,
}

impl Promise {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        if value.is_promise(env)? {
            Ok(Self { value })
        } else {
            Err(type_mismatch("promise", Some(value.value_type(env)?)))
        }
    }

    pub(crate) const fn from_value_unchecked(value: Value) -> Self {
        Self { value }
    }

    pub fn as_value(self) -> Value {
        self.value
    }
}

impl Deferred {
    pub(crate) fn from_raw(raw: sys::JSVM_Deferred) -> Result<Self> {
        Ok(Self {
            raw: non_null(raw, "JSVM_Deferred")?,
        })
    }

    pub fn as_raw(self) -> sys::JSVM_Deferred {
        self.raw.as_ptr()
    }

    pub fn resolve<T>(self, env: &Env, value: &T) -> Result<()>
    where
        T: ToJsValue + ?Sized,
    {
        let value = value.to_js_value(env)?;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_ResolveDeferred(env.as_raw(), self.as_raw(), value.as_raw())
        })
    }

    pub fn reject<T>(self, env: &Env, value: &T) -> Result<()>
    where
        T: ToJsValue + ?Sized,
    {
        let value = value.to_js_value(env)?;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_RejectDeferred(env.as_raw(), self.as_raw(), value.as_raw())
        })
    }
}

pub fn create_promise(env: &Env) -> Result<(Deferred, Promise)> {
    let mut deferred = std::ptr::null_mut();
    let mut promise = std::ptr::null_mut();
    check_status_with_env(env, unsafe {
        sys::OH_JSVM_CreatePromise(env.as_raw(), &mut deferred, &mut promise)
    })?;

    Ok((
        Deferred::from_raw(deferred)?,
        Promise::from_value_unchecked(Value::from_raw(promise)?),
    ))
}
