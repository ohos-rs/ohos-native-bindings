use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_fileshare_binding::{
    activate_permission, check_persistent_permission, deactivate_permission, persist_permission,
    revoke_permission, PolicyInfo,
};

const READ_MODE: u32 = 1;

fn to_err(e: impl ToString) -> Error {
    Error::from_reason(e.to_string())
}

fn policy(uri: String) -> Vec<PolicyInfo> {
    vec![PolicyInfo {
        uri,
        operation_mode: READ_MODE,
    }]
}

fn fmt_errors(errors: &[ohos_fileshare_binding::PolicyErrorResult]) -> String {
    if errors.is_empty() {
        return "ok (no per-uri errors)".to_string();
    }
    errors
        .iter()
        .map(|e| format!("{} {:?} {}", e.uri, e.code, e.message))
        .collect::<Vec<_>>()
        .join("; ")
}

#[napi]
pub fn persist(uri: String) -> Result<String> {
    persist_permission(&policy(uri))
        .map(|e| fmt_errors(&e))
        .map_err(to_err)
}

#[napi]
pub fn revoke(uri: String) -> Result<String> {
    revoke_permission(&policy(uri))
        .map(|e| fmt_errors(&e))
        .map_err(to_err)
}

#[napi]
pub fn activate(uri: String) -> Result<String> {
    activate_permission(&policy(uri))
        .map(|e| fmt_errors(&e))
        .map_err(to_err)
}

#[napi]
pub fn deactivate(uri: String) -> Result<String> {
    deactivate_permission(&policy(uri))
        .map(|e| fmt_errors(&e))
        .map_err(to_err)
}

#[napi]
pub fn check_persistent(uri: String) -> Result<String> {
    check_persistent_permission(&policy(uri))
        .map(|flags| format!("{flags:?}"))
        .map_err(to_err)
}

#[napi]
pub fn smoke(path: String) -> Result<String> {
    let uri = ohos_fileuri_binding::get_uri_from_path(&path).map_err(to_err)?;
    let persist_r = match persist_permission(&policy(uri.clone())) {
        Ok(e) => format!("persist {}", fmt_errors(&e)),
        Err(e) => format!("persist ERR {e}"),
    };
    let check_r = match check_persistent_permission(&policy(uri.clone())) {
        Ok(flags) => format!("check {flags:?}"),
        Err(e) => format!("check ERR {e}"),
    };
    let activate_r = match activate_permission(&policy(uri.clone())) {
        Ok(e) => format!("activate {}", fmt_errors(&e)),
        Err(e) => format!("activate ERR {e}"),
    };
    let deactivate_r = match deactivate_permission(&policy(uri.clone())) {
        Ok(e) => format!("deactivate {}", fmt_errors(&e)),
        Err(e) => format!("deactivate ERR {e}"),
    };
    let revoke_r = match revoke_permission(&policy(uri.clone())) {
        Ok(e) => format!("revoke {}", fmt_errors(&e)),
        Err(e) => format!("revoke ERR {e}"),
    };
    Ok(format!(
        "uri={uri}\n{persist_r}\n{check_r}\n{activate_r}\n{deactivate_r}\n{revoke_r}"
    ))
}
