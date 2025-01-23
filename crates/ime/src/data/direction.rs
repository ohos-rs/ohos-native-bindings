use ohos_input_method_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(InputMethod_Direction, "InputMethod_Direction_IME_DIRECTION_")]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}
