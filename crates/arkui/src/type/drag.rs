use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_PreDragStatus, "ArkUI_PreDragStatus_ARKUI_PRE_DRAG_STATUS_")]
pub enum PreDragStatus {
    Unknown,
    ActionDetecting,
    ReadyToTriggerDrag,
    PreviewLiftStarted,
    PreviewLiftFinished,
    PreviewLandingStarted,
    PreviewLandingFinished,
    CanceledBeforeDrag,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_DragResult, "ArkUI_DragResult_ARKUI_DRAG_RESULT_")]
pub enum DragResult {
    Successful,
    Failed,
    Canceled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_DropOperation, "ArkUI_DropOperation_ARKUI_DROP_OPERATION_")]
pub enum DropOperation {
    Copy,
    Move,
}
