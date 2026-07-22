use once_cell::sync::Lazy;

use crate::SysConfig;

pub const MINDSPORE: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-mindspore-sys",
    // All seven headers share one @library and one type universe (OH_AI_Status,
    // OH_AI_TensorHandle, OH_AI_ContextHandle cross model/tensor/context), so they
    // must live in a single sys crate.
    headers: vec![
        "mindspore/types.h",
        "mindspore/status.h",
        "mindspore/data_type.h",
        "mindspore/format.h",
        "mindspore/context.h",
        "mindspore/tensor.h",
        "mindspore/model.h",
    ],
    white_list: vec![
        // Everything the kit exports is OH_AI_-prefixed: handles (OH_AI_ModelHandle,
        // OH_AI_TensorHandle, OH_AI_AllocatorHandle), enums (OH_AI_Status, OH_AI_DataType,
        // OH_AI_Format, OH_AI_Priority, ...), the OH_AI_MAX_SHAPE_NUM constant, the
        // OH_AI_KernelCallBack typedef, and all 87 functions. Verb-first destructor
        // OH_AI_DestroyAllNNRTDeviceDescs is covered too.
        "OH_AI_.*",
        // Opaque NNRT device description, the one type without the OH_AI_ prefix.
        "NNRTDeviceDesc",
    ],
    // context.h/model.h/tensor.h pull in info/application_target_sdk_version.h, which
    // declares get_application_target_sdk_version. The prefixed white list already
    // excludes it, so no block list is needed.
    block_list: vec![],
    dynamic_library: vec!["mindspore_lite_ndk"],
    extra: "",
});
