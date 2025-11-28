#![allow(clippy::all)]

use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, Result};

use ohos_hilog_binding::hilog_info;
use ohos_resource_manager_binding::ResourceManager;

#[napi]
pub fn open_raw_dir<'a>(
    env: Env,
    #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object<'a>,
    dir: String,
) -> Result<()> {
    let raw_manager = ResourceManager::new(env, resource_manager);
    let raw_dir = raw_manager
        .open_dir(dir, true)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let files = raw_dir.files.clone();

    hilog_info!("open_raw_dir: {:?}", files);

    Ok(())
}
