use std::collections::{BTreeMap, HashMap};
use std::hash::BuildHasher;

use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, type_mismatch, JsvmError, Result};
use crate::{Env, Function, Object, Promise, Value, ValueType};

pub trait ToJsValue {
    fn to_js_value(&self, env: &Env) -> Result<Value>;
}

pub trait FromJsValue: Sized {
    fn from_js_value(env: &Env, value: Value) -> Result<Self>;
}

pub fn to_js_values(env: &Env, args: &[&dyn ToJsValue]) -> Result<Vec<Value>> {
    args.iter().map(|arg| arg.to_js_value(env)).collect()
}

impl ToJsValue for Value {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(*self)
    }
}

impl FromJsValue for Value {
    fn from_js_value(_env: &Env, value: Value) -> Result<Self> {
        Ok(value)
    }
}

impl ToJsValue for Object {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(self.as_value())
    }
}

impl FromJsValue for Object {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Object::from_value(env, value)
    }
}

impl ToJsValue for Function {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(self.as_value())
    }
}

impl FromJsValue for Function {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Function::from_value(env, value)
    }
}

impl ToJsValue for Promise {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(self.as_value())
    }
}

impl FromJsValue for Promise {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Promise::from_value(env, value)
    }
}

impl ToJsValue for () {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.undefined()
    }
}

impl FromJsValue for () {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::Undefined | ValueType::Null => Ok(()),
            other => Err(type_mismatch("undefined or null", Some(other))),
        }
    }
}

impl ToJsValue for bool {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.boolean(*self)
    }
}

impl FromJsValue for bool {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueBool(env.as_raw(), value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for i32 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_int32(*self)
    }
}

impl FromJsValue for i32 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut result = 0i32;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueInt32(env.as_raw(), value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for u32 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_uint32(*self)
    }
}

impl FromJsValue for u32 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut result = 0u32;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueUint32(env.as_raw(), value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for i64 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_int64(*self)
    }
}

impl FromJsValue for i64 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut result = 0i64;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueInt64(env.as_raw(), value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for f64 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_double(*self)
    }
}

impl FromJsValue for f64 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut result = 0f64;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueDouble(env.as_raw(), value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for f32 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_double((*self).into())
    }
}

impl FromJsValue for f32 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Ok(f64::from_js_value(env, value)? as f32)
    }
}

macro_rules! impl_small_signed {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToJsValue for $ty {
                fn to_js_value(&self, env: &Env) -> Result<Value> {
                    env.create_int32(i32::from(*self))
                }
            }

            impl FromJsValue for $ty {
                fn from_js_value(env: &Env, value: Value) -> Result<Self> {
                    let value = i32::from_js_value(env, value)?;
                    <$ty>::try_from(value).map_err(|_| JsvmError::IntegerOverflow(stringify!($ty)))
                }
            }
        )*
    };
}

macro_rules! impl_small_unsigned {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ToJsValue for $ty {
                fn to_js_value(&self, env: &Env) -> Result<Value> {
                    env.create_uint32(u32::from(*self))
                }
            }

            impl FromJsValue for $ty {
                fn from_js_value(env: &Env, value: Value) -> Result<Self> {
                    let value = u32::from_js_value(env, value)?;
                    <$ty>::try_from(value).map_err(|_| JsvmError::IntegerOverflow(stringify!($ty)))
                }
            }
        )*
    };
}

impl_small_signed!(i8, i16);
impl_small_unsigned!(u8, u16);

impl ToJsValue for str {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_string(self)
    }
}

impl ToJsValue for String {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        self.as_str().to_js_value(env)
    }
}

impl FromJsValue for String {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let value = value.coerce_to_string(env)?;
        let mut len = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueStringUtf8(
                env.as_raw(),
                value.as_raw(),
                std::ptr::null_mut(),
                0,
                &mut len,
            )
        })?;

        let mut buffer = vec![0u8; len + 1];
        let mut actual = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueStringUtf8(
                env.as_raw(),
                value.as_raw(),
                buffer.as_mut_ptr().cast(),
                buffer.len(),
                &mut actual,
            )
        })?;
        buffer.truncate(actual);
        String::from_utf8(buffer)
            .map_err(|_| JsvmError::Status(sys::JSVM_Status_JSVM_STRING_EXPECTED))
    }
}

impl<T> ToJsValue for Option<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        match self {
            Some(value) => value.to_js_value(env),
            None => env.null(),
        }
    }
}

impl<T, E> ToJsValue for std::result::Result<T, E>
where
    T: ToJsValue,
    E: ToJsValue,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        match self {
            Ok(value) => value.to_js_value(env),
            Err(value) => value.to_js_value(env),
        }
    }
}

impl<T> FromJsValue for Option<T>
where
    T: FromJsValue,
{
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::Undefined | ValueType::Null => Ok(None),
            _ => T::from_js_value(env, value).map(Some),
        }
    }
}

impl<T> ToJsValue for Vec<T>
where
    T: ToJsValue,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let array = env.create_array_with_length(self.len())?;
        let object = Object::from_value(env, array)?;
        for (index, value) in self.iter().enumerate() {
            object.set_element(env, index as u32, value)?;
        }
        Ok(array)
    }
}

impl<T> FromJsValue for Vec<T>
where
    T: FromJsValue,
{
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        if !value.is_array(env)? {
            return Err(type_mismatch("array", Some(value.value_type(env)?)));
        }

        let mut len = 0u32;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetArrayLength(env.as_raw(), value.as_raw(), &mut len)
        })?;

        let object = Object::from_value(env, value)?;
        let mut result = Vec::with_capacity(len as usize);
        for index in 0..len {
            result.push(object.get_element(env, index)?);
        }

        Ok(result)
    }
}

impl<T, S> ToJsValue for HashMap<String, T, S>
where
    T: ToJsValue,
    S: BuildHasher,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let object = Object::new(env)?;
        for (key, value) in self {
            object.set_named_property(env, key, value)?;
        }
        Ok(object.as_value())
    }
}

impl<T> FromJsValue for HashMap<String, T>
where
    T: FromJsValue,
{
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let object = Object::from_value(env, value)?;
        let keys = object.keys(env)?;
        let mut result = Self::with_capacity(keys.len());
        for key in keys {
            result.insert(key.clone(), object.get_named_property(env, &key)?);
        }
        Ok(result)
    }
}

impl<T> ToJsValue for BTreeMap<String, T>
where
    T: ToJsValue,
{
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let object = Object::new(env)?;
        for (key, value) in self {
            object.set_named_property(env, key, value)?;
        }
        Ok(object.as_value())
    }
}

impl<T> FromJsValue for BTreeMap<String, T>
where
    T: FromJsValue,
{
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let object = Object::from_value(env, value)?;
        let keys = object.keys(env)?;
        let mut result = Self::new();
        for key in keys {
            result.insert(key.clone(), object.get_named_property(env, &key)?);
        }
        Ok(result)
    }
}

impl ToJsValue for [u8] {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_arraybuffer(self)
    }
}

impl ToJsValue for Box<[u8]> {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        self.as_ref().to_js_value(env)
    }
}

impl<const N: usize> ToJsValue for [u8; N] {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        self.as_slice().to_js_value(env)
    }
}

impl FromJsValue for Box<[u8]> {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Ok(value.bytes(env)?.into_boxed_slice())
    }
}
