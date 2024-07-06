use anyhow::Error;
use once_cell::sync::Lazy;
use std::env;
use std::process::Command;

struct SysConfig<'a> {
    /// crate name
    pub name: &'a str,
    /// include headers
    pub headers: Vec<&'a str>,
    pub white_list: Vec<&'a str>,
    pub block_list: Vec<&'a str>,

    pub extra: &'a str,
}

static CONFIG: Lazy<Vec<SysConfig>> = Lazy::new(|| {
    vec![
        SysConfig {
            name: "ohos-bundle-sys",
            headers: vec!["bundle/native_interface_bundle.h"],
            white_list: vec![],
            block_list: vec![],
            extra: "",
        },
        SysConfig {
            name: "ohos-init-sys",
            headers: vec!["syscap_ndk.h"],
            white_list: vec![],
            block_list: vec![],
            extra: "",
        },
        // ohos-hilog-sys already exists
        SysConfig {
            name: "ohos-hilogs-sys",
            headers: vec!["hilog/log.h"],
            white_list: vec![],
            block_list: vec![],
            extra: "",
        },
        SysConfig {
            name: "ohos-asset-sys",
            headers: vec!["asset/asset_api.h", "asset/asset_type.h"],
            white_list: vec![],
            block_list: vec![],
            extra: "",
        },
        // raw and resource file manager
        // raw_file deps on string, so we changed the raw_file content with c library
        // #include <stdint.h>
        // #include <stddef.h>
        // #include <stdbool.h>
        SysConfig {
            name: "ohos-raw-sys",
            headers: vec![
                "rawfile/raw_dir.h",
                "rawfile/raw_file.h",
                "rawfile/raw_file_manager.h",
            ],
            white_list: vec!["OH_ResourceManager_.*"],
            block_list: vec!["napi_.*"],
            extra: "\n\nuse napi_sys_ohos::*;\n",
        },
        SysConfig {
            name: "ohos-resource-manager-sys",
            headers: vec![
                "resourcemanager/ohresmgr.h",
                "resourcemanager/resmgr_common.h",
            ],
            white_list: vec![
                "OH_ResourceManager_GetMediaBase64",
                "OH_ResourceManager_GetMediaBase64ByName",
                "OH_ResourceManager_GetMedia",
                "OH_ResourceManager_GetMediaByName",
                "OH_ResourceManager_GetDrawableDescriptor",
                "OH_ResourceManager_GetDrawableDescriptorByName",
                "ScreenDensity",
            ],
            block_list: vec!["napi_.*", "NativeResourceManager"],
            extra: "\n\nuse ohos_raw_sys::*;\n",
        },
    ]
});

fn generate_code(config: &SysConfig) -> anyhow::Result<()> {
    let pwd = env::current_dir()?;
    let basic_folder = pwd
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .join("sys")
        .join(config.name);

    if !basic_folder.is_dir() {
        let _ = Command::new("cargo")
            .current_dir(
                basic_folder
                    .parent()
                    .ok_or(Error::msg("Get parent path failed"))?,
            )
            .arg("new")
            .arg(config.name)
            .arg("--lib")
            .status()?;
    }
    let header_content = config
        .headers
        .iter()
        .map(|i| format!("#include <{}>", i))
        .collect::<Vec<String>>()
        .join("\n");

    let mut bindings = bindgen::Builder::default()
        .header_contents("wrapper.hpp", &header_content)
        .raw_line(
            format!("#![allow(non_snake_case)]\n#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]{}", config.extra),
        )
        .layout_tests(false);

    if !config.white_list.is_empty() {
        for i in &config.white_list {
            bindings = bindings.allowlist_function(i);
            bindings = bindings.allowlist_var(i);
            bindings = bindings.allowlist_type(i);
        }
    }

    if !config.block_list.is_empty() {
        for i in &config.block_list {
            bindings = bindings.blocklist_item(i);
        }
    }

    let bindings = bindings.generate()?;

    let out_path = basic_folder.join("src");
    bindings.write_to_file(out_path.join("lib.rs"))?;
    Ok(())
}

fn main() {
    CONFIG.iter().for_each(|i| {
        generate_code(i).unwrap();
    })
}
