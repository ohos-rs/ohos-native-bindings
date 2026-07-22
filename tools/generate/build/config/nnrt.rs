use once_cell::sync::Lazy;

use crate::SysConfig;

pub const NNRT: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-nnrt-sys",
    // Two @library values (libneural_network_core.so / libneural_network_runtime.so) but a
    // single type universe: neural_network_runtime.h includes neural_network_core.h, and both
    // sides pass OH_NNExecutor/OH_NN_ReturnCode/NN_Tensor declared in the shared type header.
    // Splitting per library would emit two incompatible copies of those types, so one crate
    // links both.
    headers: vec![
        "neural_network_runtime/neural_network_runtime_type.h",
        "neural_network_runtime/neural_network_core.h",
        "neural_network_runtime/neural_network_runtime.h",
    ],
    white_list: vec![
        // Handles (OH_NNModel, OH_NNCompilation, OH_NNExecutor), enums (OH_NN_ReturnCode,
        // OH_NN_DataType, OH_NN_OperationType, ...), plain structs (OH_NN_Tensor, OH_NN_Memory,
        // OH_NN_QuantParam, OH_NN_UInt32Array) and all 75 functions, including every destructor
        // (OH_NNModel_Destroy, OH_NNTensorDesc_Destroy, and the verb-infix
        // OH_NNExecutor_DestroyInputMemory/DestroyOutputMemory).
        "OH_NN.*",
        // The opaque handles and callback typedefs that drop the OH_ prefix: NN_QuantParam,
        // NN_TensorDesc, NN_Tensor, NN_OnRunDone, NN_OnServiceDied.
        "NN_.*",
    ],
    // The type header only pulls stddef/stdint; the other two add
    // info/application_target_sdk_version.h, whose get_application_target_sdk_version and
    // OH_API_VERSION_* are already excluded by the prefixed white list and the generator's
    // built-in blocks.
    block_list: vec![],
    dynamic_library: vec!["neural_network_core", "neural_network_runtime"],
    extra: "",
});
