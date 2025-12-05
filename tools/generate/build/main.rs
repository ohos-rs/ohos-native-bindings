#![allow(clippy::declare_interior_mutable_const)]
#![allow(clippy::borrow_interior_mutable_const)]
#![allow(clippy::module_inception)]

use crate::config::SysConfig;
use anyhow::Error;
use once_cell::sync::Lazy;
use regex::Regex;
use std::collections::BTreeSet;
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
/// Returns the processed content and a set of API versions found (> baseline).
fn add_feature_gates(content: &str) -> (String, BTreeSet<u32>) {
    let since_re = Regex::new(r"@since\s+(\d+)").unwrap();
    let lines: Vec<&str> = content.lines().collect();
    let mut result = Vec::new();
    let mut api_versions = BTreeSet::new();

    for line in lines.iter() {
        // Check if this line is a doc comment with @since
        if line.contains("#[doc = ") {
            if let Some(captures) = since_re.captures(line) {
                if let Ok(version) = captures[1].parse::<u32>() {
                    // Only add feature gate for versions above baseline
                    if version > BASELINE_API_VERSION {
                        api_versions.insert(version);
                        // Get the indentation from the current line
                        let indent = line.len() - line.trim_start().len();
                        let indent_str = &line[..indent];
                        result.push(format!(
                            "{}#[cfg(feature = \"api-{}\")]",
                            indent_str, version
                        ));
                    }
                }
            }
        }
        result.push(line.to_string());
    }

    (result.join("\n"), api_versions)
}

/// Updates the Cargo.toml file to add feature definitions for the found API versions.
fn update_cargo_toml_features(
    cargo_toml_path: &std::path::Path,
    api_versions: &BTreeSet<u32>,
) -> anyhow::Result<()> {
    if api_versions.is_empty() {
        return Ok(());
    }

    let content = fs::read_to_string(cargo_toml_path)?;

    // Remove existing [features] section if present
    let content_without_features = remove_features_section(&content);

    // Build the new features section
    let mut features_section = String::from("\n[features]\n");

    // Add default feature (empty by default)
    features_section.push_str("default = []\n");

    // Add each API version as a feature, with higher versions depending on lower ones
    let versions: Vec<u32> = api_versions.iter().copied().collect();
    for (i, version) in versions.iter().enumerate() {
        if i == 0 {
            // First (lowest) version has no dependencies
            features_section.push_str(&format!("api-{} = []\n", version));
        } else {
            // Higher versions depend on the previous version
            let prev_version = versions[i - 1];
            features_section.push_str(&format!("api-{} = [\"api-{}\"]\n", version, prev_version));
        }
    }

    // Append features section to the content
    let new_content = format!(
        "{}{}",
        content_without_features.trim_end(),
        features_section
    );

    fs::write(cargo_toml_path, new_content)?;

    Ok(())
}

/// Removes the [features] section from Cargo.toml content.
fn remove_features_section(content: &str) -> String {
    let mut result = String::new();
    let mut in_features_section = false;

    for line in content.lines() {
        let trimmed = line.trim();

        // Check if we're entering the features section
        if trimmed == "[features]" {
            in_features_section = true;
            continue;
        }

        // Check if we're entering a new section (leaving features)
        if in_features_section && trimmed.starts_with('[') && trimmed.ends_with(']') {
            in_features_section = false;
        }

        // Only include lines that are not in the features section
        if !in_features_section {
            result.push_str(line);
            result.push('\n');
        }
    }

    result
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

    let mut bindings = bindgen::Builder::default()
        .header_contents("wrapper.h", &header_content)
        .raw_line(format!(
            r"#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(clippy::missing_safety_doc)]
{}",
            config.extra
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
    let (processed_content, api_versions) = add_feature_gates(&content);

    // Write the processed content back
    fs::write(&output_file, processed_content)?;

    // Update Cargo.toml with feature definitions
    let cargo_toml_path = basic_folder.join("Cargo.toml");
    update_cargo_toml_features(&cargo_toml_path, &api_versions)?;

    Ok(())
}

fn main() {
    CONFIG.iter().for_each(|i| {
        generate_code(i).unwrap();
    })
}
