use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_ashmem_binding::{Ashmem, Protection};

fn to_err(e: ohos_ashmem_binding::AshmemError) -> Error {
    Error::from_reason(e.to_string())
}

#[napi]
pub fn roundtrip(payload: String) -> Result<String> {
    let bytes = payload.as_bytes();
    let mut ashmem = Ashmem::create("ohos-rs-demo", 4096).map_err(to_err)?;
    let size = ashmem.size();
    ashmem.map_read_write().map_err(to_err)?;
    ashmem.write(0, bytes).map_err(to_err)?;
    let read_back = ashmem.read(0, bytes.len()).map_err(to_err)?;
    let cloned = ashmem.try_clone().map_err(to_err)?;
    let mapped = ashmem.is_mapped();
    ashmem.unmap().map_err(to_err)?;
    drop(cloned);
    ashmem.close().map_err(to_err)?;
    let text = String::from_utf8_lossy(&read_back).into_owned();
    Ok(format!(
        "size={size} mapped_before_unmap={mapped} read={text:?}"
    ))
}

#[napi]
pub fn restrict_then_read() -> Result<String> {
    let mut ashmem = Ashmem::create("ohos-rs-demo-ro", 1024).map_err(to_err)?;
    ashmem.map_read_write().map_err(to_err)?;
    ashmem.write(0, b"hello").map_err(to_err)?;
    ashmem.unmap().map_err(to_err)?;
    ashmem.set_protection(Protection::READ).map_err(to_err)?;
    let write_denied = ashmem.map_read_write().is_err();
    ashmem.map_read_only().map_err(to_err)?;
    let data = ashmem.read(0, 5).map_err(to_err)?;
    Ok(format!(
        "write_after_restrict_denied={write_denied} data={}",
        String::from_utf8_lossy(&data)
    ))
}

#[napi]
pub fn smoke() -> Result<String> {
    Ok(format!(
        "{}\n{}",
        roundtrip("ashmem-ok".to_string())?,
        restrict_then_read()?
    ))
}
