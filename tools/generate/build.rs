use anyhow::Error;
use once_cell::sync::Lazy;
use std::env;
use std::path::PathBuf;
use std::process::Command;

struct SysConfig<'a> {
    /// crate name
    pub name: &'a str,
    /// include headers
    pub headers: Vec<&'a str>,
}

static CONFIG: Lazy<Vec<SysConfig>> = Lazy::new(|| {
    vec![
        SysConfig {
            name: "ohos-bundle-sys",
            headers: vec!["bundle/native_interface_bundle.h"],
        },
        SysConfig {
            name: "ohos-init-sys",
            headers: vec!["syscap_ndk.h"],
        },
        // ohos-hilog-sys already exists
        SysConfig {
            name: "ohos-hilogs-sys",
            headers: vec!["hilog/log.h"],
        },
    ]
});

fn generate_code(config: &SysConfig) -> anyhow::Result<()> {
    let pwd = env::current_dir()?;
    let basic_folder = PathBuf::from(pwd)
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .join("sys")
        .join(&config.name);

    if !basic_folder.is_dir() {
        let _ = Command::new("cargo")
            .current_dir(
                &basic_folder
                    .parent()
                    .ok_or(Error::msg("Get parent path failed"))?,
            )
            .arg("new")
            .arg(&config.name)
            .arg("--lib")
            .status()?;
    }
    let header_content = config
        .headers
        .iter()
        .map(|i| format!("#include <{}>", i))
        .collect::<Vec<String>>()
        .join("\n");

    let bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", &header_content)
        .raw_line(
            "#![allow(non_snake_case)]\n#![allow(non_upper_case_globals)]\n#![allow(non_camel_case_types)]",
        )
        .layout_tests(false)
        .generate()?;

    let out_path = basic_folder.join("src");
    bindings.write_to_file(out_path.join("lib.rs"))?;
    Ok(())
}

fn main() {
    CONFIG.iter().for_each(|i| {
        generate_code(i).unwrap();
    })
}
