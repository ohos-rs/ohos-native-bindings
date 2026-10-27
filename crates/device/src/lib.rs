use std::ffi::CStr;

fn c_str_to_static(ptr: *const std::os::raw::c_char) -> Option<&'static str> {
    if ptr.is_null() {
        return None;
    }
    unsafe { CStr::from_ptr(ptr) }.to_str().ok()
}

/// Obtains the device type (`OH_GetDeviceType`), such as `"phone"`, `"wearable"`,
/// `"liteWearable"`, `"tablet"`, `"tv"`, `"car"`, `"smartVision"`, or `"2in1"`.
pub fn device_type() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetDeviceType() })
}

/// Obtains the device manufacturer (`OH_GetManufacture`).
pub fn manufacture() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetManufacture() })
}

/// Obtains the device brand (`OH_GetBrand`).
pub fn brand() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBrand() })
}

/// Obtains the market name (`OH_GetMarketName`).
pub fn market_name() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetMarketName() })
}

/// Obtains the product series (`OH_GetProductSeries`).
pub fn product_series() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetProductSeries() })
}

/// Obtains the product model (`OH_GetProductModel`).
pub fn product_model() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetProductModel() })
}

/// Obtains the software model (`OH_GetSoftwareModel`).
pub fn software_model() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetSoftwareModel() })
}

/// Obtains the hardware model (`OH_GetHardwareModel`).
pub fn hardware_model() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetHardwareModel() })
}

/// Obtains the Bootloader version (`OH_GetBootloaderVersion`).
pub fn bootloader_version() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBootloaderVersion() })
}

/// Obtains the ABI list (`OH_GetAbiList`).
pub fn abi_list() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetAbiList() })
}

/// Obtains the security patch tag (`OH_GetSecurityPatchTag`).
pub fn security_patch_tag() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetSecurityPatchTag() })
}

/// Obtains the display version (`OH_GetDisplayVersion`).
pub fn display_version() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetDisplayVersion() })
}

/// Obtains the incremental version (`OH_GetIncrementalVersion`).
pub fn incremental_version() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetIncrementalVersion() })
}

/// Obtains the OS release type (`OH_GetOsReleaseType`), such as `"Release"`, `"Beta"`, or `"Canary"`.
pub fn os_release_type() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetOsReleaseType() })
}

/// Obtains the OS full name (`OH_GetOSFullName`).
pub fn os_full_name() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetOSFullName() })
}

/// Obtains the SDK API version (`OH_GetSdkApiVersion`).
pub fn sdk_api_version() -> u32 {
    unsafe { ohos_device_sys::OH_GetSdkApiVersion().max(0) as u32 }
}

/// Obtains the first API version (`OH_GetFirstApiVersion`).
pub fn first_api_version() -> u32 {
    unsafe { ohos_device_sys::OH_GetFirstApiVersion().max(0) as u32 }
}

/// Obtains the version ID (`OH_GetVersionId`).
pub fn version_id() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetVersionId() })
}

/// Obtains the build type (`OH_GetBuildType`).
pub fn build_type() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBuildType() })
}

/// Obtains the build user (`OH_GetBuildUser`).
pub fn build_user() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBuildUser() })
}

/// Obtains the build host (`OH_GetBuildHost`).
pub fn build_host() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBuildHost() })
}

/// Obtains the build time (`OH_GetBuildTime`).
pub fn build_time() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBuildTime() })
}

/// Obtains the build root hash (`OH_GetBuildRootHash`).
pub fn build_root_hash() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetBuildRootHash() })
}

/// Obtains the ISV distribution OS name (`OH_GetDistributionOSName`).
pub fn distribution_os_name() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetDistributionOSName() })
}

/// Obtains the ISV distribution OS version (`OH_GetDistributionOSVersion`).
pub fn distribution_os_version() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetDistributionOSVersion() })
}

/// Obtains the ISV distribution OS API version (`OH_GetDistributionOSApiVersion`).
pub fn distribution_os_api_version() -> u32 {
    unsafe { ohos_device_sys::OH_GetDistributionOSApiVersion().max(0) as u32 }
}

/// Obtains the ISV distribution OS release type (`OH_GetDistributionOSReleaseType`).
pub fn distribution_os_release_type() -> Option<&'static str> {
    c_str_to_static(unsafe { ohos_device_sys::OH_GetDistributionOSReleaseType() })
}
