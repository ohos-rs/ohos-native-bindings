use ohos_xcomponent_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    OH_NativeXComponent_TouchPointToolType,
    "OH_NativeXComponent_TouchPointToolType_OH_NATIVEXCOMPONENT_TOOL_TYPE_"
)]
pub enum TouchPointTool {
    Unknown,
    Finger,
    Pen,
    Rubber,
    Brush,
    Pencil,
    Airbrush,
    Mouse,
    Lens,
}
