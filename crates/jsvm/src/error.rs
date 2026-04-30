use std::ffi::CString;
use std::fmt;
use std::ptr::NonNull;

use ohos_jsvm_sys as sys;

use crate::{Env, Value, ValueType};

pub type Result<T> = std::result::Result<T, JsvmError>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum JsvmError {
    Status(sys::JSVM_Status),
    PendingException,
    NullPointer(&'static str),
    TypeMismatch {
        expected: &'static str,
        actual: Option<ValueType>,
    },
    InvalidString,
    IntegerOverflow(&'static str),
}

impl fmt::Display for JsvmError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Status(status) => {
                write!(f, "jsvm status error: {} ({status})", status_name(*status))
            }
            Self::PendingException => f.write_str("jsvm has a pending exception"),
            Self::NullPointer(name) => write!(f, "jsvm returned a null pointer for {name}"),
            Self::TypeMismatch { expected, actual } => match actual {
                Some(actual) => write!(f, "type mismatch: expected {expected}, got {actual}"),
                None => write!(f, "type mismatch: expected {expected}"),
            },
            Self::InvalidString => f.write_str("string contains an interior NUL byte"),
            Self::IntegerOverflow(target) => write!(f, "integer conversion overflow for {target}"),
        }
    }
}

impl std::error::Error for JsvmError {}

pub fn type_mismatch(expected: &'static str, actual: Option<ValueType>) -> JsvmError {
    JsvmError::TypeMismatch { expected, actual }
}

pub fn check_status(status: sys::JSVM_Status) -> Result<()> {
    if status == sys::JSVM_Status_JSVM_OK {
        Ok(())
    } else {
        Err(JsvmError::Status(status))
    }
}

pub fn check_status_with_env(env: &Env, status: sys::JSVM_Status) -> Result<()> {
    if status == sys::JSVM_Status_JSVM_OK {
        if env.is_exception_pending()? {
            return Err(JsvmError::PendingException);
        }

        return Ok(());
    }

    if status == sys::JSVM_Status_JSVM_PENDING_EXCEPTION || env.is_exception_pending()? {
        return Err(JsvmError::PendingException);
    }

    Err(JsvmError::Status(status))
}

pub fn pending_exception(env: &Env) -> Result<Option<Value>> {
    if !env.is_exception_pending()? {
        return Ok(None);
    }

    let mut raw = std::ptr::null_mut();
    check_status(unsafe { sys::OH_JSVM_GetAndClearLastException(env.as_raw(), &mut raw) })?;
    Ok(Some(Value::from_raw(raw)?))
}

pub fn throw_value(env: &Env, value: Value) -> Result<()> {
    check_status_with_env(env, unsafe {
        sys::OH_JSVM_Throw(env.as_raw(), value.as_raw())
    })
}

pub fn throw_error(env: &Env, code: Option<&str>, message: &str) -> Result<()> {
    let code = make_c_string(code)?;
    let message = CString::new(message).map_err(|_| JsvmError::InvalidString)?;

    check_status_with_env(env, unsafe {
        sys::OH_JSVM_ThrowError(
            env.as_raw(),
            code.as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
            message.as_ptr(),
        )
    })
}

pub fn throw_type_error(env: &Env, code: Option<&str>, message: &str) -> Result<()> {
    let code = make_c_string(code)?;
    let message = CString::new(message).map_err(|_| JsvmError::InvalidString)?;

    check_status_with_env(env, unsafe {
        sys::OH_JSVM_ThrowTypeError(
            env.as_raw(),
            code.as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
            message.as_ptr(),
        )
    })
}

pub fn throw_range_error(env: &Env, code: Option<&str>, message: &str) -> Result<()> {
    let code = make_c_string(code)?;
    let message = CString::new(message).map_err(|_| JsvmError::InvalidString)?;

    check_status_with_env(env, unsafe {
        sys::OH_JSVM_ThrowRangeError(
            env.as_raw(),
            code.as_ref()
                .map_or(std::ptr::null(), |value| value.as_ptr()),
            message.as_ptr(),
        )
    })
}

pub(crate) fn non_null<T>(raw: *mut T, name: &'static str) -> Result<NonNull<T>> {
    NonNull::new(raw).ok_or(JsvmError::NullPointer(name))
}

fn make_c_string(value: Option<&str>) -> Result<Option<CString>> {
    value
        .map(|value| CString::new(value).map_err(|_| JsvmError::InvalidString))
        .transpose()
}

fn status_name(status: sys::JSVM_Status) -> &'static str {
    match status {
        sys::JSVM_Status_JSVM_OK => "ok",
        sys::JSVM_Status_JSVM_INVALID_ARG => "invalid argument",
        sys::JSVM_Status_JSVM_OBJECT_EXPECTED => "object expected",
        sys::JSVM_Status_JSVM_STRING_EXPECTED => "string expected",
        sys::JSVM_Status_JSVM_NAME_EXPECTED => "name expected",
        sys::JSVM_Status_JSVM_FUNCTION_EXPECTED => "function expected",
        sys::JSVM_Status_JSVM_NUMBER_EXPECTED => "number expected",
        sys::JSVM_Status_JSVM_BOOLEAN_EXPECTED => "boolean expected",
        sys::JSVM_Status_JSVM_ARRAY_EXPECTED => "array expected",
        sys::JSVM_Status_JSVM_GENERIC_FAILURE => "generic failure",
        sys::JSVM_Status_JSVM_PENDING_EXCEPTION => "pending exception",
        sys::JSVM_Status_JSVM_CANCELLED => "cancelled",
        sys::JSVM_Status_JSVM_ESCAPE_CALLED_TWICE => "escape called twice",
        sys::JSVM_Status_JSVM_HANDLE_SCOPE_MISMATCH => "handle scope mismatch",
        sys::JSVM_Status_JSVM_CALLBACK_SCOPE_MISMATCH => "callback scope mismatch",
        sys::JSVM_Status_JSVM_QUEUE_FULL => "queue full",
        sys::JSVM_Status_JSVM_CLOSING => "closing",
        sys::JSVM_Status_JSVM_BIGINT_EXPECTED => "bigint expected",
        sys::JSVM_Status_JSVM_DATE_EXPECTED => "date expected",
        sys::JSVM_Status_JSVM_ARRAYBUFFER_EXPECTED => "arraybuffer expected",
        sys::JSVM_Status_JSVM_DETACHABLE_ARRAYBUFFER_EXPECTED => "detachable arraybuffer expected",
        sys::JSVM_Status_JSVM_WOULD_DEADLOCK => "would deadlock",
        sys::JSVM_Status_JSVM_NO_EXTERNAL_BUFFERS_ALLOWED => "no external buffers allowed",
        sys::JSVM_Status_JSVM_CANNOT_RUN_JS => "cannot run js",
        _ => "unknown status",
    }
}
