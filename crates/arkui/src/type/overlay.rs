//! Module type::overlay wrappers and related types.

use std::os::raw::c_void;

use crate::{
    Alignment, ArkUINode, ArkUINodeAttributeItem, ArkUINodeAttributeNumber,
    ArkUINodeCompositeAttributeItem,
};

use super::direction::Direction;

#[derive(Debug, Clone, Default)]
/// Overlay attribute payload builder.
pub struct OverlayOptions {
    text: Option<String>,
    alignment: Option<Alignment>,
    offset_x: Option<f32>,
    offset_y: Option<f32>,
    direction: Option<Direction>,
    node: Option<*mut c_void>,
}

impl OverlayOptions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn text<T: Into<String>>(&mut self, value: T) -> &mut Self {
        self.text = Some(value.into());
        self
    }

    pub fn clear_text(&mut self) -> &mut Self {
        self.text = None;
        self
    }

    pub fn alignment(&mut self, value: Alignment) -> &mut Self {
        self.alignment = Some(value);
        self
    }

    pub fn offset(&mut self, x: f32, y: f32) -> &mut Self {
        self.offset_x = Some(x);
        self.offset_y = Some(y);
        self
    }

    pub fn offset_x(&mut self, value: f32) -> &mut Self {
        self.offset_x = Some(value);
        self
    }

    pub fn offset_y(&mut self, value: f32) -> &mut Self {
        self.offset_y = Some(value);
        self
    }

    pub fn direction(&mut self, value: Direction) -> &mut Self {
        self.direction = Some(value);
        self
    }

    pub fn node(&mut self, value: &ArkUINode) -> &mut Self {
        self.node = Some(value.raw_handle().cast());
        self
    }

    pub fn raw_node(&mut self, value: *mut c_void) -> &mut Self {
        self.node = Some(value);
        self
    }

    pub fn clear_node(&mut self) -> &mut Self {
        self.node = None;
        self
    }

    fn number_values(&self) -> Vec<ArkUINodeAttributeNumber> {
        let include_direction = self.direction.is_some();
        let include_offsets =
            include_direction || self.offset_x.is_some() || self.offset_y.is_some();
        let include_alignment = include_offsets || self.alignment.is_some();
        let mut values = Vec::new();

        if include_alignment {
            values.push(ArkUINodeAttributeNumber::Int(
                self.alignment.unwrap_or(Alignment::TopStart).into(),
            ));
        }
        if include_offsets {
            values.push(ArkUINodeAttributeNumber::Float(
                self.offset_x.unwrap_or(0.0),
            ));
            values.push(ArkUINodeAttributeNumber::Float(
                self.offset_y.unwrap_or(0.0),
            ));
        }
        if let Some(direction) = self.direction {
            values.push(ArkUINodeAttributeNumber::Int(direction.into()));
        }

        values
    }
}

impl From<&OverlayOptions> for ArkUINodeAttributeItem {
    fn from(value: &OverlayOptions) -> Self {
        let mut item =
            ArkUINodeCompositeAttributeItem::new().with_number_values(value.number_values());
        if let Some(text) = &value.text {
            item = item.with_string(text.clone());
        }
        if let Some(node) = value.node {
            item = item.with_object(node);
        }
        item.into()
    }
}

impl From<OverlayOptions> for ArkUINodeAttributeItem {
    fn from(value: OverlayOptions) -> Self {
        let mut item =
            ArkUINodeCompositeAttributeItem::new().with_number_values(value.number_values());
        if let Some(text) = value.text {
            item = item.with_string(text);
        }
        if let Some(node) = value.node {
            item = item.with_object(node);
        }
        item.into()
    }
}
