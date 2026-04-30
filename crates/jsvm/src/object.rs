use std::ffi::c_void;
use std::ffi::CString;

use ohos_jsvm_sys as sys;

use crate::callback::Callback;
use crate::convert::{FromJsValue, ToJsValue};
use crate::error::{check_status_with_env, type_mismatch, JsvmError, Result};
use crate::external::drop_box_finalize;
use crate::{Env, Reference, TypeTag, Value, ValueType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Object {
    value: Value,
}

impl Object {
    pub fn new(env: &Env) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CreateObject(env.as_raw(), &mut raw)
        })?;
        Ok(Self {
            value: Value::from_raw(raw)?,
        })
    }

    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::Object | ValueType::Function => Ok(Self { value }),
            other => Err(type_mismatch("object", Some(other))),
        }
    }

    pub(crate) const fn from_value_unchecked(value: Value) -> Self {
        Self { value }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn set_named_property<T>(&self, env: &Env, name: &str, value: &T) -> Result<()>
    where
        T: ToJsValue + ?Sized,
    {
        let name = CString::new(name).map_err(|_| JsvmError::InvalidString)?;
        let value = value.to_js_value(env)?;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_SetNamedProperty(
                env.as_raw(),
                self.value.as_raw(),
                name.as_ptr(),
                value.as_raw(),
            )
        })
    }

    pub fn define_method(&self, env: &Env, name: &str, callback: &'static Callback) -> Result<()> {
        let name = CString::new(name).map_err(|_| JsvmError::InvalidString)?;
        let descriptor = sys::JSVM_PropertyDescriptor {
            utf8name: name.as_ptr(),
            name: std::ptr::null_mut(),
            method: callback.as_raw(),
            getter: std::ptr::null_mut(),
            setter: std::ptr::null_mut(),
            value: std::ptr::null_mut(),
            attributes: sys::JSVM_PropertyAttributes_JSVM_DEFAULT_METHOD,
        };
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_DefineProperties(env.as_raw(), self.value.as_raw(), 1, &descriptor)
        })
    }

    pub fn get_named_property<T>(&self, env: &Env, name: &str) -> Result<T>
    where
        T: FromJsValue,
    {
        let name = CString::new(name).map_err(|_| JsvmError::InvalidString)?;
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetNamedProperty(
                env.as_raw(),
                self.value.as_raw(),
                name.as_ptr(),
                &mut raw,
            )
        })?;
        T::from_js_value(env, Value::from_raw(raw)?)
    }

    pub fn has_named_property(&self, env: &Env, name: &str) -> Result<bool> {
        let name = CString::new(name).map_err(|_| JsvmError::InvalidString)?;
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_HasNamedProperty(
                env.as_raw(),
                self.value.as_raw(),
                name.as_ptr(),
                &mut result,
            )
        })?;
        Ok(result)
    }

    pub fn delete_named_property(&self, env: &Env, name: &str) -> Result<bool> {
        let key = env.create_string(name)?;
        self.delete_property(env, key)
    }

    pub fn set_property<K, V>(&self, env: &Env, key: &K, value: &V) -> Result<()>
    where
        K: ToJsValue + ?Sized,
        V: ToJsValue + ?Sized,
    {
        let key = key.to_js_value(env)?;
        let value = value.to_js_value(env)?;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_SetProperty(
                env.as_raw(),
                self.value.as_raw(),
                key.as_raw(),
                value.as_raw(),
            )
        })
    }

    pub fn get_property<T, K>(&self, env: &Env, key: &K) -> Result<T>
    where
        T: FromJsValue,
        K: ToJsValue + ?Sized,
    {
        let key = key.to_js_value(env)?;
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetProperty(env.as_raw(), self.value.as_raw(), key.as_raw(), &mut raw)
        })?;
        T::from_js_value(env, Value::from_raw(raw)?)
    }

    pub fn has_property<K>(&self, env: &Env, key: &K) -> Result<bool>
    where
        K: ToJsValue + ?Sized,
    {
        let key = key.to_js_value(env)?;
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_HasProperty(env.as_raw(), self.value.as_raw(), key.as_raw(), &mut result)
        })?;
        Ok(result)
    }

    pub fn has_own_property<K>(&self, env: &Env, key: &K) -> Result<bool>
    where
        K: ToJsValue + ?Sized,
    {
        let key = key.to_js_value(env)?;
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_HasOwnProperty(
                env.as_raw(),
                self.value.as_raw(),
                key.as_raw(),
                &mut result,
            )
        })?;
        Ok(result)
    }

    pub fn delete_property(&self, env: &Env, key: Value) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_DeleteProperty(
                env.as_raw(),
                self.value.as_raw(),
                key.as_raw(),
                &mut result,
            )
        })?;
        Ok(result)
    }

    pub fn set_element<T>(&self, env: &Env, index: u32, value: &T) -> Result<()>
    where
        T: ToJsValue + ?Sized,
    {
        let value = value.to_js_value(env)?;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_SetElement(env.as_raw(), self.value.as_raw(), index, value.as_raw())
        })
    }

    pub fn get_element<T>(&self, env: &Env, index: u32) -> Result<T>
    where
        T: FromJsValue,
    {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetElement(env.as_raw(), self.value.as_raw(), index, &mut raw)
        })?;
        T::from_js_value(env, Value::from_raw(raw)?)
    }

    pub fn has_element(&self, env: &Env, index: u32) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_HasElement(env.as_raw(), self.value.as_raw(), index, &mut result)
        })?;
        Ok(result)
    }

    pub fn delete_element(&self, env: &Env, index: u32) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_DeleteElement(env.as_raw(), self.value.as_raw(), index, &mut result)
        })?;
        Ok(result)
    }

    pub fn keys(&self, env: &Env) -> Result<Vec<String>> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetPropertyNames(env.as_raw(), self.value.as_raw(), &mut raw)
        })?;
        Vec::<String>::from_js_value(env, Value::from_raw(raw)?)
    }

    pub fn prototype(&self, env: &Env) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetPrototype(env.as_raw(), self.value.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn freeze(&self, env: &Env) -> Result<()> {
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_ObjectFreeze(env.as_raw(), self.value.as_raw())
        })
    }

    pub fn seal(&self, env: &Env) -> Result<()> {
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_ObjectSeal(env.as_raw(), self.value.as_raw())
        })
    }

    pub fn add_finalizer<T: 'static>(&self, env: &Env, data: T) -> Result<Reference> {
        let data = Box::into_raw(Box::new(data)).cast::<c_void>();
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_AddFinalizer(
                env.as_raw(),
                self.value.as_raw(),
                data,
                Some(drop_box_finalize::<T>),
                std::ptr::null_mut(),
                &mut raw,
            )
        })?;
        Reference::from_raw(raw)
    }

    pub fn wrap<T: 'static>(&self, env: &Env, native: T) -> Result<Reference> {
        let native = Box::into_raw(Box::new(native)).cast::<c_void>();
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_Wrap(
                env.as_raw(),
                self.value.as_raw(),
                native,
                Some(drop_box_finalize::<T>),
                std::ptr::null_mut(),
                &mut raw,
            )
        })?;
        Reference::from_raw(raw)
    }

    /// # Safety
    ///
    /// The caller must ensure that this object was wrapped with `T` and that returned references
    /// do not outlive the JS object or alias another mutable access.
    pub unsafe fn unwrap<T>(&self, env: &Env) -> Result<&mut T> {
        let mut raw = std::ptr::null_mut::<c_void>();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_Unwrap(env.as_raw(), self.value.as_raw(), &mut raw)
        })?;
        Ok(unsafe { &mut *raw.cast::<T>() })
    }

    /// # Safety
    ///
    /// The caller must ensure that this object was wrapped with `T`. This removes the JS finalizer
    /// and transfers ownership of the native value back to Rust.
    pub unsafe fn remove_wrap<T>(&self, env: &Env) -> Result<Box<T>> {
        let mut raw = std::ptr::null_mut::<c_void>();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_RemoveWrap(env.as_raw(), self.value.as_raw(), &mut raw)
        })?;
        Ok(unsafe { Box::from_raw(raw.cast::<T>()) })
    }

    pub fn type_tag(&self, env: &Env, tag: &TypeTag) -> Result<()> {
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_TypeTagObject(env.as_raw(), self.value.as_raw(), tag.as_raw())
        })
    }

    pub fn check_type_tag(&self, env: &Env, tag: &TypeTag) -> Result<bool> {
        let mut result = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_CheckObjectTypeTag(
                env.as_raw(),
                self.value.as_raw(),
                tag.as_raw(),
                &mut result,
            )
        })?;
        Ok(result)
    }
}
