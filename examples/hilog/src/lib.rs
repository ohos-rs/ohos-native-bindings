use napi_derive_ohos::napi;

#[napi]
pub fn log() {
    let _ = ohos_hilog_binding::forward_stdio_to_hilog();
    println!("hello");
    ohos_hilog_binding::hilog_info!("hello");
}
