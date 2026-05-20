use std::ffi::CString;

use ohos_jsvm_sys as sys;

use crate::callback::Callback;
use crate::convert::{to_js_values, FromJsValue, ToJsValue};
use crate::error::{check_status_with_env, type_mismatch, JsvmError, Result};
use crate::{Env, Object, Value, ValueType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Function {
    value: Value,
}

impl Function {
    pub fn create(env: &Env, name: &str, callback: &'static Callback) -> Result<Self> {
        let name = CString::new(name).map_err(|_| JsvmError::InvalidString)?;
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CreateFunction(
                env.as_raw(),
                name.as_ptr(),
                name.as_bytes().len(),
                callback.as_raw(),
                &mut raw,
            )
        })?;

        Ok(Self {
            value: Value::from_raw(raw)?,
        })
    }

    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::Function => Ok(Self { value }),
            other => Err(type_mismatch("function", Some(other))),
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn as_object(self) -> Object {
        Object::from_value_unchecked(self.value)
    }

    pub fn call_raw(&self, env: &Env, recv: Option<Value>, args: &[Value]) -> Result<Value> {
        let receiver = match recv {
            Some(recv) => recv,
            None => env.global()?.as_value(),
        };

        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CallFunction(
                env.as_raw(),
                receiver.as_raw(),
                self.value.as_raw(),
                args.len(),
                if args.is_empty() {
                    std::ptr::null()
                } else {
                    args.as_ptr().cast::<sys::JSVM_Value>()
                },
                &mut raw,
            )
        })?;

        Value::from_raw(raw)
    }

    pub fn call<T>(&self, env: &Env, recv: Option<Value>, args: &[&dyn ToJsValue]) -> Result<T>
    where
        T: FromJsValue,
    {
        let args = to_js_values(env, args)?;
        let value = self.call_raw(env, recv, &args)?;
        T::from_js_value(env, value)
    }

    pub fn new_instance_raw(&self, env: &Env, args: &[Value]) -> Result<Object> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_NewInstance(
                env.as_raw(),
                self.value.as_raw(),
                args.len(),
                if args.is_empty() {
                    std::ptr::null()
                } else {
                    args.as_ptr().cast::<sys::JSVM_Value>()
                },
                &mut raw,
            )
        })?;
        Object::from_value(env, Value::from_raw(raw)?)
    }

    pub fn new_instance(&self, env: &Env, args: &[&dyn ToJsValue]) -> Result<Object> {
        let args = to_js_values(env, args)?;
        self.new_instance_raw(env, &args)
    }

    pub fn is_callable(&self, env: &Env) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_IsCallable(env.as_raw(), self.value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}
