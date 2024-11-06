use napi_ohos::{bindgen_prelude::Either3, Result};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUINodeType,
    ARK_UI_NATIVE_NODE_API_1,
};

use crate::component::ArkUICommonAttribute;

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
}

impl From<Text> for ArkUINode {
    fn from(text: Text) -> Self {
        text.0
    }
}

impl ArkUICommonAttribute for Text {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }
}
