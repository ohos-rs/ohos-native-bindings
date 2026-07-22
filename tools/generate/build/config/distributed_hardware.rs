use once_cell::sync::Lazy;

use crate::SysConfig;

/// Distributed device management (`@library libdevicemanager_ndk.so`,
/// `@kit DistributedServiceKit`, `SystemCapability.DistributedHardware.DeviceManager`).
///
/// The whole `distributedhardware` include tree currently ships one module, `device_manager`,
/// with a single function: `OH_DeviceManager_GetLocalDeviceName`. Both headers declare the same
/// `@library`, and `oh_device_manager.h` does not include the error-code header, so both are
/// listed here to keep `DeviceManager_ErrorCode` in the same sys crate as the function returning
/// it.
///
/// Every symbol is `@since 20`, so the whole crate sits behind `feature = "api-20"`.
///
/// `oh_device_manager.h` declares `len` as a C++ reference (`unsigned int &len`) inside an
/// `extern "C"` block, so the header only parses as C++ — which the generator already forces via
/// `-x c++`. bindgen lowers the reference to a raw pointer, matching the ABI.
///
/// `oh_device_manager.h` also pulls in `info/application_target_sdk_version.h`, which defines
/// `SDK_VERSION_*`, `OH_API_VERSION_*` and `get/set_application_target_sdk_version`. The
/// allowlist is anchored on the module's own prefixes so none of those leak in.
///
/// `DeviceManager_ErrorCode` variants carry no type prefix, so they are allowlisted by name
/// rather than through the enum type. `ERR_OK` / `ERR_PERMISSION_ERROR` / `ERR_INVALID_PARAMETER`
/// are listed literally instead of as `ERR_.*` to keep unrelated error constants out.
pub const DEVICE_MANAGER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-device-manager-sys",
    headers: vec![
        "distributedhardware/device_manager/oh_device_manager.h",
        "distributedhardware/device_manager/oh_device_manager_err_code.h",
    ],
    white_list: vec![
        "OH_DeviceManager_.*",
        "DeviceManager_.*",
        "DM_ERR_.*",
        "ERR_OK",
        "ERR_PERMISSION_ERROR",
        "ERR_INVALID_PARAMETER",
    ],
    block_list: vec![],
    dynamic_library: vec!["devicemanager_ndk"],
    extra: "",
});
