use std::{fs, os::fd::AsRawFd};

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_vibrator_binding::{Attribute, FileDescription, VibratorUsage};

#[napi]
pub fn cancel() -> Result<()> {
    ohos_vibrator_binding::cancel().map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
}

#[napi]
pub fn start() -> Result<()> {
    let attribute = Attribute {
        usage: VibratorUsage::Alarm,
        ..Default::default()
    };
    ohos_vibrator_binding::start(5000, attribute).map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
}

#[napi]
pub fn custom_start(file_path: String) -> Result<()> {
    // /data/storage/el2/base/haps/entry/files/coin_drop.json
    let file_info = fs::File::open(file_path).map_err(|e| Error::from_reason(e.to_string()))?;
    let file_description = FileDescription {
        fd: file_info.as_raw_fd(),
        offset: 0,
        length: file_info
            .metadata()
            .map_err(|e| Error::from_reason(e.to_string()))?
            .len(),
    };
    let attribute = Attribute {
        usage: VibratorUsage::Ring,
        ..Default::default()
    };
    ohos_vibrator_binding::custom_start(file_description, attribute)
        .map_err(|e| Error::from_reason(e.to_string()))?;
    Ok(())
}
