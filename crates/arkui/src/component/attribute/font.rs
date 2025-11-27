use crate::{
    ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

use super::ArkUIAttributeBasic;

pub trait ArkUICommonFontAttribute: ArkUIAttributeBasic {
    fn font_size(&self, font_size: f32) -> ArkUIResult<()> {
        let font_size_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(font_size)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontSize,
                font_size_property,
            )
        })?;
        Ok(())
    }

    fn font_color(&self, font_color: u32) -> ArkUIResult<()> {
        let font_color_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Uint(font_color)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontColor,
                font_color_property,
            )
        })?;
        Ok(())
    }
}
