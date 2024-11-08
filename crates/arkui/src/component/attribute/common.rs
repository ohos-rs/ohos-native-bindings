use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUIResult,
    ARK_UI_NATIVE_NODE_API_1,
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
    fn set_width(&self, width: f32) -> ArkUIResult<()> {
        let width_property: ArkUINodeAttributeItem =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(width)]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::Width,
            width_property,
        )?;
        Ok(())
    }

    /// Set node height
    fn set_height(&self, height: f32) -> ArkUIResult<()> {
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
    fn set_percent_width(&self, width: f32) -> ArkUIResult<()> {
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
    fn set_percent_height(&self, height: f32) -> ArkUIResult<()> {
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
    fn set_background_color(&self, color: u32) -> ArkUIResult<()> {
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
    fn remove_child(&mut self, index: usize) -> ArkUIResult<Option<Rc<RefCell<ArkUINode>>>> {
        let children = self.borrow_mut();
        if index < children.children().len() {
            let removed_node = children.children_mut().remove(index);
            ARK_UI_NATIVE_NODE_API_1.remove_child(self.raw(), &removed_node.borrow())?;
            Ok(Some(removed_node))
        } else {
            Ok(None)
        }
    }

    fn add_child<T: Into<ArkUINode>>(&mut self, child: T) -> ArkUIResult<()> {
        let child_handle: Rc<RefCell<ArkUINode>> = Rc::new(RefCell::new(child.into()));

        let child_handle_clone = child_handle.clone();
        // save self ArkUINode to custom user data for event dispatch
        ARK_UI_NATIVE_NODE_API_1.set_user_data(
            &child_handle.borrow(),
            Box::into_raw(Box::new(child_handle_clone)) as *mut c_void,
        )?;
        ARK_UI_NATIVE_NODE_API_1.add_event_receiver(&child_handle.borrow())?;

        ARK_UI_NATIVE_NODE_API_1.add_child(self.raw(), &child_handle.borrow())?;
        self.borrow_mut().children_mut().push(child_handle);
        Ok(())
    }

    fn insert_child<T: Into<ArkUINode>>(&mut self, child: T, index: usize) -> ArkUIResult<()> {
        let child_handle: Rc<RefCell<ArkUINode>> = Rc::new(RefCell::new(child.into()));
        ARK_UI_NATIVE_NODE_API_1.insert_child(self.raw(), &child_handle.borrow(), index as i32)?;
        self.borrow_mut()
            .children_mut()
            .insert(index, child_handle.clone());
        Ok(())
    }
}
