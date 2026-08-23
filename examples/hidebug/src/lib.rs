use napi_derive_ohos::napi;
use ohos_hidebug_binding as hidebug;
const TAG: &str = "BINDTEST_HIDEBUG";

#[napi]
pub fn test_hidebug() -> String {
    let sys = hidebug::system_cpu_usage();
    let app = hidebug::app_cpu_usage();
    // valid usages are fractions in [0,1]; just confirm the FFI returns sane numbers.
    let ok = (0.0..=1.0).contains(&sys) && (0.0..=1.0).contains(&app);
    let msg = format!("system_cpu={sys:.4} app_cpu={app:.4} sane={ok}");
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
