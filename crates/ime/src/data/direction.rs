use ohos_input_method_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(InputMethod_Direction, "InputMethod_Direction_IME_DIRECTION_")]
pub enum Direction {
    None,
    Up,
    Down,
    Left,
    Right,
}
