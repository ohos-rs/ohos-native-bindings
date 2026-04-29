//! Attribute traits shared by built-in and custom components.

mod common;
mod event;
mod font;
mod gesture;

pub use common::{ArkUIAttributeBasic, ArkUICommonAttribute};
pub use event::ArkUIEvent;
pub use font::ArkUICommonFontAttribute;
pub use gesture::ArkUIGesture;
