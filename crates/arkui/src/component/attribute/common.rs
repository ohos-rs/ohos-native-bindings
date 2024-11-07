use napi_ohos::Result;

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ARK_UI_NATIVE_NODE_API_1,
};

pub trait ArkUIAttributeBasic {
    /// Make sure every node can get ArkUINode for built-in method with current trait
    fn raw(&self) -> &ArkUINode;

    fn borrow_mut(&mut self) -> &mut ArkUINode;
}

/// This trait is used to set common attribute for node.
/// Every node should implement this trait, include the custom node.
pub trait ArkUICommonAttribute: ArkUIAttributeBasic {
    /// Set node height
    fn set_width(&self, width: f32) -> Result<()> {
        let width_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(width)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::Width,
            width_property,
        )?;
        Ok(())
    }

    /// Set node height
    fn set_height(&self, height: f32) -> Result<()> {
        let height_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(height)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::Height,
            height_property,
        )?;
        Ok(())
    }

    /// Set percent width
    fn set_percent_width(&self, width: f32) -> Result<()> {
        let percent_width_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(width)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::WidthPercent,
            percent_width_property,
        )?;
        Ok(())
    }

    /// Set percent height
    fn set_percent_height(&self, height: f32) -> Result<()> {
        let percent_height_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(height)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::HeightPercent,
            percent_height_property,
        )?;
        Ok(())
    }

    /// Set background-color
    fn set_background_color(&self, color: u32) -> Result<()> {
        let background_color_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Uint(color)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::BackgroundColor,
            background_color_property,
        )?;
        Ok(())
    }

    /// Remove child node
    fn remove_child(&mut self, index: usize) -> Option<Box<ArkUINode>> {
        let mut children = self.borrow_mut();
        if index < children.children().len() {
            let removed_node = children.children_mut().remove(index);
            ARK_UI_NATIVE_NODE_API_1.remove_child(self.raw(), &removed_node);
            Some(removed_node)
        } else {
            None
        }
    }

    fn add_child<T: Into<ArkUINode>>(&mut self, child: T) {
        let child_handle = child.into();
        ARK_UI_NATIVE_NODE_API_1.add_child(self.raw(), &child_handle);
        self.borrow_mut()
            .children_mut()
            .push(Box::new(child_handle));
    }

    fn insert_child<T: Into<ArkUINode>>(&mut self, child: T, index: usize) {
        let child_handle = child.into();
        ARK_UI_NATIVE_NODE_API_1.insert_child(self.raw(), &child_handle, index as i32);
        self.borrow_mut()
            .children_mut()
            .insert(index, Box::new(child_handle));
    }
}
