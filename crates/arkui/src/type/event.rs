use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_NodeEventType, "ArkUI_NodeEventType_NODE_")]
pub enum NodeEventType {
    TouchEvent,
    EventOnAppear,
    EventOnDisappear,
    EventOnAreaChange,
    OnFocus,
    OnBlur,
    OnClick,
    OnTouchIntercept,
    EventOnVisibleAreaChange,
    OnHover,
    OnMouse,
    EventOnAttach,
    EventOnDetach,
    OnAccessibilityActions,
    OnPreDrag,
    OnDragStart,
    OnDragEnter,
    OnDragMove,
    OnDragLeave,
    OnDrop,
    OnDragEnd,
    #[cfg(feature = "api-14")]
    OnKeyEvent,
    #[cfg(feature = "api-14")]
    OnKeyPreIme,
    #[cfg(feature = "api-15")]
    OnFocusAxis,
    #[cfg(feature = "api-15")]
    DispatchKeyEvent,
    #[cfg(feature = "api-17")]
    OnAxis,
    #[cfg(feature = "api-18")]
    OnClickEvent,
    #[cfg(feature = "api-17")]
    OnHoverEvent,
    #[cfg(feature = "api-17")]
    VisibleAreaApproximateChangeEvent,
    #[cfg(feature = "api-15")]
    OnHoverMove,
    #[cfg(feature = "api-21")]
    OnSizeChange,
    #[cfg(feature = "api-22")]
    OnCoastingAxisEvent,
    #[cfg(feature = "api-22")]
    OnChildTouchTest,
    TextOnDetectResultUpdate,
    #[cfg(feature = "api-20")]
    TextSpanOnLongPress,
    ImageOnComplete,
    ImageOnError,
    ImageOnSvgPlayFinish,
    ImageOnDownloadProgress,
    ToggleOnChange,
    TextInputOnChange,
    TextInputOnSubmit,
    TextInputOnCut,
    TextInputOnPaste,
    TextInputOnTextSelectionChange,
    TextInputOnEditChange,
    TextInputOnContentSizeChange,
    TextInputOnInputFilterError,
    TextInputOnContentScroll,
    TextInputOnWillInsert,
    TextInputOnDidInsert,
    TextInputOnWillDelete,
    TextInputOnDidDelete,
    #[cfg(feature = "api-15")]
    TextInputOnChangeWithPreviewText,
    #[cfg(feature = "api-20")]
    TextInputOnWillChange,
    TextAreaOnChange,
    TextAreaOnPaste,
    TextAreaOnTextSelectionChange,
    TextAreaOnInputFilterError,
    TextAreaOnContentScroll,
    TextAreaOnEditChange,
    TextAreaOnSubmit,
    TextAreaOnContentSizeChange,
    TextAreaOnWillInsert,
    TextAreaOnDidInsert,
    TextAreaOnWillDelete,
    TextAreaOnDidDelete,
    #[cfg(feature = "api-15")]
    TextAreaOnChangeWithPreviewText,
    #[cfg(feature = "api-20")]
    TextAreaOnWillChange,
    CheckboxEventOnChange,
    DatePickerEventOnDateChange,
    TimePickerEventOnChange,
    TextPickerEventOnChange,
    #[cfg(feature = "api-14")]
    TextPickerEventOnScrollStop,
    CalendarPickerEventOnChange,
    SliderEventOnChange,
    RadioEventOnChange,
    ImageAnimatorEventOnStart,
    ImageAnimatorEventOnPause,
    ImageAnimatorEventOnRepeat,
    ImageAnimatorEventOnCancel,
    ImageAnimatorEventOnFinish,
    #[cfg(feature = "api-15")]
    CheckboxGroupEventOnChange,
    SwiperEventOnChange,
    SwiperEventOnAnimationStart,
    SwiperEventOnAnimationEnd,
    SwiperEventOnGestureSwipe,
    SwiperEventOnContentDidScroll,
    #[cfg(feature = "api-15")]
    SwiperEventOnContentWillScroll,
    #[cfg(feature = "api-18")]
    SwiperEventOnSelected,
    #[cfg(feature = "api-18")]
    SwiperEventOnUnselected,
    #[cfg(feature = "api-20")]
    SwiperEventOnScrollStateChanged,
    ScrollEventOnScroll,
    ScrollEventOnScrollFrameBegin,
    ScrollEventOnWillScroll,
    ScrollEventOnDidScroll,
    ScrollEventOnScrollStart,
    ScrollEventOnScrollStop,
    ScrollEventOnScrollEdge,
    ScrollEventOnReachStart,
    ScrollEventOnReachEnd,
    #[cfg(feature = "api-20")]
    ScrollEventOnWillStopDragging,
    #[cfg(feature = "api-20")]
    ScrollEventOnDidZoom,
    #[cfg(feature = "api-20")]
    ScrollEventOnZoomStart,
    #[cfg(feature = "api-20")]
    ScrollEventOnZoomStop,
    #[cfg(feature = "api-21")]
    ScrollEventOnWillStartDragging,
    #[cfg(feature = "api-21")]
    ScrollEventOnDidStopDragging,
    #[cfg(feature = "api-21")]
    ScrollEventOnWillStartFling,
    #[cfg(feature = "api-21")]
    ScrollEventOnDidStopFling,
    ListOnScrollIndex,
    ListOnWillScroll,
    ListOnDidScroll,
    #[cfg(feature = "api-15")]
    ListOnScrollVisibleContentChange,
    RefreshStateChange,
    RefreshOnRefresh,
    RefreshOnOffsetChange,
    OnWillScroll,
    WaterFlowOnDidScroll,
    WaterFlowOnScrollIndex,
    #[cfg(feature = "api-22")]
    GridOnScrollIndex,
    #[cfg(feature = "api-22")]
    GridOnWillScroll,
    #[cfg(feature = "api-22")]
    GridOnDidScroll,
    #[cfg(feature = "api-22")]
    GridOnScrollBarUpdate,
}

#[cfg(feature = "api-22")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(
    ArkUI_TouchTestStrategy,
    "ArkUI_TouchTestStrategy_ARKUI_TOUCH_TEST_STRATEGY_"
)]
pub enum TouchTestStrategy {
    Default,
    ForwardCompetition,
    Forward,
}
