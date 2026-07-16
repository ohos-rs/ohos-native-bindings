use ohos_camera_sys as sys;

use crate::native::OutputCapability;
use crate::{CameraCapabilities, CameraError, CameraPosition, CameraResult, CameraSize};

const MAX_PREVIEW_AREA: u64 = 1920 * 1080;
const MAX_PHOTO_AREA: u64 = 12_000_000;

/// Owned snapshot of profiles discovered for one camera device.
pub(crate) struct CameraProfileCatalog {
    preview: Vec<sys::Camera_Profile>,
    photo: Vec<sys::Camera_Profile>,
}

impl CameraProfileCatalog {
    pub(crate) fn from_capability(capability: &OutputCapability<'_>) -> Self {
        Self {
            preview: capability.preview_profiles(),
            photo: capability.photo_profiles(),
        }
    }

    pub(crate) fn select_preview(
        &self,
        surface_size: CameraSize,
        requested_size: Option<CameraSize>,
    ) -> CameraResult<sys::Camera_Profile> {
        if let Some(requested_size) = requested_size {
            return self
                .preview
                .iter()
                .copied()
                .find(|profile| {
                    Self::is_preview_format(profile.format)
                        && profile.size.width == requested_size.width
                        && profile.size.height == requested_size.height
                })
                .ok_or_else(|| {
                    CameraError::unsupported(
                        "select preview profile",
                        format!(
                            "preview resolution {}x{} is not supported",
                            requested_size.width, requested_size.height
                        ),
                    )
                });
        }
        let target_area = surface_size.area().clamp(1, MAX_PREVIEW_AREA);
        let target_aspect = Self::normalized_aspect(surface_size.width, surface_size.height);
        self.preview
            .iter()
            .copied()
            .filter(|profile| Self::is_preview_format(profile.format))
            .min_by_key(|profile| {
                let aspect = Self::normalized_aspect(profile.size.width, profile.size.height);
                let aspect_penalty = ((aspect - target_aspect).abs() * 1_000_000.0) as u64;
                let area = u64::from(profile.size.width) * u64::from(profile.size.height);
                (
                    area > MAX_PREVIEW_AREA,
                    aspect_penalty,
                    area.abs_diff(target_area),
                )
            })
            .ok_or_else(|| {
                CameraError::unsupported(
                    "select preview profile",
                    "camera exposes no supported preview profile",
                )
            })
    }

    pub(crate) fn select_photo(
        &self,
        requested_size: Option<CameraSize>,
    ) -> CameraResult<Option<sys::Camera_Profile>> {
        if let Some(requested_size) = requested_size {
            return self
                .photo
                .iter()
                .copied()
                .find(|profile| {
                    profile.format == sys::Camera_Format_CAMERA_FORMAT_JPEG
                        && profile.size.width == requested_size.width
                        && profile.size.height == requested_size.height
                })
                .map(Some)
                .ok_or_else(|| {
                    CameraError::unsupported(
                        "select photo profile",
                        format!(
                            "photo resolution {}x{} is not supported",
                            requested_size.width, requested_size.height
                        ),
                    )
                });
        }
        Ok(self
            .photo
            .iter()
            .copied()
            .filter(|profile| profile.format == sys::Camera_Format_CAMERA_FORMAT_JPEG)
            .min_by_key(|profile| {
                let area = u64::from(profile.size.width) * u64::from(profile.size.height);
                (area > MAX_PHOTO_AREA, area.abs_diff(MAX_PHOTO_AREA))
            }))
    }

    #[cfg(feature = "api-20")]
    pub(crate) fn select_frame(
        &self,
        requested_size: Option<CameraSize>,
    ) -> CameraResult<sys::Camera_Profile> {
        if let Some(requested_size) = requested_size {
            return self
                .preview
                .iter()
                .copied()
                .find(|profile| {
                    profile.format == sys::Camera_Format_CAMERA_FORMAT_YUV_420_SP
                        && profile.size.width == requested_size.width
                        && profile.size.height == requested_size.height
                })
                .ok_or_else(|| {
                    CameraError::unsupported(
                        "select analysis frame profile",
                        format!(
                            "analysis resolution {}x{} is not supported",
                            requested_size.width, requested_size.height
                        ),
                    )
                });
        }
        const TARGET_AREA: u64 = 1280 * 720;
        self.preview
            .iter()
            .copied()
            .filter(|profile| profile.format == sys::Camera_Format_CAMERA_FORMAT_YUV_420_SP)
            .min_by_key(|profile| {
                let area = u64::from(profile.size.width) * u64::from(profile.size.height);
                (area > TARGET_AREA, area.abs_diff(TARGET_AREA))
            })
            .ok_or_else(|| {
                CameraError::unsupported(
                    "select analysis frame profile",
                    "camera exposes no YUV analysis profile",
                )
            })
    }

    pub(crate) fn capabilities(&self, position: CameraPosition) -> CameraCapabilities {
        let mut preview_sizes = self
            .preview
            .iter()
            .filter(|profile| Self::is_preview_format(profile.format))
            .map(|profile| CameraSize::new(profile.size.width, profile.size.height))
            .collect::<Vec<_>>();
        let mut photo_sizes = self
            .photo
            .iter()
            .filter(|profile| profile.format == sys::Camera_Format_CAMERA_FORMAT_JPEG)
            .map(|profile| CameraSize::new(profile.size.width, profile.size.height))
            .collect::<Vec<_>>();
        Self::sort_sizes(&mut preview_sizes);
        Self::sort_sizes(&mut photo_sizes);
        CameraCapabilities {
            position,
            preview_sizes: preview_sizes.into(),
            photo_sizes: photo_sizes.into(),
        }
    }

    fn is_preview_format(format: sys::Camera_Format) -> bool {
        format == sys::Camera_Format_CAMERA_FORMAT_YUV_420_SP
            || format == sys::Camera_Format_CAMERA_FORMAT_RGBA_8888
    }

    fn sort_sizes(sizes: &mut Vec<CameraSize>) {
        sizes.sort_unstable_by_key(|size| (size.area(), size.width, size.height));
        sizes.dedup();
    }

    fn normalized_aspect(width: u32, height: u32) -> f64 {
        let short = width.min(height).max(1);
        let long = width.max(height).max(1);
        f64::from(long) / f64::from(short)
    }
}
