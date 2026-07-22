use once_cell::sync::Lazy;

use crate::SysConfig;

pub const CONTENT_EMBED: Lazy<SysConfig> = Lazy::new(|| {
    SysConfig {
    name: "ohos-content-embed-sys",
    headers: vec![
        "ContentEmbedKit/content_embed/content_embed_common.h",
        "ContentEmbedKit/content_embed/content_embed_document.h",
        "ContentEmbedKit/content_embed/content_embed_extension.h",
        "ContentEmbedKit/content_embed/content_embed_proxy.h",
    ],
    white_list: vec![
        "OH_ContentEmbed_.*",
        "ContentEmbed_.*",
        "CE_.*",
        "MAX_OEID_LENGTH",
        "MAX_PATH_LENGTH",
        "MAX_NAME_LENGTH",
        "MAX_DESCRIPTION_LENGTH",
    ],
    block_list: vec![
        "OH_Pixelmap.*",
        // content_embed_extension.h includes the AbilityKit headers; those types are owned
        // by ohos-ability-runtime-sys / ohos-ability-base-sys. Regenerating them here would
        // produce incompatible duplicates no caller could bridge.
        "AbilityRuntime_.*",
        "OH_AbilityRuntime_.*",
        "ABILITY_RUNTIME_.*",
        "AbilityBase_.*",
        "OH_AbilityBase_.*",
        "ABILITY_BASE_.*",
    ],
    dynamic_library: vec!["content_embed_ndk"],
    extra: "\n\n#[allow(unused_imports)]\nuse ohos_ability_base_sys::*;\n#[allow(unused_imports)]\nuse ohos_ability_runtime_sys::*;\n#[allow(unused_imports)]\nuse ohos_image_native_sys::*;",
}
});
