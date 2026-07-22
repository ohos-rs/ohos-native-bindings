use once_cell::sync::Lazy;

use crate::SysConfig;

/// HID DDK (`@library libhid.z.so`, `SystemCapability.Driver.HID.Extension`).
///
/// Two API generations share one library and one set of types, so they stay in one crate:
/// the virtual-device API (`OH_Hid_CreateDevice`/`EmitEvent`/`DestroyDevice`, `@since 11`)
/// and the host-side raw access API (`OH_Hid_Open`/`Read`/`Write`/..., `@since 18`), which
/// is gated behind `feature = "api-18"`.
///
/// Requires `ohos.permission.ACCESS_DDK_HID` (`system_grant` + `system_basic` +
/// `provisionEnable`), i.e. an ACL entry in the signing profile.
pub const HID: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hid-sys",
    headers: vec!["hid/hid_ddk_api.h", "hid/hid_ddk_types.h"],
    white_list: vec!["OH_Hid_.*", "Hid_.*", "HID_.*"],
    block_list: vec![],
    dynamic_library: vec!["hid.z"],
    extra: "",
});
