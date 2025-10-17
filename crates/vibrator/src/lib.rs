use ohos_vibrator_sys::{
    OH_Vibrator_Cancel, OH_Vibrator_PlayVibration, OH_Vibrator_PlayVibrationCustom,
    Vibrator_Attribute, Vibrator_FileDescription,
};

mod error;
mod usage;

pub use error::*;
pub use usage::*;

/// This function is used to cancel the vibration.
pub fn cancel() -> Result<(), VibratorError> {
    let ret = unsafe { OH_Vibrator_Cancel() };
    if ret != 0 {
        return Err(VibratorError::InternalError(ret));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Attribute {
    pub vibrator_id: i32,
    pub usage: VibratorUsage,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            vibrator_id: 0,
            usage: VibratorUsage::Unknown,
        }
    }
}

impl From<Attribute> for Vibrator_Attribute {
    fn from(attribute: Attribute) -> Self {
        Vibrator_Attribute {
            vibratorId: attribute.vibrator_id,
            usage: attribute.usage.into(),
        }
    }
}

/// Start the vibration.
/// ```rs
/// use ohos_vibrator_binding::{start, Attribute};
///
/// let attribute = Attribute {
///     usage: VibratorUsage::Unknown,
///     ..Default::default()
/// };
/// start(1000, attribute).unwrap();
/// ```
pub fn start(duration: i32, attribute: Attribute) -> Result<(), VibratorError> {
    let ret = unsafe { OH_Vibrator_PlayVibration(duration, attribute.into()) };
    if ret != 0 {
        return Err(VibratorError::InternalError(ret));
    }
    Ok(())
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileDescription {
    pub fd: i32,
    pub offset: u64,
    pub length: u64,
}

impl From<FileDescription> for Vibrator_FileDescription {
    fn from(file_description: FileDescription) -> Self {
        Vibrator_FileDescription {
            fd: file_description.fd,
            offset: file_description.offset as _,
            length: file_description.length as _,
        }
    }
}

/// Start the vibration with custom file description. Allow user to use custom vibration file.   
/// See details: https://developer.huawei.com/consumer/cn/doc/harmonyos-guides/vibrator-guidelines#%E6%8C%AF%E5%8A%A8%E6%95%88%E6%9E%9C%E8%AF%B4%E6%98%8E
///
/// ```rs
/// #[napi]
/// pub fn custom_start() -> Result<()> {
///     let file_info = fs::File::open("/data/test/vibrator/coin_drop.json")
///         .map_err(|e| Error::from_reason(e.to_string()))?;
///     let file_description = FileDescription {
///         fd: file_info.as_raw_fd(),
///         offset: 0,
///         length: file_info.metadata().map_err(|e| Error::from_reason(e.to_string()))?.len(),
///     };
///     let attribute = Attribute {
///         usage: VibratorUsage::Alarm,
///         ..Default::default()
///     };
///     ohos_vibrator_binding::custom_start(file_description, attribute).unwrap();
///     Ok(())
/// }
/// ```
pub fn custom_start(
    file_description: FileDescription,
    attribute: Attribute,
) -> Result<(), VibratorError> {
    let ret = unsafe { OH_Vibrator_PlayVibrationCustom(file_description.into(), attribute.into()) };
    if ret != 0 {
        return Err(VibratorError::InternalError(ret));
    }
    Ok(())
}
