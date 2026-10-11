#[allow(dead_code, unused_imports)]
#[path = "../build/config/mod.rs"]
mod config;
use config::SysConfig;
const BASELINE_API_VERSION: u32 = 12;
#[path = "../build/feature_gates.rs"]
#[allow(dead_code)]
mod feature_gates;

#[test]
fn cloned_event_setters_survive_the_real_input_allowlist_and_remain_api_24() {
    let mut builder = bindgen::Builder::default()
        .header_contents("input.h", r#"
            typedef struct ArkUI_UIInputEvent ArkUI_UIInputEvent;
            typedef enum { ARKUI_NATIVE_NODE = 0 } ArkUI_NativeAPIVariantKind;
            /** @since 12 */
            void* OH_ArkUI_QueryModuleInterfaceByName(ArkUI_NativeAPIVariantKind type, const char* name);
            /** @since 12 */
            int OH_ArkUI_UIInputEvent_GetType(const ArkUI_UIInputEvent* event);
            /** @since 24 */
            int OH_ArkUI_ClonedEvent_SetTiltAngle(const ArkUI_UIInputEvent* event, float x, float y);
            /** @since 24 */
            int OH_ArkUI_ClonedEvent_SetPressedKeys(const ArkUI_UIInputEvent* event, int* keys, int length);
            void Unrelated_Function(void);
        "#)
        .generate_comments(true)
        .layout_tests(false);
    for pattern in &config::ARKUI_INPUT.white_list {
        builder = builder
            .allowlist_function(pattern)
            .allowlist_type(pattern)
            .allowlist_var(pattern);
    }
    let raw = builder.generate().unwrap().to_string();
    assert!(raw.contains("pub fn OH_ArkUI_QueryModuleInterfaceByName"));
    assert!(raw.contains("ArkUI_NativeAPIVariantKind_ARKUI_NATIVE_NODE"));
    let (gated, _, _) = feature_gates::add_feature_gates(&raw, &Default::default(), None);
    let file = syn::parse_file(&gated).unwrap();
    let mut setters = 0;
    for item in file.items {
        if let syn::Item::ForeignMod(block) = item {
            for item in block.items {
                if let syn::ForeignItem::Fn(function) = item {
                    let name = function.sig.ident.to_string();
                    assert_ne!(name, "Unrelated_Function");
                    if name.starts_with("OH_ArkUI_ClonedEvent_") {
                        setters += 1;
                        let cfg = function
                            .attrs
                            .iter()
                            .find(|attr| attr.path().is_ident("cfg"))
                            .unwrap();
                        cfg.parse_nested_meta(|meta| {
                            assert!(meta.path.is_ident("feature"));
                            let value: syn::LitStr = meta.value()?.parse()?;
                            assert_eq!(value.value(), "api-24");
                            Ok(())
                        })
                        .unwrap();
                    }
                }
            }
        }
    }
    assert_eq!(setters, 2);
}
