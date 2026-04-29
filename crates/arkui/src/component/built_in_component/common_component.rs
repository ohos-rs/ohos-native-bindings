//! Module component::built_in_component::common_component wrappers and related types.

use crate::{
    ArkUIAttributeBasic, ArkUICommonAttribute, ArkUIEvent, ArkUIGesture, ArkUINode, ArkUINodeType,
    ArkUIResult, ARK_UI_NATIVE_NODE_API_1,
};

macro_rules! define_common_component {
    ($name:ident, $tag:ident) => {
        pub struct $name(pub(crate) ArkUINode);

        impl $name {
            pub fn new() -> ArkUIResult<Self> {
                let node =
                    ARK_UI_NATIVE_NODE_API_1.with(|api| api.create_node(ArkUINodeType::$tag))?;
                Ok(Self(ArkUINode {
                    raw: node,
                    tag: ArkUINodeType::$tag,
                    ..Default::default()
                }))
            }
        }

        impl From<$name> for ArkUINode {
            fn from(node: $name) -> Self {
                node.0
            }
        }

        impl ArkUIAttributeBasic for $name {
            fn raw(&self) -> &ArkUINode {
                &self.0
            }

            fn borrow_mut(&mut self) -> &mut ArkUINode {
                &mut self.0
            }
        }

        impl ArkUICommonAttribute for $name {}
        impl ArkUIEvent for $name {}
        impl ArkUIGesture for $name {}
    };
}

define_common_component!(Custom, Custom);
define_common_component!(Text, Text);
define_common_component!(TextInput, TextInput);
define_common_component!(XComponent, XComponent);
define_common_component!(Span, Span);
define_common_component!(ImageSpan, ImageSpan);
define_common_component!(Image, Image);
define_common_component!(Toggle, Toggle);
define_common_component!(LoadingProgress, LoadingProgress);
define_common_component!(TextArea, TextArea);
define_common_component!(Button, Button);
define_common_component!(Progress, Progress);
define_common_component!(Checkbox, Checkbox);
define_common_component!(DatePicker, DatePicker);
define_common_component!(TimePicker, TimePicker);
define_common_component!(TextPicker, TextPicker);
define_common_component!(CalendarPicker, CalendarPicker);
define_common_component!(Slider, Slider);
define_common_component!(Radio, Radio);
define_common_component!(ImageAnimator, ImageAnimator);
#[cfg(feature = "api-18")]
define_common_component!(XComponentTexture, XComponentTexture);
#[cfg(feature = "api-15")]
define_common_component!(CheckboxGroup, CheckboxGroup);
define_common_component!(Stack, Stack);
define_common_component!(Swiper, Swiper);
define_common_component!(Scroll, Scroll);
define_common_component!(ListItemGroup, ListItemGroup);
define_common_component!(List, List);
define_common_component!(ListItem, ListItem);
define_common_component!(Column, Column);
define_common_component!(Row, Row);
define_common_component!(Flex, Flex);
define_common_component!(Refresh, Refresh);
define_common_component!(WaterFlow, WaterFlow);
define_common_component!(FlowItem, FlowItem);
define_common_component!(RelativeContainer, RelativeContainer);
define_common_component!(Grid, Grid);
define_common_component!(GridItem, GridItem);
define_common_component!(CustomSpan, CustomSpan);
#[cfg(feature = "api-20")]
define_common_component!(EmbeddedComponent, EmbeddedComponent);
#[cfg(feature = "api-20")]
define_common_component!(Undefined, Undefined);
