//! On-device checks for the single-shot query bindings.
//!
//! Every exported function calls the safe layer of one binding crate and turns
//! the outcome into a human readable string. Errors are reported as `err(..)`
//! text rather than propagated, so that a denied permission or a disabled
//! service is still a usable observation instead of a crash.

use napi_derive_ohos::napi;

use ohos_battery_info_binding as battery;
use ohos_bluetooth_binding as bluetooth;
use ohos_dlp_permission_binding as dlp;
use ohos_location_binding as location;
use ohos_notification_binding as notification;
use ohos_os_account_binding as os_account;
use ohos_time_service_binding as time_service;
use ohos_wifi_binding as wifi;

const TAG: &str = "SYSQUERY_TEST";

// Name of the OS account the process belongs to. The buffer is grown by the
// safe layer, so a long name is not an error here.
#[napi]
pub fn test_os_account_name() -> String {
    let msg = match os_account::name() {
        Ok(name) => format!("account_name={name}"),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Current system time zone ID, for example `Asia/Shanghai`.
#[napi]
pub fn test_time_zone() -> String {
    let msg = match time_service::get_time_zone() {
        Ok(zone) => format!("timezone={zone}"),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Battery level plus the power source the device is plugged into. Both values
// are reported, and each one falls back to its own error text.
#[napi]
pub fn test_battery_info() -> String {
    let capacity = match battery::capacity() {
        Ok(percent) => format!("capacity={percent}"),
        Err(e) => format!("capacity=err({e})"),
    };
    let plugged = match battery::plugged_type() {
        Ok(source) => format!("plugged={source}"),
        Err(e) => format!("plugged=err({e})"),
    };
    let msg = format!("{capacity} {plugged}");
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Whether the user currently allows this application to post notifications.
// The native call reports a denied permission and a failed query alike as
// `false`, so there is no error case to map.
#[napi]
pub fn test_notification_enabled() -> String {
    let msg = format!(
        "notification_enabled={}",
        notification::is_notification_enabled()
    );
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// State of the local bluetooth switch, together with the convenience predicate
// derived from it.
#[napi]
pub fn test_bluetooth_state() -> String {
    let msg = match bluetooth::switch_state() {
        Ok(state) => format!("bt_state={state} bt_enabled={}", state.is_on()),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Whether the wifi switch of the device is turned on. This query declares no
// permission.
#[napi]
pub fn test_wifi_enabled() -> String {
    let msg = match wifi::is_wifi_enabled() {
        Ok(enabled) => format!("wifi_enabled={enabled}"),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Whether the system location switch is on. Reading the switch needs no
// location permission, unlike starting a locating session.
#[napi]
pub fn test_locating_enabled() -> String {
    let msg = match location::is_locating_enabled() {
        Ok(enabled) => format!("locating_enabled={enabled}"),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

// Whether this application runs inside a DLP sandbox. A normal application is
// expected to answer `false`.
#[napi]
pub fn test_dlp_in_sandbox() -> String {
    let msg = match dlp::is_in_sandbox() {
        Ok(in_sandbox) => format!("in_sandbox={in_sandbox}"),
        Err(e) => format!("err({e})"),
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
