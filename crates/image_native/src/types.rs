//! Shared image-native value types and enums.

use crate::sys::*;
use ohos_enum_macro::EnumFrom;

pub type ImageSize = crate::sys::Image_Size;
pub type ImageRegion = crate::sys::Image_Region;
#[cfg(feature = "api-22")]
pub type ImagePositionArea = crate::sys::Image_PositionArea;
#[cfg(feature = "api-22")]
pub type ImageScale = crate::sys::Image_Scale;
#[cfg(feature = "api-23")]
pub type ImageBufferData = crate::sys::OH_ImageBufferData;
pub type ImageMimeType = crate::sys::Image_MimeType;
pub type ImageStringRaw = crate::sys::Image_String;
#[cfg(feature = "api-13")]
pub type PictureMetadataValue = crate::sys::OH_PictureMetadata;
pub type PixelMapHdrStaticMetadata = crate::sys::OH_Pixelmap_HdrStaticMetadata;
pub type PixelMapHdrDynamicMetadata = crate::sys::OH_Pixelmap_HdrDynamicMetadata;
pub type PixelMapHdrGainmapMetadata = crate::sys::OH_Pixelmap_HdrGainmapMetadata;
pub type PixelMapHdrMetadataValue = crate::sys::OH_Pixelmap_HdrMetadataValue;
pub type HdrMetadata = crate::sys::OHHDRMetaData;
pub use ohos_native_buffer_binding::NativeBufferFormat;
pub use ohos_native_buffer_sys::OH_NativeBuffer;
pub use ohos_resource_manager_sys::RawFileDescriptor;

/// Pixel format.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(PIXEL_FORMAT, "PIXEL_FORMAT_PIXEL_FORMAT_")]
pub enum PixelFormat {
    Unknown,
    Rgb565,
    Rgba8888,
    Bgra8888,
    Rgb888,
    Alpha8,
    RgbaF16,
    Nv21,
    Nv12,
    Rgba1010102,
    YCbCrP010,
    YCrCbP010,
}

impl From<PixelFormat> for i32 {
    fn from(value: PixelFormat) -> Self {
        let raw: PIXEL_FORMAT = value.into();
        raw as i32
    }
}

impl PixelFormat {
    pub(crate) fn from_raw(value: PIXEL_FORMAT) -> Option<Self> {
        Self::try_from_raw(value)
    }

    pub(crate) fn from_i32(value: i32) -> Option<Self> {
        u32::try_from(value).ok().and_then(Self::from_raw)
    }

    pub(crate) fn from_i32_or_unknown(value: i32) -> Self {
        Self::from_i32(value).unwrap_or(Self::Unknown)
    }
}

/// Pixel-map anti-aliasing level.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_PixelmapNative_AntiAliasingLevel,
    "OH_PixelmapNative_AntiAliasingLevel_OH_PixelmapNative_AntiAliasing_"
)]
pub enum PixelMapAntiAliasingLevel {
    None,
    Low,
    Medium,
    High,
}

/// HDR metadata key.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(OH_Pixelmap_HdrMetadataKey, "OH_Pixelmap_HdrMetadataKey_HDR_")]
pub enum HdrMetadataKey {
    MetadataType,
    StaticMetadata,
    DynamicMetadata,
    GainmapMetadata,
}

/// HDR metadata type.
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    OH_Pixelmap_HdrMetadataType,
    "OH_Pixelmap_HdrMetadataType_HDR_METADATA_TYPE_"
)]
pub enum HdrMetadataType {
    None,
    Base,
    Gainmap,
    Alternate,
}

/// Image metadata type.
#[cfg(feature = "api-13")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(Image_MetadataType, "Image_MetadataType_")]
pub enum MetadataType {
    ExifMetadata,
    FragmentMetadata,
    #[cfg(feature = "api-20")]
    GifMetadata,
}

#[cfg(feature = "api-13")]
#[allow(non_upper_case_globals)]
impl MetadataType {
    pub const Exif: Self = Self::ExifMetadata;
    pub const Fragment: Self = Self::FragmentMetadata;
    #[cfg(feature = "api-20")]
    pub const Gif: Self = Self::GifMetadata;
}

/// Pixel-map allocator mode.
#[cfg(feature = "api-20")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(IMAGE_ALLOCATOR_MODE, "IMAGE_ALLOCATOR_MODE_IMAGE_ALLOCATOR_MODE_")]
pub enum AllocatorMode {
    Auto,
    Dma,
    SharedMemory,
}

/// Auxiliary picture type.
#[cfg(feature = "api-13")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    Image_AuxiliaryPictureType,
    "Image_AuxiliaryPictureType_AUXILIARY_PICTURE_TYPE_"
)]
pub enum AuxiliaryPictureType {
    Gainmap,
    DepthMap,
    UnrefocusMap,
    LinearMap,
    FragmentMap,
}

#[cfg(feature = "api-13")]
impl AuxiliaryPictureType {
    pub(crate) fn from_raw(value: Image_AuxiliaryPictureType) -> Option<Self> {
        Self::try_from_raw(value)
    }
}

/// Decoder allocator type.
#[cfg(feature = "api-15")]
#[derive(Clone, Copy, Debug, PartialEq, Eq, EnumFrom)]
#[enum_from_config(IMAGE_ALLOCATOR_TYPE, "IMAGE_ALLOCATOR_TYPE_IMAGE_ALLOCATOR_TYPE_")]
pub enum AllocatorType {
    Auto,
    Dma,
    ShareMemory,
}
