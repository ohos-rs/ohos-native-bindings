use std::{
    collections::HashMap,
    ffi::CString,
    sync::{LazyLock, Mutex},
};

use bitflags::bitflags;
use ohos_web_sys::{
    ArkWeb_CustomSchemeOption, ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CODE_CACHE_ENABLED,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CORS_ENABLED,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CSP_BYPASSING,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_DISPLAY_ISOLATED,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_FETCH_ENABLED,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_LOCAL,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_SECURE,
    ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_STANDARD,
    ArkWeb_CustomSchemeOption_OH_ARKWEB_SCHEME_OPTION_NONE, OH_ArkWeb_RegisterCustomSchemes,
};

/// custom protocol list
static CUSTOM_PROTOCOL_LIST: LazyLock<Mutex<HashMap<String, CustomProtocolOption>>> =
    LazyLock::new(|| Mutex::new(HashMap::new()));

bitflags! {
    #[derive(PartialEq, Clone, Copy)]
    pub struct CustomProtocolOption: u32 {
        const None = 0;
        const Standard = 1;
        const Local = 2;
        const DisplayIsolated = 4;
        const Secure = 8;
        const CorsEnabled = 16;
        const CspBypassing = 32;
        const FetchEnabled = 64;
        const CodeCacheEnabled = 128;
    }
}

impl From<CustomProtocolOption> for ArkWeb_CustomSchemeOption {
    fn from(value: CustomProtocolOption) -> Self {
        let mut option = 0;
        if value.contains(CustomProtocolOption::Standard) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_STANDARD;
        }
        if value.contains(CustomProtocolOption::Local) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_LOCAL;
        }
        if value.contains(CustomProtocolOption::DisplayIsolated) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_DISPLAY_ISOLATED;
        }
        if value.contains(CustomProtocolOption::Secure) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_SECURE;
        }
        if value.contains(CustomProtocolOption::CorsEnabled) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CORS_ENABLED;
        }
        if value.contains(CustomProtocolOption::CspBypassing) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CSP_BYPASSING;
        }
        if value.contains(CustomProtocolOption::FetchEnabled) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_FETCH_ENABLED;
        }
        if value.contains(CustomProtocolOption::CodeCacheEnabled) {
            option |= ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CODE_CACHE_ENABLED;
        }
        option
    }
}

impl From<ArkWeb_CustomSchemeOption> for CustomProtocolOption {
    fn from(value: ArkWeb_CustomSchemeOption) -> Self {
        match value {
            #![allow(non_upper_case_globals)]
            ArkWeb_CustomSchemeOption_OH_ARKWEB_SCHEME_OPTION_NONE => CustomProtocolOption::None,
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_STANDARD => {
                CustomProtocolOption::Standard
            }
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_LOCAL => CustomProtocolOption::Local,
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_DISPLAY_ISOLATED => {
                CustomProtocolOption::DisplayIsolated
            }
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_SECURE => CustomProtocolOption::Secure,
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CORS_ENABLED => {
                CustomProtocolOption::CorsEnabled
            }
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CSP_BYPASSING => {
                CustomProtocolOption::CspBypassing
            }
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_FETCH_ENABLED => {
                CustomProtocolOption::FetchEnabled
            }
            ArkWeb_CustomSchemeOption_ARKWEB_SCHEME_OPTION_CODE_CACHE_ENABLED => {
                CustomProtocolOption::CodeCacheEnabled
            }
            _ => CustomProtocolOption::None,
        }
    }
}

/// Register custom protocol for ArkWeb
/// You can register multi custom protocol at once
/// You need to call `CustomProtocol::register` to register to web before web init.
/// ```ignore
/// CustomProtocol::register("custom", CustomProtocolOption::Standard);
/// CustomProtocol::register("custom2", CustomProtocolOption::Standard);
/// CustomProtocol::register_to_web();
/// ```
pub struct CustomProtocol;

impl CustomProtocol {
    pub fn add_protocol(protocol: &str, option: CustomProtocolOption) {
        let mut list = CUSTOM_PROTOCOL_LIST.lock().unwrap();
        list.insert(protocol.to_string(), option);
    }

    pub fn is_protocol_registered(protocol: &str) -> bool {
        let list = CUSTOM_PROTOCOL_LIST.lock().unwrap();
        list.contains_key(&protocol.to_string())
    }

    pub fn register() {
        let mut list = CUSTOM_PROTOCOL_LIST.lock().unwrap();
        list.iter_mut().for_each(|p| unsafe {
            let protocol_cstr = CString::new(p.0.clone()).unwrap();
            let option: ArkWeb_CustomSchemeOption = (*p.1).into();
            OH_ArkWeb_RegisterCustomSchemes(protocol_cstr.as_ptr(), option as i32);
        });
    }
}
