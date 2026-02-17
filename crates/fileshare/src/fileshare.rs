use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_uint, c_void};
use std::ptr;

use ohos_fileshare_sys::*;

use crate::error::{error_from_code, FileShareError, PolicyErrorCode};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyInfo {
    pub uri: String,
    pub operation_mode: u32,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PolicyErrorResult {
    pub uri: String,
    pub code: PolicyErrorCode,
    pub message: String,
}

type PolicyApi = unsafe extern "C" fn(
    policies: *const FileShare_PolicyInfo,
    policy_num: c_uint,
    result: *mut *mut FileShare_PolicyErrorResult,
    result_num: *mut c_uint,
) -> FileManagement_ErrCode;

pub fn persist_permission(
    policies: &[PolicyInfo],
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    call_policy_api(policies, OH_FileShare_PersistPermission)
}

pub fn revoke_permission(
    policies: &[PolicyInfo],
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    call_policy_api(policies, OH_FileShare_RevokePermission)
}

pub fn activate_permission(
    policies: &[PolicyInfo],
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    call_policy_api(policies, OH_FileShare_ActivatePermission)
}

pub fn deactivate_permission(
    policies: &[PolicyInfo],
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    call_policy_api(policies, OH_FileShare_DeactivatePermission)
}

pub fn check_persistent_permission(policies: &[PolicyInfo]) -> Result<Vec<bool>, FileShareError> {
    let (cstring_uris, raw_policies) = build_raw_policies(policies)?;
    let mut result_ptr: *mut bool = ptr::null_mut();
    let mut result_num: c_uint = 0;

    let code = unsafe {
        OH_FileShare_CheckPersistentPermission(
            raw_policies.as_ptr(),
            raw_policies.len() as c_uint,
            &mut result_ptr,
            &mut result_num,
        )
    };

    let parsed = if result_ptr.is_null() || result_num == 0 {
        Vec::new()
    } else {
        let values = unsafe { std::slice::from_raw_parts(result_ptr, result_num as usize) };
        values.to_vec()
    };

    if !result_ptr.is_null() {
        unsafe { libc::free(result_ptr.cast::<c_void>()) };
    }

    drop(cstring_uris);

    if code == FileManagement_ErrCode_ERR_OK {
        Ok(parsed)
    } else {
        Err(error_from_code(code))
    }
}

fn call_policy_api(
    policies: &[PolicyInfo],
    api: PolicyApi,
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    let (cstring_uris, raw_policies) = build_raw_policies(policies)?;
    let mut result_ptr: *mut FileShare_PolicyErrorResult = ptr::null_mut();
    let mut result_num: c_uint = 0;

    let code = unsafe {
        api(
            raw_policies.as_ptr(),
            raw_policies.len() as c_uint,
            &mut result_ptr,
            &mut result_num,
        )
    };

    let parsed = parse_policy_error_results(result_ptr, result_num)?;

    if !result_ptr.is_null() {
        unsafe { OH_FileShare_ReleasePolicyErrorResult(result_ptr, result_num) };
    }

    drop(cstring_uris);

    if code == FileManagement_ErrCode_ERR_OK {
        Ok(parsed)
    } else {
        Err(error_from_code(code))
    }
}

fn build_raw_policies(
    policies: &[PolicyInfo],
) -> Result<(Vec<CString>, Vec<FileShare_PolicyInfo>), FileShareError> {
    let mut cstring_uris = Vec::with_capacity(policies.len());
    for policy in policies {
        cstring_uris
            .push(CString::new(policy.uri.as_str()).map_err(|_| FileShareError::NullByteError)?);
    }

    let mut raw_policies = Vec::with_capacity(policies.len());
    for (policy, c_uri) in policies.iter().zip(cstring_uris.iter()) {
        raw_policies.push(FileShare_PolicyInfo {
            uri: c_uri.as_ptr().cast_mut(),
            length: policy.uri.len() as c_uint,
            operationMode: policy.operation_mode as c_uint,
        });
    }

    Ok((cstring_uris, raw_policies))
}

fn parse_policy_error_results(
    ptr: *mut FileShare_PolicyErrorResult,
    len: c_uint,
) -> Result<Vec<PolicyErrorResult>, FileShareError> {
    if ptr.is_null() || len == 0 {
        return Ok(Vec::new());
    }

    let raw_results = unsafe { std::slice::from_raw_parts(ptr, len as usize) };
    let mut results = Vec::with_capacity(raw_results.len());

    for item in raw_results {
        let uri = cstr_ptr_to_string(item.uri.cast_const())?;
        let message = cstr_ptr_to_string(item.message.cast_const())?;
        results.push(PolicyErrorResult {
            uri,
            code: PolicyErrorCode::from(item.code),
            message,
        });
    }

    Ok(results)
}

fn cstr_ptr_to_string(ptr: *const c_char) -> Result<String, FileShareError> {
    if ptr.is_null() {
        return Ok(String::new());
    }
    let c_str = unsafe { CStr::from_ptr(ptr) };
    c_str
        .to_str()
        .map(|s| s.to_string())
        .map_err(|_| FileShareError::ConversionError)
}
