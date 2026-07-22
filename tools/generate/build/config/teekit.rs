use once_cell::sync::Lazy;

use crate::SysConfig;

/// TEE Client API (`SystemCapability.Tee.TeeClient`) — the REE-side half of TEEKit.
///
/// TEEKit ships two disjoint header sets. Only `tee_client/` is bindable here:
/// it declares `@library libteec.so`, which is present in the sysroot for all three
/// targets. The 16 headers under `tee/` are the GlobalPlatform TEE Internal Core API,
/// they all declare `@library NA` and ship no import library — that code is compiled
/// into a trusted application running in the secure world, not into a normal binary.
///
/// The three headers form one crate: `tee_client_api.h` includes `tee_client_type.h`,
/// which includes `tee_client_constants.h`, and they share every `TEEC_*` type.
///
/// Every symbol is `@since 20`, so the whole crate ends up behind `feature = "api-20"`.
///
/// `ListNode` is allowlisted explicitly because it is the only public type in these
/// headers without the `TEEC_` prefix: it is the intrusive list node embedded in
/// `TEEC_Context`, `TEEC_Session` and `TEEC_SharedMemory`.
pub const TEE_CLIENT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-tee-client-sys",
    headers: vec![
        "TEEKit/tee_client/tee_client_api.h",
        "TEEKit/tee_client/tee_client_type.h",
        "TEEKit/tee_client/tee_client_constants.h",
    ],
    white_list: vec!["TEEC_.*", "ListNode"],
    block_list: vec![],
    dynamic_library: vec!["teec"],
    extra: "",
});
