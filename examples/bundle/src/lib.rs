use napi_derive_ohos::napi;
use ohos_bundle_binding::{
    get_app_identifier, get_appid, get_bundle_info, get_compatible_device_type,
    get_main_element_name,
};

#[napi]
pub fn current_bundle_info() -> String {
    let info = get_bundle_info();
    format!(
        "bundle_name={}\nfingerprint={}",
        info.bundle_name, info.fingerprint
    )
}

#[napi]
pub fn app_id() -> String {
    get_appid().to_string()
}

#[napi]
pub fn app_identifier() -> String {
    get_app_identifier().to_string()
}

#[napi]
pub fn main_element_name() -> String {
    let name = get_main_element_name();
    format!(
        "bundle={}\nmodule={}\nability={}",
        name.bundle_name, name.module_name, name.ability_name
    )
}

#[napi]
pub fn compatible_device_type() -> String {
    get_compatible_device_type().to_string()
}

#[napi]
pub fn smoke() -> String {
    format!(
        "{}\nappid={}\nidentifier={}\n{}\ndevice_type={}",
        current_bundle_info(),
        app_id(),
        app_identifier(),
        main_element_name(),
        compatible_device_type()
    )
}
