use ohos_input_method_sys::{
    InputMethod_CursorInfo, OH_CursorInfo_Create, OH_CursorInfo_Destroy, OH_CursorInfo_GetRect,
    OH_CursorInfo_SetRect,
};

pub struct Cursor {
    pub(crate) raw: *mut InputMethod_CursorInfo,
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub left: f64,
    pub top: f64,
    pub width: f64,
    pub height: f64,
}

impl Cursor {
    pub fn new(rect: Rect) -> Self {
        let raw = unsafe { OH_CursorInfo_Create(rect.left, rect.top, rect.width, rect.height) };
        Cursor { raw }
    }

    pub fn rect(&self) -> Rect {
        let mut left: f64 = 0.0;
        let mut top: f64 = 0.0;
        let mut width: f64 = 0.0;
        let mut height: f64 = 0.0;
        unsafe {
            OH_CursorInfo_GetRect(self.raw, &mut left, &mut top, &mut width, &mut height);
        }
        Rect {
            left,
            top,
            width,
            height,
        }
    }

    pub fn set_rect(&self, rect: Rect) {
        unsafe {
            OH_CursorInfo_SetRect(self.raw, rect.left, rect.top, rect.width, rect.height);
        }
    }
}

impl Drop for Cursor {
    fn drop(&mut self) {
        unsafe {
            OH_CursorInfo_Destroy(self.raw);
        }
    }
}
