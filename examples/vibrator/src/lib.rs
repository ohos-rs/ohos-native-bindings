use std::{fs, os::fd::AsRawFd};

use napi_derive_ohos::napi;
use napi_ohos::{Error, Result};
use ohos_vibrator_binding::{Attribute, FileDescription, VibratorUsage};

#[napi]
pub fn cancel() {
    ohos_vibrator_binding::cancel().unwrap();
}

#[napi]
pub fn start() {
    let attribute = Attribute {
        usage: VibratorUsage::Alarm,
        ..Default::default()
    };
    ohos_vibrator_binding::start(0, attribute).unwrap();
}

#[napi]
pub fn custom_start() -> Result<()> {
    let file_info = fs::File::open("/data/test/vibrator/coin_drop.json")
        .map_err(|e| Error::from_reason(e.to_string()))?;
    let file_description = FileDescription {
        fd: file_info.as_raw_fd(),
        offset: 0,
        length: file_info.metadata().map_err(|e| Error::from_reason(e.to_string()))?.len(),
    };
    let attribute = Attribute {
        usage: VibratorUsage::Alarm,
        ..Default::default()
    };
    ohos_vibrator_binding::custom_start(file_description, attribute).unwrap();

    Ok(())
}
