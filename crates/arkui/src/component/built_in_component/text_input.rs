use crate::{
    ArkUIAttributeBasic, ArkUICommonAttribute, ArkUIEvent, ArkUIGesture, ArkUINode, ArkUINodeType,
    ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

#[derive(Clone)]
pub struct TextInput(ArkUINode);

impl TextInput {
    pub fn new() -> ArkUIResult<Self> {
        let list_item = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::TextInput)?;
        Ok(Self(ArkUINode {
            raw: list_item,
            tag: ArkUINodeType::TextInput,
            ..Default::default()
        }))
    }
}

impl From<TextInput> for ArkUINode {
    fn from(list_item: TextInput) -> Self {
        list_item.0
    }
}

impl ArkUIAttributeBasic for TextInput {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        &mut self.0
    }
}

impl ArkUICommonAttribute for TextInput {}
impl ArkUIEvent for TextInput {}
impl ArkUIGesture for TextInput {}
