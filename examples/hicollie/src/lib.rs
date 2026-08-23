use napi_derive_ohos::napi;
use ohos_hicollie_binding as hicollie;

const TAG: &str = "HICOLLIE_TEST";

// Exercise report(): must run off the main thread per the header. We only
// verify the safe wrapper returns Ok/Err without panicking and log the outcome.
#[napi]
pub fn test_report() -> String {
    let r = std::thread::spawn(hicollie::report).join();
    let msg = match r {
        Ok(Ok(six_second)) => format!("report Ok(is_six_second={six_second})"),
        Ok(Err(e)) => format!("report Err({e})"),
        Err(_) => "report thread panicked".to_string(),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// deinit when nothing was initialized should surface an error, not panic.
#[napi]
pub fn test_deinit_jank() -> String {
    let msg = match hicollie::deinit_jank_detection() {
        Ok(()) => "deinit_jank Ok".to_string(),
        Err(e) => format!("deinit_jank Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
