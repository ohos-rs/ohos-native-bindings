use ohos_arkui_input_sys::*;
use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone)]
pub enum ArkUIInputError {
    InternalError(i32),
    DeviceTypeNotSupported(String, u32),
}

impl std::fmt::Display for ArkUIInputError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArkUIInputError::InternalError(err) => write!(f, "Internal error: {}", err),
            ArkUIInputError::DeviceTypeNotSupported(api, device_type) => {
                write!(
                    f,
                    "You'r trying to use {} api, current device type not supported: {}",
                    api, device_type
                )
            }
        }
    }
}

impl std::error::Error for ArkUIInputError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_ErrorCode, "ArkUI_ErrorCode_ARKUI_ERROR_CODE_")]
pub enum ArkUIErrorCode {
    ParamInvalid,
    #[cfg(feature = "api-18")]
    #[suffix("CAPI_INIT_ERROR")]
    CapiInitError,
    #[cfg(feature = "api-15")]
    InternalError,
    #[cfg(feature = "api-21")]
    ParamError,
    #[cfg(feature = "api-18")]
    #[suffix("XCOMPONENT_STATE_INVALID")]
    XComponentStateInvalid,
    AttributeOrEventNotSupported,
    #[suffix("ARKTS_NODE_NOT_SUPPORTED")]
    ArkTSNodeNotSupported,
    AdapterNotBound,
    AdapterExist,
    ChildNodeExist,
    NodeEventParamIndexOutOfRange,
    NodeEventParamInvalid,
    NodeEventNoReturn,
    #[cfg(feature = "api-21")]
    NodeUnsupportedEventType,
    NodeIndexInvalid,
    GetInfoFailed,
    BufferSizeError,
    #[cfg(feature = "api-15")]
    NodeNotOnMainTree,
    #[cfg(feature = "api-22")]
    NodeOnInvalidThread,
    #[cfg(feature = "api-20")]
    ForceDarkConfigInvalid,
    #[cfg(feature = "api-22")]
    NodeIsAdopted,
    #[cfg(feature = "api-22")]
    NodeHasParent,
    #[cfg(feature = "api-22")]
    NodeCanNotBeAdopted,
    #[cfg(feature = "api-22")]
    NodeCanNotAdoptTo,
    #[cfg(feature = "api-22")]
    NodeIsNotInAdoptedChildren,
    #[cfg(feature = "api-20")]
    NotCustomNode,
    #[cfg(feature = "api-20")]
    ChildExisted,
    #[cfg(feature = "api-20")]
    RenderParentExisted,
    #[cfg(feature = "api-20")]
    RenderChildNotExist,
    #[cfg(feature = "api-20")]
    ParamOutOfRange,
    #[cfg(feature = "api-22")]
    RenderIsFromFrameNode,
    #[cfg(feature = "api-22")]
    RenderHasInvalidFrameNode,
    #[cfg(feature = "api-22")]
    RenderNotAdoptedNode,
    #[cfg(feature = "api-15")]
    FocusNonFocusable,
    #[cfg(feature = "api-15")]
    FocusNonFocusableAncestor,
    #[cfg(feature = "api-15")]
    FocusNonExistent,
    #[cfg(feature = "api-15")]
    ComponentSnapshotTimeout,
    #[cfg(feature = "api-23")]
    ComponentSnapshotModeNotSupported,
    #[cfg(feature = "api-23")]
    ComponentSnapshotAutoNotSupported,
    NonScrollableContainer,
    BufferSizeNotEnough,
    #[cfg(feature = "api-15")]
    NotClonedPointerEvent,
    #[cfg(feature = "api-15")]
    PostClonedComponentStatusAbnormal,
    #[cfg(feature = "api-15")]
    PostClonedNoComponentHitToRespondToTheEvent,
    #[cfg(feature = "api-20")]
    #[prefix("ArkUI_ErrorCode_ARKUI_ERROR_")]
    InputEventTypeNotSupported,
    #[cfg(feature = "api-14")]
    InvalidStyledString,
    #[cfg(feature = "api-18")]
    RecognizerTypeNotSupported,
    #[cfg(feature = "api-18")]
    #[suffix("UI_CONTEXT_INVALID")]
    UIContextInvalid,
    #[cfg(feature = "api-18")]
    CallbackInvalid,
    #[cfg(feature = "api-19")]
    DragDropOperationNotAllowed,
    #[fallback]
    Unknown(ArkUI_ErrorCode),
}

impl ArkUIErrorCode {
    pub fn code(self) -> ArkUI_ErrorCode {
        self.into()
    }
}

impl AsRef<str> for ArkUIErrorCode {
    fn as_ref(&self) -> &str {
        match self {
            ArkUIErrorCode::ParamInvalid => "ParamInvalid",
            #[cfg(feature = "api-18")]
            ArkUIErrorCode::CapiInitError => "CapiInitError",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::InternalError => "InternalError",
            #[cfg(feature = "api-21")]
            ArkUIErrorCode::ParamError => "ParamError",
            #[cfg(feature = "api-18")]
            ArkUIErrorCode::XComponentStateInvalid => "XComponentStateInvalid",
            ArkUIErrorCode::AttributeOrEventNotSupported => "AttributeOrEventNotSupported",
            ArkUIErrorCode::ArkTSNodeNotSupported => "ArkTSNodeNotSupported",
            ArkUIErrorCode::AdapterNotBound => "AdapterNotBound",
            ArkUIErrorCode::AdapterExist => "AdapterExist",
            ArkUIErrorCode::ChildNodeExist => "ChildNodeExist",
            ArkUIErrorCode::NodeEventParamIndexOutOfRange => "NodeEventParamIndexOutOfRange",
            ArkUIErrorCode::NodeEventParamInvalid => "NodeEventParamInvalid",
            ArkUIErrorCode::NodeEventNoReturn => "NodeEventNoReturn",
            #[cfg(feature = "api-21")]
            ArkUIErrorCode::NodeUnsupportedEventType => "NodeUnsupportedEventType",
            ArkUIErrorCode::NodeIndexInvalid => "NodeIndexInvalid",
            ArkUIErrorCode::GetInfoFailed => "GetInfoFailed",
            ArkUIErrorCode::BufferSizeError => "BufferSizeError",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::NodeNotOnMainTree => "NodeNotOnMainTree",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeOnInvalidThread => "NodeOnInvalidThread",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::ForceDarkConfigInvalid => "ForceDarkConfigInvalid",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeIsAdopted => "NodeIsAdopted",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeHasParent => "NodeHasParent",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeCanNotBeAdopted => "NodeCanNotBeAdopted",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeCanNotAdoptTo => "NodeCanNotAdoptTo",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::NodeIsNotInAdoptedChildren => "NodeIsNotInAdoptedChildren",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::NotCustomNode => "NotCustomNode",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::ChildExisted => "ChildExisted",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::RenderParentExisted => "RenderParentExisted",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::RenderChildNotExist => "RenderChildNotExist",
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::ParamOutOfRange => "ParamOutOfRange",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::RenderIsFromFrameNode => "RenderIsFromFrameNode",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::RenderHasInvalidFrameNode => "RenderHasInvalidFrameNode",
            #[cfg(feature = "api-22")]
            ArkUIErrorCode::RenderNotAdoptedNode => "RenderNotAdoptedNode",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::FocusNonFocusable => "FocusNonFocusable",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::FocusNonFocusableAncestor => "FocusNonFocusableAncestor",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::FocusNonExistent => "FocusNonExistent",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::ComponentSnapshotTimeout => "ComponentSnapshotTimeout",
            #[cfg(feature = "api-23")]
            ArkUIErrorCode::ComponentSnapshotModeNotSupported => {
                "ComponentSnapshotModeNotSupported"
            }
            #[cfg(feature = "api-23")]
            ArkUIErrorCode::ComponentSnapshotAutoNotSupported => {
                "ComponentSnapshotAutoNotSupported"
            }
            ArkUIErrorCode::NonScrollableContainer => "NonScrollableContainer",
            ArkUIErrorCode::BufferSizeNotEnough => "BufferSizeNotEnough",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::NotClonedPointerEvent => "NotClonedPointerEvent",
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::PostClonedComponentStatusAbnormal => {
                "PostClonedComponentStatusAbnormal"
            }
            #[cfg(feature = "api-15")]
            ArkUIErrorCode::PostClonedNoComponentHitToRespondToTheEvent => {
                "PostClonedNoComponentHitToRespondToTheEvent"
            }
            #[cfg(feature = "api-20")]
            ArkUIErrorCode::InputEventTypeNotSupported => "InputEventTypeNotSupported",
            #[cfg(feature = "api-14")]
            ArkUIErrorCode::InvalidStyledString => "InvalidStyledString",
            #[cfg(feature = "api-18")]
            ArkUIErrorCode::RecognizerTypeNotSupported => "RecognizerTypeNotSupported",
            #[cfg(feature = "api-18")]
            ArkUIErrorCode::UIContextInvalid => "UIContextInvalid",
            #[cfg(feature = "api-18")]
            ArkUIErrorCode::CallbackInvalid => "CallbackInvalid",
            #[cfg(feature = "api-19")]
            ArkUIErrorCode::DragDropOperationNotAllowed => "DragDropOperationNotAllowed",
            ArkUIErrorCode::Unknown(_) => "Unknown",
        }
    }
}

impl std::fmt::Display for ArkUIErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ArkUIErrorCode::Unknown(raw) => write!(f, "Unknown({raw})"),
            known => f.write_str(known.as_ref()),
        }
    }
}

impl std::error::Error for ArkUIErrorCode {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_known_and_unknown_arkui_error_codes_without_panicking() {
        assert_eq!(
            ArkUIErrorCode::from(ArkUI_ErrorCode_ARKUI_ERROR_CODE_PARAM_INVALID),
            ArkUIErrorCode::ParamInvalid
        );
        assert_eq!(
            ArkUIErrorCode::from(ArkUI_ErrorCode_ARKUI_ERROR_CODE_GET_INFO_FAILED),
            ArkUIErrorCode::GetInfoFailed
        );
        let unknown = ArkUIErrorCode::from(999_999);
        assert_eq!(unknown, ArkUIErrorCode::Unknown(999_999));
        assert_eq!(unknown.code(), 999_999);
        assert_eq!(unknown.as_ref(), "Unknown");
        assert_eq!(ArkUIErrorCode::try_from_raw(999_999), None);
    }
}
