use std::ffi::c_void;

use ohos_jsvm_sys as sys;

use crate::callback::Callback;
use crate::convert::FromJsValue;
use crate::error::{check_status, check_status_with_env, non_null, Result};
use crate::external::drop_box_finalize;
use crate::function::Function;
use crate::object::Object;
use crate::promise::{create_promise, Deferred, Promise};
use crate::thread;
use crate::Reference;
use crate::TypedArrayType;
use crate::Value;

#[derive(Debug)]
pub struct Env {
    raw: std::ptr::NonNull<sys::JSVM_Env__>,
    owned: bool,
}

impl Env {
    pub(crate) fn from_raw(raw: sys::JSVM_Env) -> Result<Self> {
        Ok(Self {
            raw: non_null(raw, "JSVM_Env")?,
            owned: true,
        })
    }

    /// # Safety
    ///
    /// The caller must ensure `raw` is a valid environment for the duration of the returned
    /// borrowed handle. The returned `Env` does not destroy the raw environment on drop.
    pub unsafe fn from_borrowed_raw(raw: sys::JSVM_Env) -> Result<Self> {
        Ok(Self {
            raw: non_null(raw, "JSVM_Env")?,
            owned: false,
        })
    }

    pub fn as_raw(&self) -> sys::JSVM_Env {
        self.raw.as_ptr()
    }

    pub fn open_scope(&self) -> Result<EnvScope<'_>> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_JSVM_OpenEnvScope(self.as_raw(), &mut raw) })?;
        Ok(EnvScope {
            env: self,
            raw: non_null(raw, "JSVM_EnvScope")?,
        })
    }

    pub fn open_handle_scope(&self) -> Result<HandleScope<'_>> {
        let mut raw = std::ptr::null_mut();
        check_status(unsafe { sys::OH_JSVM_OpenHandleScope(self.as_raw(), &mut raw) })?;
        Ok(HandleScope {
            env: self,
            raw: non_null(raw, "JSVM_HandleScope")?,
        })
    }

    pub fn global(&self) -> Result<Object> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_GetGlobal(self.as_raw(), &mut raw)
        })?;
        Object::from_value(self, Value::from_raw(raw)?)
    }

    pub fn undefined(&self) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_GetUndefined(self.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn null(&self) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_GetNull(self.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn boolean(&self, value: bool) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_GetBoolean(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_int32(&self, value: i32) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateInt32(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_uint32(&self, value: u32) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateUint32(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_int64(&self, value: i64) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateInt64(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_double(&self, value: f64) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateDouble(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_string(&self, value: &str) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateStringUtf8(
                self.as_raw(),
                value.as_ptr().cast(),
                value.len(),
                &mut raw,
            )
        })?;
        Value::from_raw(raw)
    }

    pub(crate) fn create_array_with_length(&self, length: usize) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateArrayWithLength(self.as_raw(), length, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub(crate) fn create_arraybuffer(&self, bytes: &[u8]) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        let mut data = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateArraybuffer(self.as_raw(), bytes.len(), &mut data, &mut raw)
        })?;

        if !data.is_null() && !bytes.is_empty() {
            unsafe {
                std::ptr::copy_nonoverlapping(bytes.as_ptr(), data.cast::<u8>(), bytes.len());
            }
        }

        Value::from_raw(raw)
    }

    pub fn create_typedarray(
        &self,
        array_type: TypedArrayType,
        length: usize,
        arraybuffer: Value,
        byte_offset: usize,
    ) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateTypedarray(
                self.as_raw(),
                array_type.as_raw(),
                length,
                arraybuffer.as_raw(),
                byte_offset,
                &mut raw,
            )
        })?;
        Value::from_raw(raw)
    }

    pub fn create_dataview(
        &self,
        byte_length: usize,
        arraybuffer: Value,
        byte_offset: usize,
    ) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateDataview(
                self.as_raw(),
                byte_length,
                arraybuffer.as_raw(),
                byte_offset,
                &mut raw,
            )
        })?;
        Value::from_raw(raw)
    }

    pub fn create_date(&self, time: f64) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateDate(self.as_raw(), time, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_bigint_int64(&self, value: i64) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateBigintInt64(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_bigint_uint64(&self, value: u64) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateBigintUint64(self.as_raw(), value, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_bigint_words(&self, sign_bit: i32, words: &[u64]) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateBigintWords(
                self.as_raw(),
                sign_bit,
                words.len(),
                if words.is_empty() {
                    std::ptr::null()
                } else {
                    words.as_ptr()
                },
                &mut raw,
            )
        })?;
        Value::from_raw(raw)
    }

    pub fn create_symbol(&self, description: Option<&str>) -> Result<Value> {
        let description = match description {
            Some(description) => self.create_string(description)?,
            None => self.undefined()?,
        };
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateSymbol(self.as_raw(), description.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub(crate) fn create_external_raw(
        &self,
        data: *mut c_void,
        finalize_cb: sys::JSVM_Finalize,
        finalize_hint: *mut c_void,
    ) -> Result<Value> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CreateExternal(self.as_raw(), data, finalize_cb, finalize_hint, &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn create_external<T: 'static>(&self, value: T) -> Result<Value> {
        let data = Box::into_raw(Box::new(value)).cast::<c_void>();
        self.create_external_raw(data, Some(drop_box_finalize::<T>), std::ptr::null_mut())
    }

    pub fn create_reference(&self, value: Value, initial_refcount: u32) -> Result<Reference> {
        Reference::new(self, value, initial_refcount)
    }

    pub fn create_object(&self) -> Result<Object> {
        Object::new(self)
    }

    pub fn create_function(&self, name: &str, callback: &'static Callback) -> Result<Function> {
        Function::create(self, name, callback)
    }

    pub fn json_parse(&self, json: &str) -> Result<Value> {
        let json = self.create_string(json)?;
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_JsonParse(self.as_raw(), json.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }

    pub fn json_stringify(&self, value: Value) -> Result<String> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_JsonStringify(self.as_raw(), value.as_raw(), &mut raw)
        })?;
        String::from_js_value(self, Value::from_raw(raw)?)
    }

    pub fn create_promise(&self) -> Result<(Deferred, Promise)> {
        create_promise(self)
    }

    pub fn is_exception_pending(&self) -> Result<bool> {
        let mut result = false;
        check_status(unsafe { sys::OH_JSVM_IsExceptionPending(self.as_raw(), &mut result) })?;
        Ok(result)
    }

    pub fn take_pending_exception_string(&self) -> Result<Option<String>> {
        let Some(value) = crate::pending_exception(self)? else {
            return Ok(None);
        };
        let message = value.coerce_to_string(self)?;
        String::from_js_value(self, message).map(Some)
    }

    pub fn version(&self) -> Result<u32> {
        let mut result = 0u32;
        check_status(unsafe { sys::OH_JSVM_GetVersion(self.as_raw(), &mut result) })?;
        Ok(result)
    }

    pub fn adjust_external_memory(&self, change_in_bytes: i64) -> Result<i64> {
        let mut result = 0i64;
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_AdjustExternalMemory(self.as_raw(), change_in_bytes, &mut result)
        })?;
        Ok(result)
    }

    pub fn is_locked(&self) -> Result<bool> {
        thread::is_locked(self)
    }

    pub fn acquire_lock(&self) -> Result<crate::EnvLockGuard<'_>> {
        crate::EnvLockGuard::acquire(self)
    }

    pub fn evaluate(&self, source: &str) -> Result<Value> {
        let script = self.create_string(source)?;
        let mut compiled = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_CompileScript(
                self.as_raw(),
                script.as_raw(),
                std::ptr::null(),
                0,
                false,
                std::ptr::null_mut(),
                &mut compiled,
            )
        })?;

        let mut result = std::ptr::null_mut();
        check_status_with_env(self, unsafe {
            sys::OH_JSVM_RunScript(self.as_raw(), compiled, &mut result)
        })?;
        Value::from_raw(result)
    }
}

impl Drop for Env {
    fn drop(&mut self) {
        if self.owned {
            let _ = unsafe { sys::OH_JSVM_DestroyEnv(self.as_raw()) };
        }
    }
}

#[derive(Debug)]
pub struct EnvScope<'a> {
    env: &'a Env,
    raw: std::ptr::NonNull<sys::JSVM_EnvScope__>,
}

impl<'a> EnvScope<'a> {
    pub fn as_raw(&self) -> sys::JSVM_EnvScope {
        self.raw.as_ptr()
    }
}

impl Drop for EnvScope<'_> {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_JSVM_CloseEnvScope(self.env.as_raw(), self.as_raw()) };
    }
}

#[derive(Debug)]
pub struct HandleScope<'a> {
    env: &'a Env,
    raw: std::ptr::NonNull<sys::JSVM_HandleScope__>,
}

impl<'a> HandleScope<'a> {
    pub fn as_raw(&self) -> sys::JSVM_HandleScope {
        self.raw.as_ptr()
    }
}

impl Drop for HandleScope<'_> {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_JSVM_CloseHandleScope(self.env.as_raw(), self.as_raw()) };
    }
}
