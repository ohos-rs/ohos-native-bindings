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

/// Whether the given rawfile path is a directory.
#[napi]
pub fn is_raw_dir<'a>(
    env: Env,
    #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object<'a>,
    path: String,
) -> Result<bool> {
    let raw_manager = ResourceManager::new(env, resource_manager);
    Ok(raw_manager.is_raw_dir(path))
}

/// Open a rawfile, read it fully and return its byte length plus a short
/// text preview.
#[napi]
pub fn read_raw_file<'a>(
    env: Env,
    #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object<'a>,
    file_name: String,
) -> Result<String> {
    let raw_manager = ResourceManager::new(env, resource_manager);
    let dir = raw_manager
        .open_dir("", false)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let file = dir.open_file(&file_name);
    let size = file.file_size();
    let contents = file.read(size as usize);
    let preview: String = String::from_utf8_lossy(&contents[..contents.len().min(64)]).into_owned();
    Ok(format!("size: {size} bytes, preview: {preview:?}"))
}

/// Seek around in a rawfile to demonstrate offset/remain APIs.
#[napi]
pub fn seek_raw_file<'a>(
    env: Env,
    #[napi(ts_arg_type = "resourceManager.ResourceManager")] resource_manager: Object<'a>,
    file_name: String,
) -> Result<String> {
    let raw_manager = ResourceManager::new(env, resource_manager);
    let dir = raw_manager
        .open_dir("", false)
        .map_err(|e| Error::from_reason(e.to_string()))?;

    let file = dir.open_file(&file_name);
    let size = file.file_size();
    let offset_after_seek = file.seek(size / 2, 0);
    let remain = file.remain();
    let tail = file.read(remain as usize);
    let preview: String = String::from_utf8_lossy(&tail[..tail.len().min(32)]).into_owned();
    Ok(format!(
        "size: {size}, seek(to {}) -> offset {}, remain {}, tail preview: {preview:?}",
        size / 2,
        offset_after_seek,
        remain
    ))
}
