use crate::{
    ArkUIAttributeBasic, ArkUICommonAttribute, ArkUINode, ArkUINodeType, ArkUIResult,
    ARK_UI_NATIVE_NODE_API_1,
};

pub struct ListItem(ArkUINode);

impl ListItem {
    pub fn new() -> ArkUIResult<Self> {
        let list_item = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::ListItem)?;
        Ok(Self(ArkUINode {
            raw: list_item,
            children: Vec::new(),
            tag: ArkUINodeType::ListItem,
        }))
    }
}

impl From<ListItem> for ArkUINode {
    fn from(list_item: ListItem) -> Self {
        list_item.0
    }
}

impl ArkUIAttributeBasic for ListItem {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        &mut self.0
    }
}

impl ArkUICommonAttribute for ListItem {}
