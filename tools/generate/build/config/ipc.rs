use once_cell::sync::Lazy;

use crate::SysConfig;

pub const IPC: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-ipc-sys",
    // All four headers share `OHIPCParcel` / `OHIPCRemoteProxy` / `OHIPCRemoteStub`,
    // so they have to live in one crate.
    headers: vec![
        "IPCKit/ipc_error_code.h",
        "IPCKit/ipc_cparcel.h",
        "IPCKit/ipc_cremote_object.h",
        "IPCKit/ipc_cskeleton.h",
    ],
    white_list: vec![
        // Functions, error codes, `OH_IPC_MemAllocator`, `OH_IPC_MessageOption`,
        // `OH_IPC_RequestMode`.
        "OH_IPC.*",
        // Opaque handles are spelled without the separating underscore.
        "OHIPC.*",
        "OH_OnRemote.*",
        "OH_OnDeathRecipient.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ipc_capi"],
    extra: "",
});
