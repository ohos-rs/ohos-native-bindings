use napi_derive_ohos::napi;
use ohos_hilog_binding::{hilog_info, hilog_warn};

#[napi]
pub fn hi_log_info() {
    hilog_info!("hello rust");
}

#[napi]
pub fn hi_log_warn() {
    hilog_warn!("hello rust");
}
