use napi_ohos::Result;

use crate::{ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ARK_UI_NATIVE_NODE_API_1};

use super::ArkUIAttributeBasic;

pub trait ArkUICommonFontAttribute: ArkUIAttributeBasic {
    fn set_font_size(&self, font_size: f32) -> Result<()> {
        let font_size_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(font_size)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::FontSize,
            font_size_property,
        )?;
        Ok(())
    }

    fn set_font_color(&self, font_color: u32) -> Result<()> {
        let font_color_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Uint(font_color)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::FontColor,
            font_color_property,
        )?;
        Ok(())
    }
}
