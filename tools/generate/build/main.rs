#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]
#![allow(clippy::module_inception)]

use crate::config::SysConfig;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{BTreeMap, HashMap};
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

/// Baseline API version - versions <= this don't need feature gates
const BASELINE_API_VERSION: u32 = 12;

mod config;
mod feature_gates;
mod header_since;

use feature_gates::{add_feature_gates, merge_symbol_usage_min};
use header_since::{HeaderSinceInventory, IncludedHeaders};

static CONFIG: Lazy<Vec<Lazy<SysConfig>>> = Lazy::new(|| {
    vec![
        config::ARKUI,
        config::XCOMPONENT,
        config::RESOURCE_MANAGER,
        config::ABILITY,
        config::NATIVE_CHILD_PROCESS,
        config::ASSET,
        config::BUNDLE,
        config::HILOG,
        config::INIT,
        config::VSYNC,
        config::NATIVE_DISPLAY_SOLOIST,
        config::INPUT,
        config::INPUT_METHOD,
        config::DISPLAY,
        config::WINDOW_MANAGER,
        config::NATIVE_WINDOW,
        config::ACCESSIBILITY,
        config::NATIVE_BUFFER,
        config::PASTEBOARD,
        config::UDMF,
        config::IMAGE_NATIVE,
        config::IMAGE,
        config::ARK_WEB,
        config::SENSORS,
        config::VIBRATOR,
        config::QOS,
        config::NET_CONNECTION,
        config::NET_STACK,
        config::OHAUDIO_BASE,
        config::OHAUDIO,
        config::OHAUDIO_SUITE,
        config::FILEURI,
        config::FILESHARE,
        config::DRAWING,
        config::ARKUI_INPUT,
        config::JSVM,
        config::CAMERA,
        config::OPENGTX,
        config::HUKS,
        config::HICOLLIE,
    ]
});

fn sys_crate_manifest(name: &str) -> String {
    let description = name
        .trim_start_matches("ohos-")
        .trim_end_matches("-sys")
        .replace('-', " ");

    format!(
        r#"[package]
name        = "{name}"
version     = "0.1.0"
edition     = "2021"
license     = "MIT OR Apache-2.0"
description = "OpenHarmony's {description} sys binding for rust"

[dependencies]

[features]
default = []
api-13  = []
api-14  = ["api-13"]
api-15  = ["api-14"]
api-16  = ["api-15"]
api-17  = ["api-16"]
api-18  = ["api-17"]
api-19  = ["api-18"]
api-20  = ["api-19"]
api-21  = ["api-20"]
api-22  = ["api-21"]
api-23  = ["api-22"]
api-24  = ["api-23"]

[lints]
workspace = true
"#,
    )
}

fn max_manifest_api_feature(content: &str) -> Option<u32> {
    let api_feature_re = Regex::new(r"(?m)^api-(\d+)\s*=").unwrap();
    api_feature_re
        .captures_iter(content)
        .filter_map(|capture| capture[1].parse::<u32>().ok())
        .max()
}

fn manifest_feature_entry(content: &str, version: u32) -> Option<(usize, usize)> {
    let feature_re = Regex::new(&format!(r"(?m)^api-{version}\s*=")).unwrap();
    let start = feature_re.find(content)?.start();
    let mut end = start;
    let mut bracket_depth = 0_i32;
    let mut saw_bracket = false;

    for line in content[start..].split_inclusive('\n') {
        for ch in line.chars() {
            match ch {
                '[' => {
                    saw_bracket = true;
                    bracket_depth += 1;
                }
                ']' => bracket_depth -= 1,
                _ => {}
            }
        }
        end += line.len();
        if saw_bracket && bracket_depth == 0 {
            return Some((start, end));
        }
    }

    (saw_bracket && bracket_depth == 0).then_some((start, end))
}

fn sync_manifest_api_features(path: &Path, target_version: u32) -> anyhow::Result<()> {
    let mut content = fs::read_to_string(path)?;
    let original = content.clone();

    loop {
        let current_version = max_manifest_api_feature(&content).ok_or_else(|| {
            Error::msg(format!(
                "No API feature entries found in {}",
                path.display()
            ))
        })?;
        if current_version >= target_version {
            break;
        }

        let (start, end) = manifest_feature_entry(&content, current_version).ok_or_else(|| {
            Error::msg(format!(
                "Could not locate api-{current_version} feature entry in {}",
                path.display()
            ))
        })?;
        let next_version = current_version + 1;
        let mut next_entry = content[start..end].replace(
            &format!("api-{current_version}"),
            &format!("api-{next_version}"),
        );
        if current_version > BASELINE_API_VERSION + 1 {
            next_entry = next_entry.replace(
                &format!("\"api-{}\"", current_version - 1),
                &format!("\"api-{current_version}\""),
            );
        }
        content.insert_str(end, &next_entry);
    }

    if content != original {
        fs::write(path, content)?;
    }
    Ok(())
}

fn sys_crate_folder(config: &SysConfig) -> anyhow::Result<std::path::PathBuf> {
    let pwd = env::current_dir()?;
    Ok(pwd
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .parent()
        .ok_or(Error::msg("Get parent path failed"))?
        .join("sys")
        .join(config.name))
}

fn generate_code(
    config: &SysConfig,
    header_inventory: &mut HeaderSinceInventory,
) -> anyhow::Result<(std::path::PathBuf, BTreeMap<String, u32>)> {
    let basic_folder = sys_crate_folder(config)?;

    let mut created_new_crate = false;
    if !basic_folder.is_dir() {
        let status = Command::new("cargo")
            .current_dir(
                basic_folder
                    .parent()
                    .ok_or(Error::msg("Get parent path failed"))?,
            )
            .arg("new")
            .arg(config.name)
            .arg("--lib")
            .status()?;

        if !status.success() {
            return Err(Error::msg(format!(
                "cargo new failed for {} with status {}",
                config.name, status
            )));
        }

        created_new_crate = true;
    }

    if created_new_crate {
        fs::write(
            basic_folder.join("Cargo.toml"),
            sys_crate_manifest(config.name),
        )?;
    }

    let header_content = config
        .headers
        .iter()
        .map(|i| format!("#include <{}>", i))
        .collect::<Vec<String>>()
        .join("\n");

    let dynamic_library_content = config
        .dynamic_library
        .iter()
        .map(|i| format!("#[link(name = \"{}\")]", i))
        .collect::<Vec<String>>()
        .join("\n");

    let included_headers = IncludedHeaders::default();
    let mut bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", &header_content)
        .parse_callbacks(Box::new(included_headers.clone()))
        .raw_line(format!(
            r#"#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
// bindgen's bitfield accessors transmute/cast between identical types.
#![allow(clippy::useless_transmute)]
#![allow(clippy::unnecessary_cast)]
{}

{}
unsafe extern "C" {{}}"#,
            config.extra, dynamic_library_content
        ))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers") // keep comments from system headers
        .default_alias_style(bindgen::AliasVariation::TypeAlias)
        .translate_enum_integer_types(true)
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
    // Don't generate deprecated functions or types
    // Don't generate API version constants
    let bindings = bindings
        .blocklist_item(r".*@deprecated.*")
        .blocklist_item(r"OH_API_VERSION_.*")
        .blocklist_item(r"OH_CURRENT_API_VERSION")
        .generate()?;

    let out_path = basic_folder.join("src");
    let output_file = out_path.join("lib.rs");

    // Write to file first, then read and process to add feature gates
    bindings.write_to_file(&output_file)?;

    let header_since = header_inventory.for_headers(included_headers.paths())?;
    Ok((output_file, header_since))
}

fn format_rust_file(path: &Path) -> anyhow::Result<()> {
    let rustfmt = env::var_os("RUSTFMT").unwrap_or_else(|| "rustfmt".into());
    let status = Command::new(rustfmt)
        .arg("--edition")
        .arg("2021")
        .arg(path)
        .status()?;

    if !status.success() {
        return Err(Error::msg(format!(
            "rustfmt failed for {} with status {}",
            path.display(),
            status
        )));
    }

    Ok(())
}

fn main() {
    // Unit tests exercise generation with fixtures and must not rewrite the workspace.
    println!("cargo:rerun-if-env-changed=OHOS_BINDINGS_SKIP_GENERATION");
    println!("cargo:rerun-if-changed=build");
    if env::var_os("OHOS_BINDINGS_SKIP_GENERATION").is_some_and(|value| value == "1") {
        return;
    }
    let mut header_inventory = HeaderSinceInventory::default();
    let mut failed_configs = Vec::new();
    let mut generated_files = Vec::new();
    CONFIG
        .iter()
        .for_each(|i| match generate_code(i, &mut header_inventory) {
            Ok((output_file, header_since)) => {
                generated_files.push((i.name, output_file, header_since))
            }
            Err(e) => {
                eprintln!("Failed to generate code for {}: {}", i.name, e);
                failed_configs.push(i.name);
            }
        });

    let mut global_symbol_usage_min = HashMap::new();
    let mut raw_outputs = Vec::new();
    for (name, output_file, header_since) in generated_files {
        match fs::read_to_string(&output_file) {
            Ok(content) => {
                let (_, _, local_usage_min) = add_feature_gates(&content, &header_since, None);
                merge_symbol_usage_min(&mut global_symbol_usage_min, &local_usage_min);
                raw_outputs.push((name, output_file, content, header_since));
            }
            Err(e) => {
                eprintln!("Failed to read generated code for {}: {}", name, e);
                failed_configs.push(name);
            }
        }
    }

    let mut max_generated_api_version = BASELINE_API_VERSION;
    for (name, output_file, content, header_since) in raw_outputs {
        let (processed_content, api_versions, _) =
            add_feature_gates(&content, &header_since, Some(&global_symbol_usage_min));
        if let Some(version) = api_versions.last() {
            max_generated_api_version = max_generated_api_version.max(*version);
        }
        if let Err(e) = fs::write(&output_file, processed_content) {
            eprintln!("Failed to write generated code for {}: {}", name, e);
            failed_configs.push(name);
        } else if let Err(e) = format_rust_file(&output_file) {
            eprintln!("Failed to format generated code for {}: {}", name, e);
            failed_configs.push(name);
        }
    }

    for config in CONFIG.iter() {
        let manifest_path = match sys_crate_folder(config) {
            Ok(crate_dir) => crate_dir.join("Cargo.toml"),
            Err(e) => {
                eprintln!(
                    "Failed to locate manifest for {} while synchronizing API features: {}",
                    config.name, e
                );
                failed_configs.push(config.name);
                continue;
            }
        };
        if let Err(e) = sync_manifest_api_features(&manifest_path, max_generated_api_version) {
            eprintln!(
                "Failed to synchronize API features for {}: {}",
                manifest_path.display(),
                e
            );
            failed_configs.push(config.name);
        }
    }

    if !failed_configs.is_empty() {
        eprintln!("\nWarning: Failed to generate code for the following configs:");
        for name in &failed_configs {
            eprintln!("  - {}", name);
        }
        eprintln!(
            "\nNote: Some header files may have syntax errors that need to be fixed manually."
        );
        // Don't exit with error code to allow other configs to succeed
    }
}
