//! Device probes for the native fence and native effect bindings.
//!
//! Neither binding can be exercised end to end from a plain NAPI module:
//!
//! * The fence kit has no creation entry point at all — a fence file
//!   descriptor only ever comes out of another kit (a native window acquire or
//!   release fence, for instance), and none of those can be reached without a
//!   surface. The probes below therefore cover the parts that do not need a
//!   real fence: the `-1` sentinel rejection, the ownership round trip and the
//!   timeout guards, plus `is_valid` on a descriptor that is not a fence, which
//!   the native API accepts because it only rejects negative descriptors.
//!   Waiting on a descriptor that is not a fence is left alone on purpose.
//! * The effect kit needs a pixel map, which the effect crate itself cannot
//!   produce. This example depends on the image-native binding instead and
//!   hands its pixel map pointer over through `PixelMapHandle::from_raw`, so
//!   the filters below run on real pixels.

use std::fs::File;
use std::os::fd::{AsFd, AsRawFd, OwnedFd};
use std::time::Duration;

use napi_derive_ohos::napi;

use ohos_image_native_binding::{
    ImageNativeError, PixelFormat, PixelMap, PixelMapAlphaType, PixelMapInitializationOptions,
};
use ohos_native_effect_binding::{ColorMatrix, Filter, PixelMapHandle, TileMode, COLOR_MATRIX_LEN};
use ohos_native_fence_binding::{Fence, FenceError, FenceRef};

const TAG: &str = "GFX_TEST";

const SOURCE_WIDTH: u32 = 8;
const SOURCE_HEIGHT: u32 = 8;

fn log(msg: &str) {
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
}

// --------------------------------------------------------------------------
// native fence
// --------------------------------------------------------------------------

/// `Fence::from_raw_fd` must refuse the `-1` sentinel the graphics stack uses
/// for "already signalled, no fence".
#[napi]
pub fn test_fence_rejects_sentinel() -> String {
    let msg = match unsafe { Fence::from_raw_fd(-1) } {
        Err(FenceError::InvalidFd) => "from_raw_fd(-1) Err(InvalidFd) ok=true".to_string(),
        Err(e) => format!("from_raw_fd(-1) Err({e}) ok=false"),
        Ok(fence) => {
            // Never close a descriptor that should not have been accepted.
            let fd = fence.into_raw_fd();
            format!("from_raw_fd(-1) unexpectedly Ok(fd={fd}) ok=false")
        }
    };
    log(&msg);
    msg
}

/// `OH_NativeFence_IsValid` only rejects negative descriptors, so a plain open
/// descriptor that is not a fence is reported as valid. No fence descriptor is
/// reachable from this process, so this is the only real call the check can be
/// given here.
#[napi]
pub fn test_fence_is_valid_on_open_fd() -> String {
    let msg = match File::open("/dev/null") {
        Ok(file) => {
            let fd = file.as_raw_fd();
            let valid = FenceRef::new(file.as_fd()).is_valid();
            format!("IsValid(open fd={fd}, not a fence)={valid} ok={valid}")
        }
        Err(e) => format!("open /dev/null failed: {e} ok=false"),
    };
    log(&msg);
    msg
}

/// `OwnedFd` -> `Fence` -> raw -> `Fence` keeps the same descriptor and closes
/// it exactly once, through `OH_NativeFence_Close`.
#[napi]
pub fn test_fence_ownership_round_trip() -> String {
    let msg = match File::open("/dev/null") {
        Ok(file) => {
            let owned = OwnedFd::from(file);
            let original = owned.as_raw_fd();
            let fence = Fence::from(owned);
            let kept = fence.as_raw_fd() == original;
            let released = fence.into_raw_fd();
            match unsafe { Fence::from_raw_fd(released) } {
                Ok(fence) => {
                    let valid = fence.is_valid();
                    let readopted = fence.as_raw_fd() == original;
                    // Closes the descriptor through OH_NativeFence_Close.
                    fence.close();
                    let ok = kept && readopted && valid;
                    format!(
                        "fd={original} kept_by_From={kept} kept_by_into_raw_fd={readopted} \
                         is_valid={valid} closed=1 ok={ok}"
                    )
                }
                Err(e) => format!("re-adopting fd={released} failed: {e} ok=false"),
            }
        }
        Err(e) => format!("open /dev/null failed: {e} ok=false"),
    };
    log(&msg);
    msg
}

/// The millisecond timeout is validated in Rust before the native wait is
/// reached: a sub-millisecond timeout and one that overflows the 32-bit
/// millisecond field are both rejected without any native call.
#[napi]
pub fn test_fence_timeout_guards() -> String {
    let msg = match File::open("/dev/null") {
        Ok(file) => {
            let fence = FenceRef::new(file.as_fd());
            let zero = fence.wait(Duration::ZERO);
            let too_long = fence.wait(Duration::from_millis(u64::from(u32::MAX) + 1));
            let ok =
                zero == Err(FenceError::ZeroTimeout) && too_long == Err(FenceError::TimeoutTooLong);
            format!("wait(0)={zero:?} wait(u32::MAX+1 ms)={too_long:?} ok={ok}")
        }
        Err(e) => format!("open /dev/null failed: {e} ok=false"),
    };
    log(&msg);
    msg
}

// --------------------------------------------------------------------------
// native effect
// --------------------------------------------------------------------------

/// Build a BGRA source pixel map with a horizontal gradient.
fn source_pixel_map() -> Result<(PixelMap, Vec<u8>), ImageNativeError> {
    let mut options = PixelMapInitializationOptions::new()?;
    options.set_width(SOURCE_WIDTH)?;
    options.set_height(SOURCE_HEIGHT)?;
    options.set_pixel_format(PixelFormat::Bgra8888)?;
    options.set_alpha_type(PixelMapAlphaType::Opaque)?;

    let mut data = Vec::with_capacity((SOURCE_WIDTH * SOURCE_HEIGHT * 4) as usize);
    for y in 0..SOURCE_HEIGHT {
        for x in 0..SOURCE_WIDTH {
            let b = (x * 32) as u8;
            let g = (y * 32) as u8;
            data.extend_from_slice(&[b, g, 0xC0, 0xFF]);
        }
    }
    let original = data.clone();
    let pixel_map = PixelMap::create(&mut data, &mut options)?;
    Ok((pixel_map, original))
}

/// Run a filter over a freshly built source pixel map and describe the result.
///
/// The rendered pixel map is adopted back into an owned [`PixelMap`] so that it
/// is released instead of leaked.
fn run_filter<F>(label: &str, apply: F) -> String
where
    F: FnOnce(&mut Filter) -> ohos_native_effect_binding::Result<()>,
{
    let (source, _) = match source_pixel_map() {
        Ok(pair) => pair,
        Err(e) => return format!("{label}: source pixel map failed: {e} ok=false"),
    };

    let handle = match unsafe { PixelMapHandle::from_raw(source.as_raw().cast()) } {
        Some(handle) => handle,
        None => return format!("{label}: source pixel map pointer is null ok=false"),
    };

    let mut filter = match Filter::create(handle) {
        Ok(filter) => filter,
        Err(e) => return format!("{label}: Filter::create failed: {e} ok=false"),
    };

    if let Err(e) = apply(&mut filter) {
        return format!("{label}: effect call failed: {e} ok=false");
    }

    let rendered = match filter.effect_pixel_map() {
        Ok(rendered) => rendered,
        Err(e) => return format!("{label}: effect_pixel_map failed: {e} ok=false"),
    };

    let raw = rendered.as_raw();
    let Some(result) = PixelMap::from_raw(raw.cast()) else {
        return format!("{label}: rendered pixel map pointer is null ok=false");
    };

    match result.image_info() {
        Ok(info) => {
            let width = info.width().unwrap_or(0);
            let height = info.height().unwrap_or(0);
            let ok = width == SOURCE_WIDTH && height == SOURCE_HEIGHT;
            format!("{label}: result={raw:p} {width}x{height} ok={ok}")
        }
        Err(e) => format!("{label}: result={raw:p} image_info failed: {e} ok=false"),
    }
}

/// Blur an 8x8 pixel map and render the result.
#[napi]
pub fn test_effect_blur() -> String {
    let msg = run_filter("blur(4.0)", |filter| filter.blur(4.0));
    log(&msg);
    msg
}

/// Blur with an explicit tile mode (API 14 entry point).
#[napi]
pub fn test_effect_blur_with_tile_mode() -> String {
    let msg = run_filter("blur_with_tile_mode(4.0, Clamp)", |filter| {
        filter.blur_with_tile_mode(4.0, TileMode::Clamp)
    });
    log(&msg);
    msg
}

/// Gray scale, the simplest effect with no parameters.
#[napi]
pub fn test_effect_gray_scale() -> String {
    let msg = run_filter("gray_scale", Filter::gray_scale);
    log(&msg);
    msg
}

/// Brighten followed by invert, to check that effects stack in order.
#[napi]
pub fn test_effect_brighten_then_invert() -> String {
    let msg = run_filter("brighten(0.5)+invert", |filter| {
        filter.brighten(0.5)?;
        filter.invert()
    });
    log(&msg);
    msg
}

/// Apply the identity color matrix and compare the rendered pixels with the
/// source bytes that were handed to the pixel map.
#[napi]
pub fn test_effect_identity_color_matrix() -> String {
    let msg = identity_color_matrix();
    log(&msg);
    msg
}

fn identity_color_matrix() -> String {
    let (source, original) = match source_pixel_map() {
        Ok(pair) => pair,
        Err(e) => return format!("identity matrix: source pixel map failed: {e} ok=false"),
    };

    let Some(handle) = (unsafe { PixelMapHandle::from_raw(source.as_raw().cast()) }) else {
        return "identity matrix: source pixel map pointer is null ok=false".to_string();
    };

    let mut filter = match Filter::create(handle) {
        Ok(filter) => filter,
        Err(e) => return format!("identity matrix: Filter::create failed: {e} ok=false"),
    };

    if let Err(e) = filter.set_color_matrix(&ColorMatrix::IDENTITY) {
        return format!("identity matrix: set_color_matrix failed: {e} ok=false");
    }

    let rendered = match filter.effect_pixel_map() {
        Ok(rendered) => rendered,
        Err(e) => return format!("identity matrix: effect_pixel_map failed: {e} ok=false"),
    };

    let Some(result) = PixelMap::from_raw(rendered.as_raw().cast()) else {
        return "identity matrix: rendered pixel map pointer is null ok=false".to_string();
    };

    let mut read_back = vec![0u8; original.len()];
    match result.read_pixels(&mut read_back) {
        Ok(size) => {
            let full = size == original.len();
            let identical = full && read_back == original;
            let differing = read_back
                .iter()
                .zip(original.iter())
                .filter(|(a, b)| a != b)
                .count();
            format!(
                "identity matrix: read {size}/{} bytes, differing={differing} identical={identical} ok={full}",
                original.len()
            )
        }
        Err(e) => format!("identity matrix: read_pixels failed: {e} ok=false"),
    }
}

/// `ColorMatrix` is plain Rust data: the identity matrix and the conversions to
/// and from a raw array need no device call.
#[napi]
pub fn test_effect_color_matrix_values() -> String {
    let identity = ColorMatrix::IDENTITY;
    let diagonal_ones = (0..4).all(|row| identity.values()[row * 5 + row] == 1.0);
    let others_zero = (0..COLOR_MATRIX_LEN)
        .filter(|index| index % 5 != index / 5)
        .all(|index| identity.values()[index] == 0.0);
    let default_is_identity = ColorMatrix::default() == identity;

    let mut raw = [0.0f32; COLOR_MATRIX_LEN];
    for (index, value) in raw.iter_mut().enumerate() {
        *value = index as f32;
    }
    let round_trip: [f32; COLOR_MATRIX_LEN] = ColorMatrix::from(raw).into();
    let round_trips = round_trip == raw;

    let mut edited = ColorMatrix::IDENTITY;
    edited.values_mut()[0] = 2.0;
    let edits = edited.values()[0] == 2.0 && edited != identity;

    let ok = diagonal_ones && others_zero && default_is_identity && round_trips && edits;
    let msg = format!(
        "ColorMatrix len={COLOR_MATRIX_LEN} identity_diagonal={diagonal_ones} \
         identity_rest_zero={others_zero} default_is_identity={default_is_identity} \
         array_round_trip={round_trips} values_mut={edits} ok={ok}"
    );
    log(&msg);
    msg
}
