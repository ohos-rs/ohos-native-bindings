use once_cell::sync::Lazy;

use super::SysConfig;

/// hicollie: business-thread stuck / jank detection and timeout watchdog
/// (`@library libohhicollie.so`, `@kit PerformanceAnalysisKit`).
///
/// Single shared object, single header; no permission is required.
pub const HICOLLIE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-hicollie-sys",
    headers: vec!["hicollie/hicollie.h"],
    white_list: vec![
        // functions, callback typedefs, freeze-type enum
        "OH_HiCollie.*",
        // structs and enum types
        "HiCollie_.*",
        // error codes and flag constants
        "HICOLLIE_.*",
        // error code that breaks the `HICOLLIE_` casing convention
        "OH_HICOLLIE_.*",
        // OH_HiCollie_Freeze_Type variants (they carry no type prefix)
        "OH_THREAD_BLOCK_.*",
        "OH_LIFECYCLE_.*",
        "OH_APP_INPUT_BLOCK",
        "OH_BUSINESS_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohhicollie"],
    extra: "",
});
