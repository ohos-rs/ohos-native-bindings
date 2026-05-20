use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, JsvmError, Result};
use crate::{Env, FromJsValue, ToJsValue, Value};

impl ToJsValue for serde_json::Value {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let json = serde_json::to_string(self).map_err(|_| JsvmError::InvalidString)?;
        let json = env.create_string(&json)?;
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_JsonParse(env.as_raw(), json.as_raw(), &mut raw)
        })?;
        Value::from_raw(raw)
    }
}

impl FromJsValue for serde_json::Value {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let mut raw = std::ptr::null_mut();
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_JsonStringify(env.as_raw(), value.as_raw(), &mut raw)
        })?;
        let json = String::from_js_value(env, Value::from_raw(raw)?)?;
        serde_json::from_str(&json).map_err(|_| JsvmError::InvalidString)
    }
}
