use once_cell::sync::Lazy;

use crate::SysConfig;

// NotificationKit exposes a single NDK entry point. The header transitively pulls in
// info/application_target_sdk_version.h, whose symbols live in libc rather than
// libohnotification, so the allow list stays pinned to the OH_Notification_ prefix.

pub const NOTIFICATION: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-notification-sys",
    headers: vec!["NotificationKit/notification.h"],
    white_list: vec!["OH_Notification_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohnotification"],
    extra: "",
});
