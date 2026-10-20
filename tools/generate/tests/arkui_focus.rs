#[allow(dead_code, unused_imports)]
#[path = "../build/config/mod.rs"]
mod config;
use config::SysConfig;
const BASELINE_API_VERSION: u32 = 12;
#[path = "../build/feature_gates.rs"]
#[allow(dead_code)]
mod feature_gates;

#[test]
fn focus_header_is_registered_and_requests_keep_the_api_15_signature() {
    assert!(config::ARKUI
        .headers
        .contains(&"arkui/native_interface_focus.h"));
    let mut builder = bindgen::Builder::default()
        .header_contents(
            "native_interface_focus.h",
            r#"
                typedef struct ArkUI_Node* ArkUI_NodeHandle;
                typedef enum { ARKUI_ERROR_CODE_NO_ERROR = 0 } ArkUI_ErrorCode;
                /** @since 15 */
                ArkUI_ErrorCode OH_ArkUI_FocusRequest(ArkUI_NodeHandle node);
                void Unrelated_Function(void);
            "#,
        )
        .generate_comments(true)
        .layout_tests(false);
    for pattern in &config::ARKUI.white_list {
        builder = builder
            .allowlist_function(pattern)
            .allowlist_type(pattern)
            .allowlist_var(pattern);
    }
    for pattern in &config::ARKUI.block_list {
        builder = builder.blocklist_item(pattern);
    }
    let raw = builder.generate().unwrap().to_string();
    let (gated, _, _) = feature_gates::add_feature_gates(&raw, &Default::default(), None);
    let file = syn::parse_file(&gated).unwrap();
    let functions: Vec<_> = file
        .items
        .iter()
        .filter_map(|item| match item {
            syn::Item::ForeignMod(block) => Some(&block.items),
            _ => None,
        })
        .flatten()
        .filter_map(|item| match item {
            syn::ForeignItem::Fn(function) => Some(function),
            _ => None,
        })
        .collect();
    assert_eq!(functions.len(), 1);
    let request = functions[0];
    assert_eq!(request.sig.ident, "OH_ArkUI_FocusRequest");
    assert_eq!(request.sig.inputs.len(), 1);
    let syn::FnArg::Typed(node) = &request.sig.inputs[0] else {
        panic!("expected a native node parameter");
    };
    let syn::Type::Path(node) = node.ty.as_ref() else {
        panic!("expected ArkUI_NodeHandle");
    };
    assert!(node.path.is_ident("ArkUI_NodeHandle"));
    let syn::ReturnType::Type(_, result) = &request.sig.output else {
        panic!("expected a native result code");
    };
    let syn::Type::Path(result) = result.as_ref() else {
        panic!("expected ArkUI_ErrorCode");
    };
    assert!(result.path.is_ident("ArkUI_ErrorCode"));
    let cfg = request
        .attrs
        .iter()
        .find(|attr| attr.path().is_ident("cfg"))
        .expect("focus requests need an API gate");
    cfg.parse_nested_meta(|meta| {
        assert!(meta.path.is_ident("feature"));
        let value: syn::LitStr = meta.value()?.parse()?;
        assert_eq!(value.value(), "api-15");
        Ok(())
    })
    .unwrap();
}
