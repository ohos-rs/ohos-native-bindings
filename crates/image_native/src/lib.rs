//! Safe wrappers for OpenHarmony image-native APIs.

mod common;
mod error;
mod image;
mod image_packer;
mod image_receiver;
mod image_source;
mod picture;
mod pixel_map;
pub mod types;

pub use ohos_image_native_sys as sys;

pub use common::{
    ImageString, NativeBufferHandle, NativeColorSpaceManagerHandle, PixelMapNativeHandle,
};
pub use error::{ImageNativeError, ImageNativeResult};
pub use image::{Image, NativeImage};
#[cfg(feature = "api-18")]
pub use image_packer::PackingOptionsForSequence;
pub use image_packer::{ImagePacker, PackingOptions};
pub use image_receiver::{ImageReceiver, ImageReceiverOptions};
#[cfg(feature = "api-13")]
pub use image_source::DecodingOptionsForPicture;
pub use image_source::{DecodingOptions, ImageSource, ImageSourceInfo};
#[cfg(feature = "api-23")]
pub use picture::ComposeOptions;
#[cfg(feature = "api-13")]
pub use picture::{AuxiliaryPicture, AuxiliaryPictureInfo, Picture, PictureMetadata};
pub use pixel_map::{PixelMap, PixelMapImageInfo, PixelMapInitializationOptions};
#[cfg(feature = "api-20")]
pub use types::AllocatorMode;
#[cfg(feature = "api-15")]
pub use types::AllocatorType;
#[cfg(feature = "api-13")]
pub use types::AuxiliaryPictureType;
#[cfg(feature = "api-13")]
pub use types::MetadataType;
pub use types::NativeBufferFormat;
pub use types::{HdrMetadataKey, HdrMetadataType, PixelFormat, PixelMapAntiAliasingLevel};
