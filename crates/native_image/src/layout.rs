//! Checked linear RGBA layouts and native row copying.

use crate::Error;
use ohos_native_buffer_binding::NativeBufferFormat as Format;

#[derive(Clone, Copy)]
pub(crate) struct Layout {
    pub(crate) height: usize,
    pub(crate) row_bytes: usize,
    pub(crate) byte_len: usize,
}

fn invalid(operation: &'static str) -> Error {
    Error {
        operation,
        code: -1,
    }
}

impl Layout {
    pub(crate) fn new(width: u32, height: u32) -> Result<Self, Error> {
        if width == 0 || height == 0 || width > i32::MAX as u32 || height > i32::MAX as u32 {
            return Err(invalid("native dimensions"));
        }
        let width = usize::try_from(width).map_err(|_| invalid("width"))?;
        let height = usize::try_from(height).map_err(|_| invalid("height"))?;
        let row_bytes = width
            .checked_mul(4)
            .ok_or_else(|| invalid("RGBA row length"))?;
        let byte_len = row_bytes
            .checked_mul(height)
            .filter(|length| *length <= isize::MAX as usize)
            .ok_or_else(|| invalid("RGBA byte length"))?;
        Ok(Self {
            height,
            row_bytes,
            byte_len,
        })
    }

    pub(crate) fn allocate(self) -> Result<Vec<u8>, Error> {
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(self.byte_len)
            .map_err(|_| invalid("RGBA allocation"))?;
        bytes.resize(self.byte_len, 0);
        Ok(bytes)
    }

    pub(crate) fn mapped_len(self, stride: i32) -> Result<usize, Error> {
        let stride = usize::try_from(stride)
            .ok()
            .filter(|stride| *stride >= self.row_bytes)
            .ok_or_else(|| invalid("RGBA stride"))?;
        stride
            .checked_mul(self.height)
            .filter(|length| *length <= isize::MAX as usize)
            .ok_or_else(|| invalid("RGBA mapped length"))
    }
}

pub(crate) fn validate_format(format: i32) -> Result<(bool, bool), Error> {
    let bgra = format == Format::BGRA_8888 as i32 || format == Format::BGRX_8888 as i32;
    let opaque = format == Format::RGBX_8888 as i32 || format == Format::BGRX_8888 as i32;
    if bgra || opaque || format == Format::RGBA_8888 as i32 {
        Ok((bgra, opaque))
    } else {
        Err(invalid("RGBA format"))
    }
}

pub(crate) fn copy(
    bytes: &[u8],
    stride: i32,
    format: i32,
    layout: Layout,
) -> Result<Vec<u8>, Error> {
    let (bgra, opaque) = validate_format(format)?;
    let mapped_len = layout.mapped_len(stride)?;
    if bytes.len() < mapped_len {
        return Err(invalid("short RGBA mapping"));
    }
    let mut pixels = layout.allocate()?;
    for (source, output) in bytes[..mapped_len]
        .chunks_exact(stride as usize)
        .zip(pixels.chunks_exact_mut(layout.row_bytes))
    {
        for (source, output) in source[..layout.row_bytes]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(output.as_chunks_mut::<4>().0.iter_mut())
        {
            output.copy_from_slice(source);
            if bgra {
                output.swap(0, 2);
            }
            if opaque {
                output[3] = 255;
            }
        }
    }
    Ok(pixels)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generic_dimensions_have_no_application_pixel_budget() {
        assert_eq!(Layout::new(4096, 2048).unwrap().byte_len, 33_554_432);
        for dimensions in [
            (0, 1),
            (1, 0),
            (u32::MAX, 1),
            (i32::MAX as u32, i32::MAX as u32),
        ] {
            assert!(Layout::new(dimensions.0, dimensions.1).is_err());
        }
    }

    #[test]
    fn rows_preserve_alpha_and_strip_padding_with_bgra_conversion() {
        let layout = Layout::new(1, 2).unwrap();
        let pixels = copy(
            &[3, 2, 1, 4, 99, 99, 99, 99, 7, 6, 5, 8, 99, 99, 99, 99],
            8,
            Format::BGRA_8888 as i32,
            layout,
        )
        .unwrap();
        assert_eq!(pixels, [1, 2, 3, 4, 5, 6, 7, 8]);
    }

    #[test]
    fn incomplete_mappings_and_strides_never_return_short_frames() {
        let layout = Layout::new(2, 2).unwrap();
        assert!(copy(&[0; 15], 8, Format::RGBA_8888 as i32, layout).is_err());
        assert!(copy(&[0; 16], 7, Format::RGBA_8888 as i32, layout).is_err());
        assert!(copy(&[0; 16], -1, Format::RGBA_8888 as i32, layout).is_err());
        assert!(copy(&[0; 16], 8, Format::YUV_422_I as i32, layout).is_err());
        assert_eq!(
            copy(&[0; 16], 8, Format::RGBX_8888 as i32, layout).unwrap()[3],
            255
        );
    }
}
