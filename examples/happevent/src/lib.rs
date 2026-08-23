use napi_derive_ohos::napi;
use ohos_hiappevent_binding::EventParams;
const TAG: &str = "BINDTEST_HIAPPEVENT";

#[napi]
pub fn test_hiappevent() -> String {
    // Build params via the collapsed typed `add`; a tag-type mismatch is rejected
    // at build(). Here everything matches, so build should succeed.
    let params = EventParams::builder()
        .add("count", 42_i64)
        .add("name", "unit")
        .add("ratio", 1.5_f64)
        .build();
    let msg = match params {
        Ok(_) => "build Ok (typed add + build succeeded)".to_string(),
        Err(e) => format!("build Err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
