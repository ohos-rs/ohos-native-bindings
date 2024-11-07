use crate::{
    ArkUIAttributeBasic, ArkUICommonAttribute, ArkUINode, ArkUINodeAttributeItem,
    ArkUINodeAttributeNumber, ArkUINodeType, ArkUIResult, ScrollBarDisplayMode,
    ARK_UI_NATIVE_NODE_API_1,
};

pub struct List(ArkUINode);

impl List {
    pub fn new() -> ArkUIResult<Self> {
        let list = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::List)?;
        Ok(Self(ArkUINode {
            raw: list,
            children: Vec::new(),
            tag: ArkUINodeType::List,
        }))
    }

    pub fn set_scroll_bar_state(&mut self, mode: ScrollBarDisplayMode) -> ArkUIResult<()> {
        let scroll_bar_display_mode_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(mode.into())]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            &self.0,
            crate::ArkUINodeAttributeType::ScrollBarDisplayMode,
            scroll_bar_display_mode_property,
        )?;
        Ok(())
    }
}

impl From<List> for ArkUINode {
    fn from(list: List) -> Self {
        list.0
    }
}

impl ArkUIAttributeBasic for List {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        &mut self.0
    }
}

impl ArkUICommonAttribute for List {}
