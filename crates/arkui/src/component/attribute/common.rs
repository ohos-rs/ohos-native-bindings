//! Module component::attribute::common wrappers and related types.

use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use crate::{
    ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber, ArkUINodeAttributeType,
    ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

/// Minimal node access required by all attribute traits.
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

    fn reset_attribute(&self, attribute: ArkUINodeAttributeType) -> ArkUIResult<()> {
        ARK_UI_NATIVE_NODE_API_1.with(|api| api.reset_attribute(self.raw(), attribute))
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

    // BEGIN_GENERATED_COMMON_ATTRIBUTE_METHODS
    fn set_background_image<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BackgroundImage, value.into())
    }

    fn get_background_image(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundImage)
    }

    fn set_padding<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Padding, value.into())
    }

    fn get_padding(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Padding)
    }

    fn set_margin<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Margin, value.into())
    }

    fn get_margin(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Margin)
    }

    fn set_translate<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Translate, value.into())
    }

    fn get_translate(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Translate)
    }

    fn set_scale<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Scale, value.into())
    }

    fn get_scale(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Scale)
    }

    fn set_rotate<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Rotate, value.into())
    }

    fn get_rotate(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Rotate)
    }

    fn set_brightness<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Brightness, value.into())
    }

    fn get_brightness(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Brightness)
    }

    fn set_saturation<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Saturation, value.into())
    }

    fn get_saturation(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Saturation)
    }

    fn set_blur<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Blur, value.into())
    }

    fn get_blur(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Blur)
    }

    fn set_linear_gradient<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::LinearGradient, value.into())
    }

    fn get_linear_gradient(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::LinearGradient)
    }

    fn set_alignment<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Alignment, value.into())
    }

    fn get_alignment(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Alignment)
    }

    fn set_border_width<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BorderWidth, value.into())
    }

    fn get_border_width(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderWidth)
    }

    fn set_border_radius<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BorderRadius, value.into())
    }

    fn get_border_radius(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderRadius)
    }

    fn set_border_color<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BorderColor, value.into())
    }

    fn get_border_color(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderColor)
    }

    fn set_border_style<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BorderStyle, value.into())
    }

    fn get_border_style(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderStyle)
    }

    fn set_visibility<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Visibility, value.into())
    }

    fn get_visibility(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Visibility)
    }

    fn set_clip<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Clip, value.into())
    }

    fn get_clip(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Clip)
    }

    fn set_clip_shape<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ClipShape, value.into())
    }

    fn get_clip_shape(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ClipShape)
    }

    fn set_transform<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Transform, value.into())
    }

    fn get_transform(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Transform)
    }

    fn set_hit_test_behavior<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::HitTestBehavior, value.into())
    }

    fn get_hit_test_behavior(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::HitTestBehavior)
    }

    fn set_position<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Position, value.into())
    }

    fn get_position(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Position)
    }

    fn set_shadow<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Shadow, value.into())
    }

    fn get_shadow(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Shadow)
    }

    fn set_custom_shadow<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::CustomShadow, value.into())
    }

    fn get_custom_shadow(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::CustomShadow)
    }

    fn set_background_image_size<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BackgroundImageSize,
            value.into(),
        )
    }

    fn get_background_image_size(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundImageSize)
    }

    fn set_background_image_size_with_style<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BackgroundImageSizeWithStyle,
            value.into(),
        )
    }

    fn get_background_image_size_with_style(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundImageSizeWithStyle)
    }

    fn set_background_blur_style<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BackgroundBlurStyle,
            value.into(),
        )
    }

    fn get_background_blur_style(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundBlurStyle)
    }

    fn set_transform_center<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::TransformCenter, value.into())
    }

    fn get_transform_center(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::TransformCenter)
    }

    fn set_opacity_transition<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::OpacityTransition,
            value.into(),
        )
    }

    fn get_opacity_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::OpacityTransition)
    }

    fn set_rotate_transition<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::RotateTransition,
            value.into(),
        )
    }

    fn get_rotate_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RotateTransition)
    }

    fn set_scale_transition<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ScaleTransition, value.into())
    }

    fn get_scale_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ScaleTransition)
    }

    fn set_translate_transition<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::TranslateTransition,
            value.into(),
        )
    }

    fn get_translate_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::TranslateTransition)
    }

    fn set_move_transition<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::MoveTransition, value.into())
    }

    fn get_move_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::MoveTransition)
    }

    fn set_focusable<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Focusable, value.into())
    }

    fn get_focusable(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Focusable)
    }

    fn set_default_focus<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::DefaultFocus, value.into())
    }

    fn get_default_focus(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::DefaultFocus)
    }

    fn set_response_region<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ResponseRegion, value.into())
    }

    fn get_response_region(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ResponseRegion)
    }

    fn set_overlay<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Overlay, value.into())
    }

    fn get_overlay(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Overlay)
    }

    fn reset_overlay(&self) -> ArkUIResult<()> {
        self.reset_attribute(crate::ArkUINodeAttributeType::Overlay)
    }

    fn set_sweep_gradient<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::SweepGradient, value.into())
    }

    fn get_sweep_gradient(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::SweepGradient)
    }

    fn set_radial_gradient<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::RadialGradient, value.into())
    }

    fn get_radial_gradient(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RadialGradient)
    }

    fn set_mask<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Mask, value.into())
    }

    fn get_mask(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Mask)
    }

    fn set_blend_mode<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BlendMode, value.into())
    }

    fn get_blend_mode(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BlendMode)
    }

    fn set_direction<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Direction, value.into())
    }

    fn get_direction(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Direction)
    }

    fn set_constraint_size<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ConstraintSize, value.into())
    }

    fn get_constraint_size(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ConstraintSize)
    }

    fn set_gray_scale<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::GrayScale, value.into())
    }

    fn get_gray_scale(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::GrayScale)
    }

    fn set_invert<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Invert, value.into())
    }

    fn get_invert(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Invert)
    }

    fn set_sepia<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Sepia, value.into())
    }

    fn get_sepia(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Sepia)
    }

    fn set_contrast<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Contrast, value.into())
    }

    fn get_contrast(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Contrast)
    }

    fn set_foreground_color<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ForegroundColor, value.into())
    }

    fn get_foreground_color(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ForegroundColor)
    }

    fn set_offset<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Offset, value.into())
    }

    fn get_offset(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Offset)
    }

    fn set_mark_anchor<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::MarkAnchor, value.into())
    }

    fn get_mark_anchor(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::MarkAnchor)
    }

    fn set_background_image_position<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BackgroundImagePosition,
            value.into(),
        )
    }

    fn get_background_image_position(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundImagePosition)
    }

    fn set_align_rules<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::AlignRules, value.into())
    }

    fn get_align_rules(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AlignRules)
    }

    fn set_align_self<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::AlignSelf, value.into())
    }

    fn get_align_self(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AlignSelf)
    }

    fn set_accessibility_group<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityGroup,
            value.into(),
        )
    }

    fn get_accessibility_group(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityGroup)
    }

    fn set_accessibility_text<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityText,
            value.into(),
        )
    }

    fn get_accessibility_text(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityText)
    }

    fn set_accessibility_mode<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityMode,
            value.into(),
        )
    }

    fn get_accessibility_mode(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityMode)
    }

    fn set_accessibility_description<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityDescription,
            value.into(),
        )
    }

    fn get_accessibility_description(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityDescription)
    }

    fn set_focus_status<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::FocusStatus, value.into())
    }

    fn get_focus_status(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::FocusStatus)
    }

    fn set_aspect_ratio<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::AspectRatio, value.into())
    }

    fn get_aspect_ratio(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AspectRatio)
    }

    fn set_layout_weight<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::LayoutWeight, value.into())
    }

    fn get_layout_weight(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::LayoutWeight)
    }

    fn set_display_priority<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::DisplayPriority, value.into())
    }

    fn get_display_priority(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::DisplayPriority)
    }

    fn set_outline_width<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::OutlineWidth, value.into())
    }

    fn get_outline_width(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::OutlineWidth)
    }

    fn set_padding_percent<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::PaddingPercent, value.into())
    }

    fn get_padding_percent(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::PaddingPercent)
    }

    fn set_margin_percent<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::MarginPercent, value.into())
    }

    fn get_margin_percent(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::MarginPercent)
    }

    fn set_geometry_transition<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::GeometryTransition,
            value.into(),
        )
    }

    fn get_geometry_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::GeometryTransition)
    }

    fn set_relative_layout_chain_mode<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::RelativeLayoutChainMode,
            value.into(),
        )
    }

    fn get_relative_layout_chain_mode(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RelativeLayoutChainMode)
    }

    fn set_render_fit<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::RenderFit, value.into())
    }

    fn get_render_fit(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RenderFit)
    }

    fn set_outline_color<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::OutlineColor, value.into())
    }

    fn get_outline_color(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::OutlineColor)
    }

    fn set_size<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Size, value.into())
    }

    fn get_size(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Size)
    }

    fn set_render_group<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::RenderGroup, value.into())
    }

    fn get_render_group(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RenderGroup)
    }

    fn set_color_blend<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ColorBlend, value.into())
    }

    fn get_color_blend(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ColorBlend)
    }

    fn set_foreground_blur_style<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::ForegroundBlurStyle,
            value.into(),
        )
    }

    fn get_foreground_blur_style(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ForegroundBlurStyle)
    }

    fn set_layout_rect<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::LayoutRect, value.into())
    }

    fn get_layout_rect(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::LayoutRect)
    }

    fn set_focus_on_touch<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::FocusOnTouch, value.into())
    }

    fn get_focus_on_touch(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::FocusOnTouch)
    }

    fn set_border_width_percent<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BorderWidthPercent,
            value.into(),
        )
    }

    fn get_border_width_percent(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderWidthPercent)
    }

    fn set_border_radius_percent<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BorderRadiusPercent,
            value.into(),
        )
    }

    fn get_border_radius_percent(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BorderRadiusPercent)
    }

    fn set_accessibility_id<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::AccessibilityId, value.into())
    }

    fn get_accessibility_id(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityId)
    }

    fn set_accessibility_actions<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityActions,
            value.into(),
        )
    }

    fn get_accessibility_actions(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityActions)
    }

    fn set_accessibility_role<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityRole,
            value.into(),
        )
    }

    fn get_accessibility_role(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityRole)
    }

    fn set_accessibility_state<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityState,
            value.into(),
        )
    }

    fn get_accessibility_state(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityState)
    }

    fn set_accessibility_value<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::AccessibilityValue,
            value.into(),
        )
    }

    fn get_accessibility_value(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AccessibilityValue)
    }

    fn set_expand_safe_area<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ExpandSafeArea, value.into())
    }

    fn get_expand_safe_area(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ExpandSafeArea)
    }

    fn set_visible_area_change_ratio<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::VisibleAreaChangeRatio,
            value.into(),
        )
    }

    fn get_visible_area_change_ratio(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::VisibleAreaChangeRatio)
    }

    #[cfg(feature = "api-17")]
    fn set_visible_area_change_options(
        &self,
        options: &crate::VisibleAreaEventOptions,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::VisibleAreaChangeRatio,
            crate::ArkUINodeAttributeItem::Object(options.raw().cast()),
        )
    }

    #[cfg(feature = "api-17")]
    fn get_visible_area_change_options(
        &self,
    ) -> ArkUIResult<Option<crate::VisibleAreaEventOptions>> {
        match self.get_attribute(crate::ArkUINodeAttributeType::VisibleAreaChangeRatio)? {
            ArkUINodeAttributeItem::Object(ptr) => {
                Ok(Some(crate::VisibleAreaEventOptions::from_raw(ptr.cast())))
            }
            _ => Ok(None),
        }
    }

    fn set_transition<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::Transition, value.into())
    }

    fn get_transition(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::Transition)
    }

    fn set_unique_id<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::UniqueId, value.into())
    }

    fn get_unique_id(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::UniqueId)
    }

    fn set_focus_box<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::FocusBox, value.into())
    }

    fn get_focus_box(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::FocusBox)
    }

    fn set_click_distance<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::ClickDistance, value.into())
    }

    fn get_click_distance(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::ClickDistance)
    }

    #[cfg(feature = "api-21")]
    fn set_allow_force_dark<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::AllowForceDark, value.into())
    }

    #[cfg(feature = "api-21")]
    fn get_allow_force_dark(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::AllowForceDark)
    }

    #[cfg(feature = "api-15")]
    fn set_backdrop_blur<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::BackdropBlur, value.into())
    }

    #[cfg(feature = "api-15")]
    fn get_backdrop_blur(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackdropBlur)
    }

    #[cfg(feature = "api-19")]
    fn set_background_image_resizable_with_slice<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::BackgroundImageResizableWithSlice,
            value.into(),
        )
    }

    #[cfg(feature = "api-19")]
    fn get_background_image_resizable_with_slice(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::BackgroundImageResizableWithSlice)
    }

    #[cfg(feature = "api-21")]
    fn set_height_layoutpolicy<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::HeightLayoutpolicy,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    fn get_height_layoutpolicy(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::HeightLayoutpolicy)
    }

    #[cfg(feature = "api-18")]
    fn set_next_focus<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::NextFocus, value.into())
    }

    #[cfg(feature = "api-18")]
    fn get_next_focus(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::NextFocus)
    }

    #[cfg(feature = "api-21")]
    fn set_pixel_round<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::PixelRound, value.into())
    }

    #[cfg(feature = "api-21")]
    fn get_pixel_round(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::PixelRound)
    }

    #[cfg(feature = "api-21")]
    fn set_position_edges<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::PositionEdges, value.into())
    }

    #[cfg(feature = "api-21")]
    fn get_position_edges(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::PositionEdges)
    }

    #[cfg(feature = "api-20")]
    fn set_rotate_angle<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::RotateAngle, value.into())
    }

    #[cfg(feature = "api-20")]
    fn get_rotate_angle(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::RotateAngle)
    }

    #[cfg(feature = "api-14")]
    fn set_tab_stop<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(crate::ArkUINodeAttributeType::TabStop, value.into())
    }

    #[cfg(feature = "api-14")]
    fn get_tab_stop(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::TabStop)
    }

    #[cfg(feature = "api-20")]
    fn set_translate_with_percent<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::TranslateWithPercent,
            value.into(),
        )
    }

    #[cfg(feature = "api-20")]
    fn get_translate_with_percent(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::TranslateWithPercent)
    }

    #[cfg(feature = "api-17")]
    fn set_visible_area_approximate_change_ratio<T: Into<ArkUINodeAttributeItem>>(
        &self,
        value: T,
    ) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::VisibleAreaApproximateChangeRatio,
            value.into(),
        )
    }

    #[cfg(feature = "api-17")]
    fn get_visible_area_approximate_change_ratio(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::VisibleAreaApproximateChangeRatio)
    }

    #[cfg(feature = "api-21")]
    fn set_width_layoutpolicy<T: Into<ArkUINodeAttributeItem>>(&self, value: T) -> ArkUIResult<()> {
        self.set_attribute(
            crate::ArkUINodeAttributeType::WidthLayoutpolicy,
            value.into(),
        )
    }

    #[cfg(feature = "api-21")]
    fn get_width_layoutpolicy(&self) -> ArkUIResult<ArkUINodeAttributeItem> {
        self.get_attribute(crate::ArkUINodeAttributeType::WidthLayoutpolicy)
    }

    // END_GENERATED_COMMON_ATTRIBUTE_METHODS
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
