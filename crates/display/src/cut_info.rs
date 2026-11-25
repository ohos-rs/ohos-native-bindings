use ohos_display_sys::{NativeDisplayManager_CutoutInfo, NativeDisplayManager_Rect};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

impl From<NativeDisplayManager_Rect> for Rect {
    fn from(value: NativeDisplayManager_Rect) -> Self {
        Self {
            left: value.left,
            top: value.top,
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct WaterfallDisplayAreaRect {
    pub left: Rect,
    pub top: Rect,
    pub right: Rect,
    pub bottom: Rect,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct DisplayCutInfo {
    pub bounding_rect_info: i32,
    pub bounding_rect: Rect,
    pub waterfall_display_area_rect: WaterfallDisplayAreaRect,
}

impl From<NativeDisplayManager_CutoutInfo> for DisplayCutInfo {
    fn from(value: NativeDisplayManager_CutoutInfo) -> Self {
        let bounding_rect = if value.boundingRectsLength > 0 && !value.boundingRects.is_null() {
            // SAFETY: `boundingRectsLength > 0` and non-null guarantees at least one rect entry.
            Rect::from(unsafe { *value.boundingRects })
        } else {
            Rect::default()
        };

        DisplayCutInfo {
            bounding_rect_info: value.boundingRectsLength,
            bounding_rect,
            waterfall_display_area_rect: WaterfallDisplayAreaRect {
                left: Rect::from(value.waterfallDisplayAreaRects.left),
                top: Rect::from(value.waterfallDisplayAreaRects.top),
                right: Rect::from(value.waterfallDisplayAreaRects.right),
                bottom: Rect::from(value.waterfallDisplayAreaRects.bottom),
            },
        }
    }
}

#[cfg(test)]
mod tests {
    use std::ptr;

    use ohos_display_sys::NativeDisplayManager_WaterfallDisplayAreaRects;

    use super::*;

    fn raw_rect(left: i32, top: i32, width: u32, height: u32) -> NativeDisplayManager_Rect {
        NativeDisplayManager_Rect {
            left,
            top,
            width,
            height,
        }
    }

    #[test]
    fn handles_null_or_empty_bounding_rects() {
        let raw = NativeDisplayManager_CutoutInfo {
            boundingRectsLength: 0,
            boundingRects: ptr::null_mut(),
            waterfallDisplayAreaRects: NativeDisplayManager_WaterfallDisplayAreaRects {
                left: raw_rect(0, 0, 10, 100),
                top: raw_rect(0, 0, 0, 0),
                right: raw_rect(0, 0, 0, 0),
                bottom: raw_rect(0, 0, 0, 0),
            },
        };

        let info = DisplayCutInfo::from(raw);
        assert_eq!(info.bounding_rect_info, 0);
        assert_eq!(info.bounding_rect, Rect::default());
        assert_eq!(
            info.waterfall_display_area_rect.left,
            Rect {
                left: 0,
                top: 0,
                width: 10,
                height: 100,
            }
        );

        let mut dummy = raw_rect(12, 24, 30, 40);
        let non_null_empty = NativeDisplayManager_CutoutInfo {
            boundingRectsLength: 0,
            boundingRects: &raw mut dummy,
            waterfallDisplayAreaRects: NativeDisplayManager_WaterfallDisplayAreaRects {
                left: raw_rect(0, 0, 0, 0),
                top: raw_rect(0, 0, 0, 0),
                right: raw_rect(0, 0, 0, 0),
                bottom: raw_rect(0, 0, 0, 0),
            },
        };
        assert_eq!(
            DisplayCutInfo::from(non_null_empty).bounding_rect,
            Rect::default()
        );
    }

    #[test]
    fn preserves_bounding_rect_coordinates_without_swapping() {
        let mut rect = raw_rect(120, 16, 48, 32);
        let raw = NativeDisplayManager_CutoutInfo {
            boundingRectsLength: 1,
            boundingRects: &raw mut rect,
            waterfallDisplayAreaRects: NativeDisplayManager_WaterfallDisplayAreaRects {
                left: raw_rect(1, 2, 3, 4),
                top: raw_rect(5, 6, 7, 8),
                right: raw_rect(9, 10, 11, 12),
                bottom: raw_rect(13, 14, 15, 16),
            },
        };

        let info = DisplayCutInfo::from(raw);
        assert_eq!(info.bounding_rect_info, 1);
        assert_eq!(
            info.bounding_rect,
            Rect {
                left: 120,
                top: 16,
                width: 48,
                height: 32,
            }
        );
        assert_eq!(
            info.waterfall_display_area_rect.bottom,
            Rect {
                left: 13,
                top: 14,
                width: 15,
                height: 16,
            }
        );
    }
}
