use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_fileuri_binding::{
    get_file_name, get_full_directory_uri, get_path_from_uri, get_uri_from_path, is_valid_uri,
};

fn to_err(e: ohos_fileuri_binding::FileUriError) -> Error {
    Error::from_reason(e.to_string())
}

#[napi]
pub fn uri_from_path(path: String) -> Result<String> {
    get_uri_from_path(&path).map_err(to_err)
}

#[napi]
pub fn path_from_uri(uri: String) -> Result<String> {
    get_path_from_uri(&uri).map_err(to_err)
}

#[napi]
pub fn directory_uri(uri: String) -> Result<String> {
    get_full_directory_uri(&uri).map_err(to_err)
}

#[napi]
pub fn file_name(uri: String) -> Result<String> {
    get_file_name(&uri).map_err(to_err)
}

#[napi]
pub fn valid_uri(uri: String) -> bool {
    is_valid_uri(&uri)
}

#[napi]
pub fn smoke(path: String) -> Result<String> {
    let uri = get_uri_from_path(&path).map_err(to_err)?;
    let back = get_path_from_uri(&uri).map_err(to_err)?;
    let dir = get_full_directory_uri(&uri).map_err(to_err)?;
    let name = get_file_name(&uri).map_err(to_err)?;
    Ok(format!(
        "path={path}\nuri={uri}\npath_from_uri={back}\ndir_uri={dir}\nfile_name={name}\nvalid_uri={}\nvalid_garbage={}",
        is_valid_uri(&uri),
        is_valid_uri("not a uri")
    ))
}
