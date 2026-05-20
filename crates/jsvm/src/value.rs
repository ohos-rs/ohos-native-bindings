use std::ffi::c_void;
use std::fmt;
use std::slice;

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, non_null, type_mismatch, JsvmError, Result};
use crate::{Env, Function, Object, Promise, ValueType};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Value {
    raw: std::ptr::NonNull<sys::JSVM_Value__>,
}

impl Value {
    pub(crate) fn from_raw(raw: sys::JSVM_Value) -> Result<Self> {
        Ok(Self {
            raw: non_null(raw, "JSVM_Value")?,
        })
    }

    pub fn as_raw(self) -> sys::JSVM_Value {
        self.raw.as_ptr()
    }

    pub fn value_type(self, env: &Env) -> Result<ValueType> {
        let mut value_type = 0;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_Typeof(env.as_raw(), self.as_raw(), &mut value_type)
        })?;

        ValueType::from_raw(value_type).ok_or(JsvmError::Status(sys::JSVM_Status_JSVM_INVALID_ARG))
    }

    pub fn is_array(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsArray(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn is_arraybuffer(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsArraybuffer(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn is_typedarray(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsTypedarray(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn is_dataview(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsDataview(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn is_date(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsDate(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn is_promise(self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsPromise(env.as_raw(), self.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn strict_equals(self, env: &Env, other: Value) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_StrictEquals(env.as_raw(), self.as_raw(), other.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn equals(self, env: &Env, other: Value) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_Equals(env.as_raw(), self.as_raw(), other.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn instanceof(self, env: &Env, constructor: Value) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_Instanceof(
                env.as_raw(),
                self.as_raw(),
                constructor.as_raw(),
                &mut result,
            )
        })?;
        Ok(result)
    }

    pub fn coerce_to_string(self, env: &Env) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CoerceToString(env.as_raw(), self.as_raw(), &mut raw)
        })?;
        Self::from_raw(raw)
    }

    pub fn as_object(self, env: &Env) -> Result<Object> {
        Object::from_value(env, self)
    }

    pub fn as_function(self, env: &Env) -> Result<Function> {
        Function::from_value(env, self)
    }

    pub fn as_promise(self, env: &Env) -> Result<Promise> {
        Promise::from_value(env, self)
    }

    pub fn bytes(self, env: &Env) -> Result<Vec<u8>> {
        if self.is_arraybuffer(env)? {
            let mut data = std::ptr::null_mut::<c_void>();
            let mut byte_length = 0usize;
            check_status_with_env(env, unsafe {
                sys::OH_JSVM_GetArraybufferInfo(
                    env.as_raw(),
                    self.as_raw(),
                    &mut data,
                    &mut byte_length,
                )
            })?;

            let bytes = unsafe { slice::from_raw_parts(data.cast::<u8>(), byte_length) };
            return Ok(bytes.to_vec());
        }

        if self.is_typedarray(env)? {
            let mut typed_array_type = 0;
            let mut length = 0usize;
            let mut data = std::ptr::null_mut::<c_void>();
            let mut arraybuffer = std::ptr::null_mut();
            let mut byte_offset = 0usize;

            check_status_with_env(env, unsafe {
                sys::OH_JSVM_GetTypedarrayInfo(
                    env.as_raw(),
                    self.as_raw(),
                    &mut typed_array_type,
                    &mut length,
                    &mut data,
                    &mut arraybuffer,
                    &mut byte_offset,
                )
            })?;

            match typed_array_type {
                sys::JSVM_TypedarrayType_JSVM_UINT8_ARRAY
                | sys::JSVM_TypedarrayType_JSVM_UINT8_CLAMPED_ARRAY
                | sys::JSVM_TypedarrayType_JSVM_INT8_ARRAY => {
                    let bytes = unsafe { slice::from_raw_parts(data.cast::<u8>(), length) };
                    Ok(bytes.to_vec())
                }
                _ => Err(type_mismatch(
                    "Uint8Array, Uint8ClampedArray, or ArrayBuffer",
                    Some(self.value_type(env)?),
                )),
            }
        } else {
            Err(type_mismatch(
                "Uint8Array, Uint8ClampedArray, or ArrayBuffer",
                Some(self.value_type(env)?),
            ))
        }
    }
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Value").field(&self.raw).finish()
    }
}
