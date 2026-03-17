use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUINodeAttributeType,
    ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

pub trait ArkUIAttributeBasic {
    /// Make sure every node can get ArkUINode for built-in method with current trait
    fn raw(&self) -> &ArkUINode;

    fn borrow_mut(&mut self) -> &mut ArkUINode;
}

/// This trait is used to set common attribute for node.
/// Every node should implement this trait, include the custom node.
pub trait ArkUICommonAttribute: ArkUIAttributeBasic {
    fn set_attribute(
        &self,
        attribute: ArkUINodeAttributeType,
        value: ArkUINodeAttributeItem,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.set_attribute(self.raw(), attribute, value))
    }

    fn get_attribute(
        &self,
        attribute: ArkUINodeAttributeType,
    ) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.get_attribute(self.raw(), attribute))
    }

    fn set_number_attribute(
        &self,
        attribute: ArkUINodeAttributeType,
        values: Vec<ArkUINodeAttributeNumber>,
    ) -> ArkUIResult<()> {
        self.set_attribute(attribute, ArkUINodeAttributeItem::NumberValue(values))
    }

    fn set_i32_attribute(&self, attribute: ArkUINodeAttributeType, value: i32) -> ArkUIResult<()> {
        self.set_number_attribute(attribute, vec![ArkUINodeAttributeNumber::Int(value)])
    }

    fn set_u32_attribute(&self, attribute: ArkUINodeAttributeType, value: u32) -> ArkUIResult<()> {
        self.set_number_attribute(attribute, vec![ArkUINodeAttributeNumber::Uint(value)])
    }

    fn set_f32_attribute(&self, attribute: ArkUINodeAttributeType, value: f32) -> ArkUIResult<()> {
        self.set_number_attribute(attribute, vec![ArkUINodeAttributeNumber::Float(value)])
    }

    fn set_bool_attribute(
        &self,
        attribute: ArkUINodeAttributeType,
        value: bool,
    ) -> ArkUIResult<()> {
        self.set_i32_attribute(attribute, if value { 1 } else { 0 })
    }

    fn set_string_attribute<T: Into<String>>(
        &self,
        attribute: ArkUINodeAttributeType,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(attribute, ArkUINodeAttributeItem::String(value.into()))
    }

    fn set_object_attribute(
        &self,
        attribute: ArkUINodeAttributeType,
        value: *mut ::std::os::raw::c_void,
    ) -> ArkUIResult<()> {
        self.set_attribute(attribute, ArkUINodeAttributeItem::Object(value))
    }

    /// Set node height
    fn width(&self, width: f32) -> ArkUIResult<()> {
        self.set_f32_attribute(crate::ArkUINodeAttributeType::Width, width)
    }

    /// Set node height
    fn height(&self, height: f32) -> ArkUIResult<()> {
        self.set_f32_attribute(crate::ArkUINodeAttributeType::Height, height)
    }

    /// Set percent width
    fn percent_width(&self, width: f32) -> ArkUIResult<()> {
        self.set_f32_attribute(crate::ArkUINodeAttributeType::WidthPercent, width)
    }

    /// Set percent height
    fn percent_height(&self, height: f32) -> ArkUIResult<()> {
        self.set_f32_attribute(crate::ArkUINodeAttributeType::HeightPercent, height)
    }

    /// Set background-color
    fn background_color(&self, color: u32) -> ArkUIResult<()> {
        self.set_u32_attribute(crate::ArkUINodeAttributeType::BackgroundColor, color)
    }

    fn opacity(&self, opacity: f32) -> ArkUIResult<()> {
        self.set_f32_attribute(crate::ArkUINodeAttributeType::Opacity, opacity)
    }

    fn enabled(&self, enabled: bool) -> ArkUIResult<()> {
        self.set_bool_attribute(crate::ArkUINodeAttributeType::Enabled, enabled)
    }

    fn id<T: Into<String>>(&self, id: T) -> ArkUIResult<()> {
        self.set_string_attribute(crate::ArkUINodeAttributeType::Id, id)
    }

    fn z_index(&self, z_index: i32) -> ArkUIResult<()> {
        self.set_i32_attribute(crate::ArkUINodeAttributeType::ZIndex, z_index)
    }

    /// Remove child node
    fn remove_child(&mut self, index: usize) -> ArkUIResult<Option<Rc<RefCell<ArkUINode>>>> {
        let children = self.borrow_mut();
        if index < children.children().len() {
            let removed_node = children.children_mut().remove(index);
            ARK_UI_NATIVE_NODE_API_1
                .with(|api| api.remove_child(self.raw(), &removed_node.borrow()))?;
            Ok(Some(removed_node))
        } else {
            Ok(None)
        }
    }

    fn add_child<T: Into<ArkUINode>>(&mut self, child: T) -> ArkUIResult<()> {
        let child_handle: Rc<RefCell<ArkUINode>> = Rc::new(RefCell::new(child.into()));

        let child_handle_clone = child_handle.clone();
        // save self ArkUINode to custom user data for event dispatch
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_user_data(
                &child_handle.borrow(),
                Box::into_raw(Box::new(child_handle_clone)) as *mut c_void,
            )
        })?;
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_event_receiver(&child_handle.borrow()))?;

        ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_child(self.raw(), &child_handle.borrow()))?;
        self.borrow_mut().children_mut().push(child_handle);
        Ok(())
    }

    fn insert_child<T: Into<ArkUINode>>(&mut self, child: T, index: usize) -> ArkUIResult<()> {
        let child_handle: Rc<RefCell<ArkUINode>> = Rc::new(RefCell::new(child.into()));
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.insert_child(self.raw(), &child_handle.borrow(), index as i32))?;
        self.borrow_mut()
            .children_mut()
            .insert(index, child_handle.clone());
        Ok(())
    }
}
