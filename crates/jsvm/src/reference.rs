use std::ptr::NonNull;

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, non_null, Result};
use crate::{Env, FromJsValue, Value};

#[derive(Debug)]
pub struct Reference {
    raw: Option<NonNull<sys::JSVM_Ref__>>,
}

unsafe impl Send for Reference {}

impl Reference {
    pub fn new(env: &Env, value: Value, initial_refcount: u32) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CreateReference(env.as_raw(), value.as_raw(), initial_refcount, &mut raw)
        })?;
        Ok(Self {
            raw: Some(non_null(raw, "JSVM_Ref")?),
        })
    }

    pub(crate) fn from_raw(raw: sys::JSVM_Ref) -> Result<Self> {
        Ok(Self {
            raw: Some(non_null(raw, "JSVM_Ref")?),
        })
    }

    pub fn as_raw(&self) -> sys::JSVM_Ref {
        self.raw.map_or(std::ptr::null_mut(), |raw| raw.as_ptr())
    }

    pub fn get_value<T>(&self, env: &Env) -> Result<T>
    where
        T: FromJsValue,
    {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetReferenceValue(env.as_raw(), self.as_raw(), &mut raw)
        })?;
        T::from_js_value(env, Value::from_raw(raw)?)
    }

    pub fn reference(&self, env: &Env) -> Result<u32> {
        let mut result = 0u32;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_ReferenceRef(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn unref(&self, env: &Env) -> Result<u32> {
        let mut result = 0u32;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_ReferenceUnref(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn delete(mut self, env: &Env) -> Result<()> {
        if let Some(raw) = self.raw.take() {
            check_status_with_env(env, unsafe {
                sys::OH_JSVM_DeleteReference(env.as_raw(), raw.as_ptr())
            })?;
        }
        Ok(())
    }
}

impl Drop for Reference {
    fn drop(&mut self) {
        if self.raw.is_some() {
            eprintln!("JSVM Reference dropped without delete; call Reference::delete(env)");
        }
    }
}
