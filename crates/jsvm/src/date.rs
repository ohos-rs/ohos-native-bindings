use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, type_mismatch, Result};
use crate::{Env, FromJsValue, ToJsValue, Value};

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Date {
    value: Value,
}

impl Date {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        if value.is_date(env)? {
            Ok(Self { value })
        } else {
            Err(type_mismatch("Date", Some(value.value_type(env)?)))
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn time(self, env: &Env) -> Result<f64> {
        let mut result = 0f64;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetDateValue(env.as_raw(), self.value.as_raw(), &mut result)
        })?;
        Ok(result)
    }
}

impl ToJsValue for Date {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(self.value)
    }
}

impl FromJsValue for Date {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Self::from_value(env, value)
    }
}
