use napi_derive_ohos::napi;
use ohos_ability_access_control_binding::check_self_permission;

fn check_one(name: &str) -> String {
    format!("{name}={}", check_self_permission(name))
}

/// Check a single HarmonyOS permission on this process.
#[napi]
pub fn check_permission(name: String) -> bool {
    check_self_permission(name)
}

/// Check the permissions this demo actually declares / uses.
#[napi]
pub fn smoke() -> String {
    [
        check_one("ohos.permission.INTERNET"),
        check_one("ohos.permission.VIBRATE"),
        check_one("ohos.permission.ACCELEROMETER"),
        check_one("ohos.permission.CAMERA"),
        check_one("ohos.permission.GYROSCOPE"),
    ]
    .join("\n")
}
