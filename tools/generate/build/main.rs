use crate::config::SysConfig;
use anyhow::Error;
use once_cell::sync::Lazy;
use std::env;
use std::process::Command;

mod config;

static CONFIG: Lazy<Vec<Lazy<SysConfig>>> = Lazy::new(|| {
    vec![
        config::ARKUI,
        config::EVENT,
        config::XCOMPONENT,
        config::RAW,
        config::RESOURCE_MANAGER,
        config::ABILITY,
        config::ASSET,
        config::BUNDLE,
        config::HILOG,
        config::INIT,
        config::VSYNC,
        config::NATIVE_DISPLAY_SOLOIST,
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
        .header_contents("wrapper.h", &header_content)
        .raw_line(
            format!("#![allow(non_snake_case)]\n#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]{}", config.extra),
        )
        .clang_arg("-x")
        .clang_arg("c")
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
