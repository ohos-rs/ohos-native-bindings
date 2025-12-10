#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]
#![allow(clippy::module_inception)]

use crate::config::SysConfig;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::{BTreeSet, HashMap};
use std::env;
use std::fs;
use std::process::Command;

/// Baseline API version - versions <= this don't need feature gates
const BASELINE_API_VERSION: u32 = 12;

mod config;

static CONFIG: Lazy<Vec<Lazy<SysConfig>>> = Lazy::new(|| {
    vec![
        config::ARKUI,
        config::EVENT,
        config::XCOMPONENT,
        config::RESOURCE_MANAGER,
        config::ABILITY,
        config::ASSET,
        config::BUNDLE,
        config::HILOG,
        config::INIT,
        config::VSYNC,
        config::NATIVE_DISPLAY_SOLOIST,
        config::MULTI_MODAL_INPUT,
        config::INPUT_METHOD,
        config::DISPLAY,
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
    ]
});

/// Adds `#[cfg(feature = "api-XX")]` attributes based on `@since XX` annotations in doc comments.
/// Only adds feature gates for API versions > BASELINE_API_VERSION.
/// Also handles enum constants that reference types with higher API versions.
/// Returns the processed content and a set of API versions found (> baseline).
fn add_feature_gates(content: &str) -> (String, BTreeSet<u32>) {
    let since_re = Regex::new(r"@since\s+(\d+)").unwrap();
    let type_def_re = Regex::new(r"^pub type (\w+)\s*=").unwrap();
    // Match single-line const: pub const NAME: TYPE = or pub const NAME : TYPE =
    // Note: there can be space before and/or after the colon
    let const_single_line_re = Regex::new(r"^pub const \w+\s*:\s*(\w+)\s*=").unwrap();
    // Match start of multi-line const: pub const NAME: (with optional space before colon)
    let const_start_re = Regex::new(r"^pub const \w+\s*:\s*$").unwrap();
    // Match continuation line with type: TYPE =
    let const_type_line_re = Regex::new(r"^\s*(\w+)\s*=").unwrap();

    let lines: Vec<&str> = content.lines().collect();
    let mut api_versions = BTreeSet::new();

    // First pass: collect all type definitions and their API versions
    let mut type_versions: HashMap<String, u32> = HashMap::new();
    let mut pending_version: Option<u32> = None;

    for line in &lines {
        let trimmed = line.trim();

        if trimmed.starts_with("#[doc = ") {
            // Extract @since version from doc comment
            if let Some(captures) = since_re.captures(trimmed) {
                if let Ok(version) = captures[1].parse::<u32>() {
                    if version > BASELINE_API_VERSION {
                        pending_version = Some(version);
                    }
                }
            }
        } else if trimmed.starts_with("pub type ") {
            // Record type with its API version
            if let Some(captures) = type_def_re.captures(trimmed) {
                if let Some(version) = pending_version {
                    type_versions.insert(captures[1].to_string(), version);
                }
            }
            pending_version = None;
        } else if !trimmed.starts_with("#[") && !trimmed.is_empty() {
            // Reset pending version for non-attribute, non-empty lines
            pending_version = None;
        }
    }

    // Second pass: add cfg attributes
    let mut result = Vec::new();
    let mut cfg_already_added = false;
    // For multi-line const declarations
    let mut pending_const_line: Option<String> = None;
    let mut pending_const_indent: String = String::new();

    for line in lines.iter() {
        let trimmed = line.trim();
        let indent = line.len() - trimmed.len();
        let indent_str = &line[..indent];

        // Handle continuation of multi-line const
        if let Some(ref const_line) = pending_const_line.clone() {
            if let Some(captures) = const_type_line_re.captures(trimmed) {
                let type_name = &captures[1];
                if !cfg_already_added {
                    if let Some(&version) = type_versions.get(type_name) {
                        api_versions.insert(version);
                        result.push(format!(
                            "{}#[cfg(feature = \"api-{}\")]",
                            pending_const_indent, version
                        ));
                    }
                }
                result.push(const_line.clone());
                cfg_already_added = false;
            } else {
                // Not a type line, just push the pending const line
                result.push(const_line.clone());
            }
            pending_const_line = None;
            pending_const_indent.clear();
            result.push(line.to_string());
            continue;
        }

        if trimmed.starts_with("#[doc = ") {
            // Check for @since in doc comment
            if let Some(captures) = since_re.captures(trimmed) {
                if let Ok(version) = captures[1].parse::<u32>() {
                    if version > BASELINE_API_VERSION {
                        api_versions.insert(version);
                        // Add cfg before the doc comment
                        result.push(format!(
                            "{}#[cfg(feature = \"api-{}\")]",
                            indent_str, version
                        ));
                        cfg_already_added = true;
                    }
                }
            }
            result.push(line.to_string());
        } else if trimmed.starts_with("pub const ") {
            // Check if this is a single-line or multi-line const
            if const_start_re.is_match(trimmed) {
                // Multi-line const: buffer this line and wait for the type on next line
                pending_const_line = Some(line.to_string());
                pending_const_indent = indent_str.to_string();
            } else if let Some(captures) = const_single_line_re.captures(trimmed) {
                // Single-line const
                if !cfg_already_added {
                    let type_name = &captures[1];
                    if let Some(&version) = type_versions.get(type_name) {
                        api_versions.insert(version);
                        result.push(format!(
                            "{}#[cfg(feature = \"api-{}\")]",
                            indent_str, version
                        ));
                    }
                }
                cfg_already_added = false;
                result.push(line.to_string());
            } else {
                // Other const format, just push
                cfg_already_added = false;
                result.push(line.to_string());
            }
        } else if trimmed.starts_with("pub type ")
            || trimmed.starts_with("pub fn ")
            || trimmed.starts_with("pub struct ")
            || trimmed.starts_with("pub enum ")
            || trimmed.starts_with("pub static ")
        {
            // Reset state for other declarations
            cfg_already_added = false;
            result.push(line.to_string());
        } else if !trimmed.starts_with("#[")
            && !trimmed.is_empty()
            && !trimmed.starts_with("//")
            && !trimmed.starts_with("extern ")
            && !trimmed.starts_with('{')
            && !trimmed.starts_with('}')
        {
            // Reset state for other non-attribute lines
            cfg_already_added = false;
            result.push(line.to_string());
        } else {
            result.push(line.to_string());
        }
    }

    // Handle any remaining pending const line
    if let Some(const_line) = pending_const_line {
        result.push(const_line);
    }

    (result.join("\n"), api_versions)
}

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

    let dynamic_library_content = config
        .dynamic_library
        .iter()
        .map(|i| format!("#[link(name = \"{}\")]", i))
        .collect::<Vec<String>>()
        .join("\n");

    let mut bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", &header_content)
        .raw_line(format!(
            r#"#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
{}

{}
unsafe extern "C" {{}}"#,
            config.extra, dynamic_library_content
        ))
        .clang_arg("-x")
        .clang_arg("c")
        .generate_comments(true)
        .clang_arg("-fretain-comments-from-system-headers") // keep comments from system headers
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
    let bindings = bindings.blocklist_item(r".*@deprecated.*").generate()?;

    let out_path = basic_folder.join("src");
    let output_file = out_path.join("lib.rs");

    // Write to file first, then read and process to add feature gates
    bindings.write_to_file(&output_file)?;

    // Read the generated content
    let content = fs::read_to_string(&output_file)?;

    // Add feature gates based on @since annotations
    let (processed_content, _) = add_feature_gates(&content);

    // Write the processed content back
    fs::write(&output_file, processed_content)?;

    Ok(())
}

fn main() {
    CONFIG.iter().for_each(|i| {
        generate_code(i).unwrap();
    })
}
