use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env};
use ohos_raw_binding::Raw;

#[napi]
pub fn raw_example(
    env: Env,
    #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object,
) -> i32 {
    let raw_manager = Raw::new(env, resource_manager);
    let raw_dir = raw_manager.open_dir("");
    let count = raw_dir.count();
    count
}
