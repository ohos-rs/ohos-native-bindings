use once_cell::sync::Lazy;

use crate::SysConfig;

/// Device Certificate Kit / Certificate Manager (`SystemCapability.Security.CertificateManager`).
///
/// Both headers declare the same `@library` and `cm_native_api.h` includes `cm_native_type.h`,
/// so they form a single sys crate.
///
/// The header's `@library` reads `libohcert_manager.so`, but the only file shipped in the
/// sysroot is `libohcert_manager.z.so`, hence the `.z` link name.
///
/// Every symbol is `@since 22`, so the whole crate is gated behind `feature = "api-22"`.
///
/// `cm_native_api.h` also pulls in `info/application_target_sdk_version.h`, which defines
/// `OH_API_VERSION_*`, `OH_CURRENT_API_VERSION` and `get_application_target_sdk_version`.
/// The allowlist is therefore anchored on the two real prefixes rather than a broad `OH_.*`,
/// which would drag those SDK-version macros in.
pub const DEVICE_CERTIFICATE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-cert-manager-sys",
    headers: vec![
        "device_certificate/certmanager/cm_native_api.h",
        "device_certificate/certmanager/cm_native_type.h",
    ],
    white_list: vec!["OH_CertManager_.*", "OH_CM_.*"],
    block_list: vec![],
    dynamic_library: vec!["ohcert_manager.z"],
    extra: "",
});
