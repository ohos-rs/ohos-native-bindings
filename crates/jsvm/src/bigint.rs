use ohos_jsvm_sys as sys;

use crate::error::{check_status_with_env, type_mismatch, JsvmError, Result};
use crate::{Env, FromJsValue, ToJsValue, Value, ValueType};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct BigInt {
    value: Value,
}

impl BigInt {
    pub fn from_value(env: &Env, value: Value) -> Result<Self> {
        match value.value_type(env)? {
            ValueType::BigInt => Ok(Self { value }),
            other => Err(type_mismatch("bigint", Some(other))),
        }
    }

    pub fn as_value(self) -> Value {
        self.value
    }

    pub fn get_i64(self, env: &Env) -> Result<(i64, bool)> {
        let mut result = 0i64;
        let mut lossless = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueBigintInt64(
                env.as_raw(),
                self.value.as_raw(),
                &mut result,
                &mut lossless,
            )
        })?;
        Ok((result, lossless))
    }

    pub fn get_u64(self, env: &Env) -> Result<(u64, bool)> {
        let mut result = 0u64;
        let mut lossless = false;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueBigintUint64(
                env.as_raw(),
                self.value.as_raw(),
                &mut result,
                &mut lossless,
            )
        })?;
        Ok((result, lossless))
    }

    pub fn words(self, env: &Env) -> Result<(bool, Vec<u64>)> {
        let mut word_count = 0usize;
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueBigintWords(
                env.as_raw(),
                self.value.as_raw(),
                std::ptr::null_mut(),
                &mut word_count,
                std::ptr::null_mut(),
            )
        })?;

        let mut sign_bit = 0;
        let mut words = vec![0u64; word_count];
        check_status_with_env(env, unsafe {
            sys::OH_JSVM_GetValueBigintWords(
                env.as_raw(),
                self.value.as_raw(),
                &mut sign_bit,
                &mut word_count,
                words.as_mut_ptr(),
            )
        })?;
        words.truncate(word_count);
        Ok((sign_bit != 0, words))
    }
}

impl ToJsValue for BigInt {
    fn to_js_value(&self, _env: &Env) -> Result<Value> {
        Ok(self.value)
    }
}

impl FromJsValue for BigInt {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        Self::from_value(env, value)
    }
}

impl ToJsValue for i128 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let sign_bit = i32::from(*self < 0);
        let magnitude = self.unsigned_abs();
        let words = [
            (magnitude & u64::MAX as u128) as u64,
            (magnitude >> 64) as u64,
        ];
        env.create_bigint_words(sign_bit, trim_words(&words))
    }
}

impl FromJsValue for i128 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let (sign_bit, words) = BigInt::from_value(env, value)?.words(env)?;
        if words.len() > 2 {
            return Err(JsvmError::IntegerOverflow("i128"));
        }
        let magnitude = words_to_u128(&words);
        if sign_bit {
            if magnitude > (i128::MAX as u128) + 1 {
                return Err(JsvmError::IntegerOverflow("i128"));
            }
            if magnitude == (i128::MAX as u128) + 1 {
                return Ok(i128::MIN);
            }
            Ok(-(magnitude as i128))
        } else {
            i128::try_from(magnitude).map_err(|_| JsvmError::IntegerOverflow("i128"))
        }
    }
}

impl ToJsValue for u128 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        let words = [(*self & u64::MAX as u128) as u64, (*self >> 64) as u64];
        env.create_bigint_words(0, trim_words(&words))
    }
}

impl FromJsValue for u128 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let (sign_bit, words) = BigInt::from_value(env, value)?.words(env)?;
        if sign_bit || words.len() > 2 {
            return Err(JsvmError::IntegerOverflow("u128"));
        }
        Ok(words_to_u128(&words))
    }
}

impl ToJsValue for u64 {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        env.create_bigint_uint64(*self)
    }
}

impl FromJsValue for u64 {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        let (value, lossless) = BigInt::from_value(env, value)?.get_u64(env)?;
        if lossless {
            Ok(value)
        } else {
            Err(JsvmError::IntegerOverflow("u64"))
        }
    }
}

impl ToJsValue for usize {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        u64::try_from(*self)
            .map_err(|_| JsvmError::IntegerOverflow("usize"))?
            .to_js_value(env)
    }
}

impl FromJsValue for usize {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        usize::try_from(u64::from_js_value(env, value)?)
            .map_err(|_| JsvmError::IntegerOverflow("usize"))
    }
}

impl ToJsValue for isize {
    fn to_js_value(&self, env: &Env) -> Result<Value> {
        i64::try_from(*self)
            .map_err(|_| JsvmError::IntegerOverflow("isize"))?
            .to_js_value(env)
    }
}

impl FromJsValue for isize {
    fn from_js_value(env: &Env, value: Value) -> Result<Self> {
        isize::try_from(i64::from_js_value(env, value)?)
            .map_err(|_| JsvmError::IntegerOverflow("isize"))
    }
}

fn trim_words(words: &[u64; 2]) -> &[u64] {
    if words[1] == 0 {
        &words[..1]
    } else {
        &words[..]
    }
}

fn words_to_u128(words: &[u64]) -> u128 {
    let low = words.first().copied().unwrap_or(0) as u128;
    let high = words.get(1).copied().unwrap_or(0) as u128;
    low | (high << 64)
}
