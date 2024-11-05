use napi_ohos::{bindgen_prelude::Either3, Result};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUINodeType,
    ARK_UI_NATIVE_NODE_API_1,
};

pub struct Text(ArkUINode);

impl Text {
    pub fn new() -> Result<Self> {
        let text = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::Text)?;
        Ok(Self(ArkUINode {
            raw: text,
            children: Vec::new(),
            tag: ArkUINodeType::Text,
        }))
    }

    pub fn node(&self) -> ArkUINode {
        self.0.clone()
    }

    pub fn set_font_size(&self, font_size: f32) -> Result<()> {
        let font_size_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(
                font_size,
            ))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::FontSize,
            font_size_property,
        )?;
        Ok(())
    }

    pub fn set_content<T: Into<String>>(&self, content: T) -> Result<()> {
        let content_property = ArkUINodeAttributeItem::String(content.into());
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::TextContent,
            content_property,
        )?;
        Ok(())
    }

    pub fn set_percent_width(&self, width: f32) -> Result<()> {
        let percent_width_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(width))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::WidthPercent,
            percent_width_property,
        )?;
        Ok(())
    }

    pub fn set_percent_height(&self, height: f32) -> Result<()> {
        let percent_height_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(height))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::HeightPercent,
            percent_height_property,
        )?;
        Ok(())
    }

    pub fn set_background_color(&self, color: u32) -> Result<()> {
        let background_color_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::C(color))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::BackgroundColor,
            background_color_property,
        )?;
        Ok(())
    }
}

impl From<Text> for ArkUINode {
    fn from(text: Text) -> Self {
        text.0
    }
}
