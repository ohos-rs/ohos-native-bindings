use crate::{
    ArkUIAttributeBasic, ArkUICommonFontAttribute, ArkUIEvent, ArkUIGesture, ArkUINode,
    ArkUINodeAttributeItem, ArkUINodeAttributeType, ArkUINodeType, ArkUIResult,
    ARK_UI_NATIVE_NODE_API_1
};

#[cfg(feature = "xcomponent")]
use ohos_xcomponent_binding::NativeXComponent as XC;
#[cfg(feature = "xcomponent")]
use ohos_xcomponent_sys::OH_NativeXComponent_GetNativeXComponent;

use crate::component::ArkUICommonAttribute;

#[derive(Clone)]
pub struct XComponent(ArkUINode);

impl XComponent {
    pub fn new() -> ArkUIResult<Self> {
        let xcomponent = ARK_UI_NATIVE_NODE_API_1.create_node(ArkUINodeType::XComponent)?;
        Ok(Self(ArkUINode {
            raw: xcomponent,
            tag: ArkUINodeType::XComponent,
            ..Default::default()
        }))
    }

    #[cfg(feature = "xcomponent")]
    pub fn native_xcomponent(&self) -> XC {
        use ohos_xcomponent_binding::XComponentRaw;

        let handle = unsafe { OH_NativeXComponent_GetNativeXComponent(self.0.raw) };
        let id = ARK_UI_NATIVE_NODE_API_1
            .get_attribute(&self.0, ArkUINodeAttributeType::XComponentId.into())
            .ok()
            .and_then(|attr| {
                if let ArkUINodeAttributeItem::String(xcomponent_id) = attr {
                    Some(xcomponent_id)
                } else {
                    None
                }
            });
        match id {
            Some(id) => XC::with_id(XComponentRaw(handle), id),
            None => XC::new(XComponentRaw(handle)),
        }
    }
}

impl From<XComponent> for ArkUINode {
    fn from(xcomponent: XComponent) -> Self {
        xcomponent.0
    }
}

impl ArkUIAttributeBasic for XComponent {
    fn raw(&self) -> &ArkUINode {
        &self.0
    }

    fn borrow_mut(&mut self) -> &mut ArkUINode {
        &mut self.0
    }
}

impl ArkUICommonAttribute for XComponent {}
impl ArkUICommonFontAttribute for XComponent {}
impl ArkUIEvent for XComponent {}
impl ArkUIGesture for XComponent {}
