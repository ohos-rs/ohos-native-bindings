use crate::{
    ArkUIAttributeBasic, ArkUICommonFontAttribute, ArkUIError, ArkUIEvent, ArkUIGesture, ArkUINode,
    ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUINodeType, TextAlignment,
    ARK_UI_NATIVE_NODE_API_1,
};

use crate::component::ArkUICommonAttribute;

pub struct Text(ArkUINode);

impl Text {
    pub fn new() -> Result<Self, ArkUIError> {
        let text = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::Text)?;
        Ok(Self(ArkUINode {
            raw: text,
            tag: ArkUINodeType::Text,
            ..Default::default()
        }))
    }

    pub fn content<T: Into<String>>(&self, content: T) -> Result<(), ArkUIError> {
        let content_property = ArkUINodeAttributeItem::String(content.into());
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::TextContent,
            content_property,
        )?;
        Ok(())
    }

    pub fn alignment(&self, alignment: TextAlignment) -> Result<(), ArkUIError> {
        let alignment_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(
                alignment.into(),
            )]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::TextAlign,
            alignment_property,
        )?;
        Ok(())
    }
}

impl From<Text> for ArkUINode {
    fn from(text: Text) -> Self {
        text.0
    }
}

impl ArkUIAttributeBasic for Text {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        &mut self.0
    }
}

impl ArkUICommonAttribute for Text {}
impl ArkUICommonFontAttribute for Text {}
impl ArkUIEvent for Text {}
impl ArkUIGesture for Text {}
