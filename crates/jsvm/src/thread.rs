use ohos_jsvm_sys as sys;

use crate::error::{check_status, check_status_with_env, Result};
use crate::Env;

#[derive(Debug)]
pub struct EnvLockGuard<'a> {
    env: &'a Env,
}

impl<'a> EnvLockGuard<'a> {
    pub(crate) fn acquire(env: &'a Env) -> Result<Self> {
        check_status_with_env(env, unsafe { sys::OH_JSVM_AcquireLock(env.as_raw()) })?;
        Ok(Self { env })
    }
}

impl Drop for EnvLockGuard<'_> {
    fn drop(&mut self) {
        let _ = unsafe { sys::OH_JSVM_ReleaseLock(self.env.as_raw()) };
    }
}

pub(crate) fn is_locked(env: &Env) -> Result<bool> {
    let mut result = false;
    check_status(unsafe { sys::OH_JSVM_IsLocked(env.as_raw(), &mut result) })?;
    Ok(result)
}
