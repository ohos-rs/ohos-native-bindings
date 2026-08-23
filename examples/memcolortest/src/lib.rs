use napi_derive_ohos::napi;
use ohos_native_color_space_manager_binding as color_space;
use ohos_purgeable_memory_binding as purgeable;

use color_space::{Chromaticity, ColorSpaceManager, ColorSpaceName, ColorSpacePrimaries};
use purgeable::{PurgeableMemory, PurgeableMemoryError};

const TAG: &str = "MEMCOLOR_TEST";

/// Size of the purgeable block used by the tests, in bytes.
const BLOCK: usize = 4096;
/// Byte the rebuild function fills the block with.
const REBUILD_FILL: u8 = 0xAB;
/// Byte written through `append_modify`, on top of the rebuild fill.
const MODIFY_BYTE: u8 = 0x5A;

/// The pattern written through the write guard: a repeating ramp, so a wrong
/// offset or a stale block is visible in the comparison.
fn expected_byte(index: usize) -> u8 {
    (index % 251) as u8
}

/// Create a block, write a known pattern through the write guard, then read it
/// back through the read guard and compare.
///
/// The system may reclaim the content once the write guard is dropped, so both
/// `ContentPurged` and a block holding the rebuild fill are reported as they
/// are instead of being treated as failures.
#[napi]
pub fn test_purgeable_write_read() -> String {
    let msg = match PurgeableMemory::new(BLOCK, |content: &mut [u8]| {
        content.fill(REBUILD_FILL);
        true
    }) {
        Err(e) => format!("purgeable create Err({e})"),
        Ok(mut mem) => {
            let len = mem.len();
            // The write guard borrows the object exclusively; it is scoped so
            // that the permit is released before the read below.
            let write_ok = {
                let write = mem.write();
                match write {
                    Err(e) => Err(format!("purgeable len={len} write Err({e})")),
                    Ok(mut guard) => {
                        let written = guard.len();
                        for (i, byte) in guard.iter_mut().enumerate() {
                            *byte = expected_byte(i);
                        }
                        Ok(written == len && len == BLOCK)
                    }
                }
            };
            match write_ok {
                Err(message) => message,
                Ok(write_ok) => match mem.read() {
                    Err(PurgeableMemoryError::ContentPurged) => format!(
                        "purgeable len={len} write_ok={write_ok} purged=true readback_match=false"
                    ),
                    Err(e) => format!("purgeable len={len} write_ok={write_ok} read Err({e})"),
                    Ok(content) => {
                        let readback_match = content.len() == len
                            && content
                                .iter()
                                .enumerate()
                                .all(|(i, b)| *b == expected_byte(i));
                        let rebuilt = content.iter().all(|b| *b == REBUILD_FILL);
                        format!(
                            "purgeable len={len} write_ok={write_ok} readback_match={readback_match} rebuilt={rebuilt} first={:#04x}",
                            content.first().copied().unwrap_or(0)
                        )
                    }
                },
            }
        }
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// Append a modification on top of the rebuild function and read the content
/// back, checking that the appended change is visible and that the object
/// survives the extra registered builder.
#[napi]
pub fn test_purgeable_append_modify() -> String {
    let msg = match PurgeableMemory::new(BLOCK, |content: &mut [u8]| {
        content.fill(REBUILD_FILL);
        true
    }) {
        Err(e) => format!("purgeable_append create Err({e})"),
        Ok(mut mem) => match mem.append_modify(|content: &mut [u8]| {
            if let Some(first) = content.first_mut() {
                *first = MODIFY_BYTE;
            }
            true
        }) {
            Err(e) => format!("purgeable_append append_modify Err({e})"),
            Ok(()) => match mem.read() {
                Err(PurgeableMemoryError::ContentPurged) => {
                    format!(
                        "purgeable_append len={} append_ok=true purged=true",
                        mem.len()
                    )
                }
                Err(e) => format!("purgeable_append append_ok=true read Err({e})"),
                Ok(content) => {
                    let first = content.first().copied().unwrap_or(0);
                    let tail_fill = content.iter().skip(1).all(|b| *b == REBUILD_FILL);
                    format!(
                        "purgeable_append len={} append_ok=true first={first:#04x} modify_match={} tail_fill={tail_fill}",
                        content.len(),
                        first == MODIFY_BYTE
                    )
                }
            },
        },
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// Create a standard sRGB color space instance, read its name back and query
/// its white point and gamma. The instance is released when it goes out of
/// scope, which exercises the RAII drop.
#[napi]
pub fn test_colorspace_from_name() -> String {
    let msg = match ColorSpaceManager::from_name(ColorSpaceName::Srgb) {
        Err(e) => format!("colorspace from_name Err({e})"),
        Ok(manager) => {
            let name = match manager.name() {
                Ok(name) => format!("{name:?}"),
                Err(e) => format!("Err({e})"),
            };
            let name_match = manager.name().map(|n| n == ColorSpaceName::Srgb) == Ok(true);
            let white = match manager.white_point() {
                Ok(point) => format!("({},{})", point.x, point.y),
                Err(e) => format!("Err({e})"),
            };
            let gamma = match manager.gamma() {
                Ok(gamma) => format!("{gamma}"),
                Err(e) => format!("Err({e})"),
            };
            format!("colorspace name={name} name_match={name_match} white={white} gamma={gamma}")
        }
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}

/// Create a custom color space from BT.709 primaries with a D65 white point and
/// a gamma of 2.2, then read the gamma back and compare it within a float
/// tolerance.
#[napi]
pub fn test_colorspace_custom() -> String {
    const GAMMA: f32 = 2.2;
    const TOLERANCE: f32 = 1e-3;

    let primaries = ColorSpacePrimaries {
        red: Chromaticity::new(0.64, 0.33),
        green: Chromaticity::new(0.30, 0.60),
        blue: Chromaticity::new(0.15, 0.06),
        white: Chromaticity::new(0.3127, 0.3290),
    };

    let msg = match ColorSpaceManager::from_primaries_and_gamma(primaries, GAMMA) {
        Err(e) => format!("colorspace_custom from_primaries_and_gamma Err({e})"),
        Ok(manager) => {
            let name = match manager.name() {
                Ok(name) => format!("{name:?}"),
                Err(e) => format!("Err({e})"),
            };
            let (gamma, gamma_match) = match manager.gamma() {
                Ok(gamma) => (format!("{gamma}"), (gamma - GAMMA).abs() < TOLERANCE),
                Err(e) => (format!("Err({e})"), false),
            };
            let white = match manager.white_point() {
                Ok(point) => format!("({},{})", point.x, point.y),
                Err(e) => format!("Err({e})"),
            };
            format!(
                "colorspace_custom name={name} gamma={gamma} gamma_match={gamma_match} white={white}"
            )
        }
    };
    ohos_hilog_binding::hilog_info!("{}: {}", TAG, msg);
    msg
}
