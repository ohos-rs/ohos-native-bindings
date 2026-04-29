//! Shared image value types.

use ohos_enum_macro::EnumFrom;
use ohos_image_sys::*;

pub use ohos_image_sys::{
    ImagePacker_Opts, OhosImageComponent, OhosImageDecodingOps, OhosImageReceiverInfo,
    OhosImageRect, OhosImageSize, OhosImageSource, OhosImageSourceDelayTimeList,
    OhosImageSourceInfo, OhosImageSourceOps, OhosImageSourceProperty,
    OhosImageSourceSupportedFormat, OhosImageSourceSupportedFormatList, OhosImageSourceUpdateData,
    OhosPixelMapCreateOps, OhosPixelMapInfos,
};
pub use ohos_resource_manager_sys::RawFileDescriptor;

/// Anti-aliasing level for pixel-map scaling.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_PixelMap_AntiAliasingLevel,
    "OH_PixelMap_AntiAliasingLevel_OH_PixelMap_AntiAliasing_"
)]
pub enum PixelMapAntiAliasingLevel {
    None,
    Low,
    Medium,
    High,
}
