use std::collections::{BTreeMap, HashMap};

#[path = "../build/feature_gates.rs"]
mod feature_gates;
#[path = "../build/header_since.rs"]
mod header_since;

const BASELINE_API_VERSION: u32 = 12;

fn generate_fixture() -> (String, BTreeMap<String, u32>) {
    let headers = header_since::IncludedHeaders::default();
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/opaque_api.h");
    let raw = bindgen::Builder::default()
        .header_contents("wrapper.h", "#include <opaque_api.h>")
        .clang_arg(format!("-I{}", path.parent().unwrap().display()))
        .clang_arg("-x")
        .clang_arg("c++")
        .clang_arg("-std=c++17")
        .parse_callbacks(Box::new(headers.clone()))
        .allowlist_type("ArkUI_.*")
        .allowlist_type("OH_Camera.*")
        .allowlist_function("OH_Camera.*")
        .generate_comments(true)
        .layout_tests(false)
        .generate()
        .unwrap()
        .to_string();
    let since = header_since::HeaderSinceInventory::default()
        .for_headers(headers.paths())
        .unwrap();
    (raw, since)
}

fn gated_structs(source: &str) -> BTreeMap<String, u32> {
    let file = syn::parse_file(source).unwrap();
    let mut result = BTreeMap::new();
    for item in file.items {
        if let syn::Item::Struct(item) = item {
            for attr in item.attrs.iter().filter(|attr| attr.path().is_ident("cfg")) {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("feature") {
                        let value: syn::LitStr = meta.value()?.parse()?;
                        if let Some(api) = value.value().strip_prefix("api-") {
                            result.insert(item.ident.to_string(), api.parse().unwrap());
                        }
                    }
                    Ok(())
                })
                .unwrap();
            }
        }
    }
    result
}

#[test]
fn nested_callback_infers_opaque_since_without_header_metadata() {
    let (raw, _) = generate_fixture();
    let (output, _, usage) = feature_gates::add_feature_gates(&raw, &BTreeMap::new(), None);
    assert_eq!(usage.get("ArkUI_ParallelGestureEvent"), Some(&26));
    assert_eq!(gated_structs(&output)["ArkUI_ParallelGestureEvent"], 26);

    // Changing only the callback's layout must not change its dependencies.
    let start = raw.find("    pub setGestureParallelTo:").unwrap();
    let end = start + raw[start..].find("\n}").unwrap();
    let compact = format!(
        "{}{}{}",
        &raw[..start],
        raw[start..end]
            .lines()
            .map(str::trim)
            .collect::<Vec<_>>()
            .join(" "),
        &raw[end..],
    );
    let (compact_output, _, _) = feature_gates::add_feature_gates(&compact, &BTreeMap::new(), None);
    assert_eq!(gated_structs(&output), gated_structs(&compact_output));
}

#[test]
fn filtered_functions_do_not_erase_opaque_header_since() {
    let (raw, since) = generate_fixture();
    assert!(!raw.contains("OH_MetadataObjectExt_GetType"));
    assert_eq!(since["OH_Camera_MetadataObjectExt"], 26);

    // Exercise both local and cross-crate passes, as the real generator does.
    let (_, _, usage) = feature_gates::add_feature_gates(&raw, &since, None);
    let mut global_usage = HashMap::new();
    feature_gates::merge_symbol_usage_min(&mut global_usage, &usage);
    let (output, _, _) = feature_gates::add_feature_gates(&raw, &since, Some(&global_usage));
    let gates = gated_structs(&output);
    assert_eq!(gates["ArkUI_ParallelGestureEvent"], 26);
    assert_eq!(gates["OH_Camera_MetadataObjectExt"], 26);
    assert!(!output.contains("OH_MetadataObjectExt_GetType"));
}
