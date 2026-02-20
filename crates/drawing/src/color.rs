use ohos_drawing_sys::OH_Drawing_ColorSetArgb;

pub type Color = u32;

pub fn argb(alpha: u32, red: u32, green: u32, blue: u32) -> Color {
    unsafe { OH_Drawing_ColorSetArgb(alpha, red, green, blue) }
}
