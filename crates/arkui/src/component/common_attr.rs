use napi_ohos::{bindgen_prelude::Either3, Result};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ARK_UI_NATIVE_NODE_API_1,
};

/// This trait is used to set common attribute for node.
/// Every node should implement this trait, include the custom node.
pub trait ArkUICommonAttribute {
    /// Make sure every node can get ArkUINode for built-in method with current trait
    fn raw(&self) -> &ArkUINode;

    /// Set node height
    fn set_height(&self, height: f32) -> Result<()> {
        let percent_width_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(height))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::Height,
            percent_width_property,
        )?;
        Ok(())
    }

    /// Set percent width
    fn set_percent_width(&self, width: f32) -> Result<()> {
        let percent_width_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(width))]);
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
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::A(height))]);
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
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber(Either3::C(color))]);
        ARK_UI_NATIVE_NODE_API_1.set_attribute(
            self.raw(),
            crate::ArkUINodeAttributeType::BackgroundColor,
            background_color_property,
        )?;
        Ok(())
    }
}
