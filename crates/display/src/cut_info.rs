use ohos_display_sys::NativeDisplayManager_CutoutInfo;

#[derive(Clone, Debug)]
pub struct Rect {
    pub left: i32,
    pub top: i32,
    pub width: u32,
    pub height: u32,
}

#[derive(Clone, Debug)]
pub struct WaterfallDisplayAreaRect {
    pub left: Rect,
    pub top: Rect,
    pub right: Rect,
    pub bottom: Rect,
}

#[derive(Clone, Debug)]
pub struct DisplayCutInfo {
    pub bounding_rect_info: i32,
    pub bounding_rect: Rect,
    pub waterfall_display_area_rect: WaterfallDisplayAreaRect,
}

impl From<NativeDisplayManager_CutoutInfo> for DisplayCutInfo {
    fn from(value: NativeDisplayManager_CutoutInfo) -> Self {
        DisplayCutInfo {
            bounding_rect_info: value.boundingRectsLength,
            bounding_rect: Rect {
                left: unsafe { *value.boundingRects }.top,
                top: unsafe { *value.boundingRects }.left,
                width: unsafe { *value.boundingRects }.width,
                height: unsafe { *value.boundingRects }.height,
            },
            waterfall_display_area_rect: WaterfallDisplayAreaRect {
                left: Rect {
                    left: value.waterfallDisplayAreaRects.left.left,
                    top: value.waterfallDisplayAreaRects.left.top,
                    width: value.waterfallDisplayAreaRects.left.width,
                    height: value.waterfallDisplayAreaRects.left.height,
                },
                top: Rect {
                    left: value.waterfallDisplayAreaRects.top.left,
                    top: value.waterfallDisplayAreaRects.top.top,
                    width: value.waterfallDisplayAreaRects.top.width,
                    height: value.waterfallDisplayAreaRects.top.height,
                },
                right: Rect {
                    left: value.waterfallDisplayAreaRects.right.left,
                    top: value.waterfallDisplayAreaRects.right.top,
                    width: value.waterfallDisplayAreaRects.right.width,
                    height: value.waterfallDisplayAreaRects.right.height,
                },
                bottom: Rect {
                    left: value.waterfallDisplayAreaRects.bottom.left,
                    top: value.waterfallDisplayAreaRects.bottom.top,
                    width: value.waterfallDisplayAreaRects.bottom.width,
                    height: value.waterfallDisplayAreaRects.bottom.height,
                },
            },
        }
    }
}
