//! Module component::attribute::font wrappers and related types.

use crate::{
    ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

use super::ArkUIAttributeBasic;

/// Font-related attribute helpers shared by text-capable components.
pub trait ArkUICommonFontAttribute: ArkUIAttributeBasic {
    fn font_size(&self, font_size: f32) -> ArkUIResult<()> {
        let font_size_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Float(font_size)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontSize,
                font_size_property,
            )
        })?;
        Ok(())
    }

    fn get_font_size(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontSize))
    }

    fn font_color(&self, font_color: u32) -> ArkUIResult<()> {
        let font_color_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Uint(font_color)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontColor,
                font_color_property,
            )
        })?;
        Ok(())
    }

    fn get_font_color(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontColor))
    }

    fn font_style(&self, font_style: i32) -> ArkUIResult<()> {
        let font_style_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(font_style)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontStyle,
                font_style_property,
            )
        })?;
        Ok(())
    }

    fn get_font_style(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontStyle))
    }

    fn font_weight(&self, font_weight: i32) -> ArkUIResult<()> {
        let font_weight_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(font_weight)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontWeight,
                font_weight_property,
            )
        })?;
        Ok(())
    }

    fn get_font_weight(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontWeight))
    }

    fn font_family<T: Into<String>>(&self, font_family: T) -> ArkUIResult<()> {
        let font_family_property = ArkUINodeAttributeItem::String(font_family.into());
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontFamily,
                font_family_property,
            )
        })?;
        Ok(())
    }

    fn get_font_family(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontFamily))
    }

    fn font_feature<T: Into<String>>(&self, font_feature: T) -> ArkUIResult<()> {
        let font_feature_property = ArkUINodeAttributeItem::String(font_feature.into());
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::FontFeature,
                font_feature_property,
            )
        })?;
        Ok(())
    }

    fn get_font_feature(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1
            .with(|api| api.get_attribute(self.raw(), crate::ArkUINodeAttributeType::FontFeature))
    }

    #[cfg(feature = "api-15")]
    fn immutable_font_weight(&self, font_weight: i32) -> ArkUIResult<()> {
        let font_weight_property =
            ArkUINodeAttributeItem::NumberValue(vec![ArkUINodeAttributeNumber::Int(font_weight)]);
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.set_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::ImmutableFontWeight,
                font_weight_property,
            )
        })?;
        Ok(())
    }

    #[cfg(feature = "api-15")]
    fn get_immutable_font_weight(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| {
            api.get_attribute(
                self.raw(),
                crate::ArkUINodeAttributeType::ImmutableFontWeight,
            )
        })
    }
}
