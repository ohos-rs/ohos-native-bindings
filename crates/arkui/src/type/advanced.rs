//! Module type::advanced wrappers and related types.

use ohos_arkui_sys::*;

use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ListItemSwipeActionDirection,
    "ArkUI_ListItemSwipeActionDirection_ARKUI_LIST_ITEM_SWIPE_ACTION_DIRECTION_"
)]
#[cfg(feature = "api-21")]
/// Swipe direction used by list-item swipe actions.
pub enum ListItemSwipeActionDirection {
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_GestureRecognizerState,
    "ArkUI_GestureRecognizerState_ARKUI_GESTURE_RECOGNIZER_STATE_"
)]
/// State of a gesture recognizer lifecycle.
pub enum GestureRecognizerState {
    Ready,
    Detecting,
    Pending,
    Blocked,
    Successful,
    Failed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_TextSpanType, "ArkUI_TextSpanType_ARKUI_TEXT_SPAN_TYPE_")]
#[cfg(feature = "api-22")]
/// Span type contained in rich text.
pub enum TextSpanType {
    Text,
    Image,
    Mixed,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_TextResponseType,
    "ArkUI_TextResponseType_ARKUI_TEXT_RESPONSE_TYPE_"
)]
#[cfg(feature = "api-22")]
/// Trigger type that produced a text response.
pub enum TextResponseType {
    RightClick,
    LongPress,
    Select,
    Default,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_NavDestinationState,
    "ArkUI_NavDestinationState_ARKUI_NAV_DESTINATION_STATE_"
)]
/// Lifecycle states for navigation destinations.
pub enum NavDestinationState {
    OnShow,
    OnHide,
    OnAppear,
    OnDisappear,
    OnWillShow,
    OnWillHide,
    OnWillAppear,
    OnWillDisappear,
    OnBackPress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_RouterPageState,
    "ArkUI_RouterPageState_ARKUI_ROUTER_PAGE_STATE_"
)]
/// Lifecycle states for router pages.
pub enum RouterPageState {
    AboutToAppear,
    AboutToDisappear,
    OnShow,
    OnHide,
    OnBackPress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_HorizontalAlignment,
    "ArkUI_HorizontalAlignment_ARKUI_HORIZONTAL_ALIGNMENT_"
)]
/// Horizontal alignment values for layout options.
pub enum HorizontalAlignment {
    Start,
    Center,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_VerticalAlignment,
    "ArkUI_VerticalAlignment_ARKUI_VERTICAL_ALIGNMENT_"
)]
/// Vertical alignment values for layout options.
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_Axis, "ArkUI_Axis_ARKUI_AXIS_")]
/// Direction axis used by scrolling and layout APIs.
pub enum Axis {
    Vertical,
    Horizontal,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_BarrierDirection,
    "ArkUI_BarrierDirection_ARKUI_BARRIER_DIRECTION_"
)]
/// Direction used by relative-layout barriers.
pub enum BarrierDirection {
    Start,
    End,
    Top,
    Bottom,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_SwiperIndicatorType,
    "ArkUI_SwiperIndicatorType_ARKUI_SWIPER_INDICATOR_TYPE_"
)]
/// Indicator style used by swiper components.
pub enum SwiperIndicatorType {
    Dot,
    Digit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_FontWeight, "ArkUI_FontWeight_ARKUI_FONT_WEIGHT_")]
/// Font-weight values mapped to ArkUI constants.
pub enum FontWeight {
    #[suffix("W100")]
    W100,
    #[suffix("W200")]
    W200,
    #[suffix("W300")]
    W300,
    #[suffix("W400")]
    W400,
    #[suffix("W500")]
    W500,
    #[suffix("W600")]
    W600,
    #[suffix("W700")]
    W700,
    #[suffix("W800")]
    W800,
    #[suffix("W900")]
    W900,
    Bold,
    Normal,
    Bolder,
    Lighter,
    Medium,
    Regular,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ListItemSwipeEdgeEffect,
    "ArkUI_ListItemSwipeEdgeEffect_ARKUI_LIST_ITEM_SWIPE_EDGE_EFFECT_"
)]
/// Edge behavior when list-item swipe reaches bounds.
pub enum ListItemSwipeEdgeEffect {
    Spring,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_ListItemSwipeActionState,
    "ArkUI_ListItemSwipeActionState_ARKUI_LIST_ITEM_SWIPE_ACTION_STATE_"
)]
/// State of list-item swipe action animation.
pub enum ListItemSwipeActionState {
    Collapsed,
    Expanded,
    Actioning,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_NodeAdapterEventType,
    "ArkUI_NodeAdapterEventType_NODE_ADAPTER_EVENT_"
)]
/// Event types emitted by node-adapter callbacks.
pub enum NodeAdapterEventType {
    WillAttachToNode,
    WillDetachFromNode,
    OnGetNodeId,
    OnAddNodeToAdapter,
    OnRemoveNodeFromAdapter,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_PixelRoundCalcPolicy,
    "ArkUI_PixelRoundCalcPolicy_ARKUI_PIXELROUNDCALCPOLICY_"
)]
#[cfg(feature = "api-21")]
/// Policy controlling pixel-rounding calculations.
pub enum PixelRoundCalcPolicy {
    Noforceround,
    Forceceil,
    Forcefloor,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_DialogState, "ArkUI_DialogState_DIALOG_")]
#[cfg(feature = "api-19")]
/// Runtime state of native/custom dialogs.
pub enum DialogState {
    Uninitialized,
    Initialized,
    Appearing,
    Appeared,
    Disappearing,
    Disappeared,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_NodeContentEventType,
    "ArkUI_NodeContentEventType_NODE_CONTENT_EVENT_"
)]
/// Event types emitted by `NodeContent`.
pub enum NodeContentEventType {
    OnAttachToWindow,
    OnDetachFromWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AnimationFillMode,
    "ArkUI_AnimationFillMode_ARKUI_ANIMATION_FILL_MODE_"
)]
/// Fill behavior outside keyframe/animation active range.
pub enum AnimationFillMode {
    None,
    Forwards,
    Backwards,
    Both,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AnimationDirection,
    "ArkUI_AnimationDirection_ARKUI_ANIMATION_DIRECTION_"
)]
/// Direction behavior for keyframe animations.
pub enum AnimationDirection {
    Normal,
    Reverse,
    Alternate,
    AlternateReverse,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_NodeDirtyFlag, "ArkUI_NodeDirtyFlag_NODE_")]
/// Dirty flags describing what changed in a node.
pub enum NodeDirtyFlag {
    NeedMeasure,
    NeedLayout,
    NeedRender,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_CoastingAxisEventPhase,
    "ArkUI_CoastingAxisEventPhase_ARKUI_COASTING_AXIS_EVENT_PHASE_"
)]
#[cfg(feature = "api-22")]
/// Phase of coasting-axis events.
pub enum CoastingAxisEventPhase {
    None,
    Begin,
    Update,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeyEventType, "ArkUI_KeyEventType_ARKUI_KEY_EVENT_")]
#[cfg(feature = "api-14")]
/// Key event action type.
pub enum KeyEventType {
    Unknown,
    Down,
    Up,
    LongPress,
    Click,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeySourceType, "ArkUI_KeySourceType_ARKUI_KEY_SOURCE_")]
#[cfg(feature = "api-14")]
/// Input source that generated a key event.
pub enum KeySourceType {
    Unknown,
    TypeMouse,
    TypeKeyboard,
    TypeJoystick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_KeyIntension, "ArkUI_KeyIntension_ARKUI_KEY_INTENSION_")]
#[cfg(feature = "api-14")]
/// Key intention semantic mapped by ArkUI.
pub enum KeyIntension {
    Unknown,
    Up,
    Down,
    Left,
    Right,
    Select,
    Escape,
    Back,
    Forward,
    Menu,
    Home,
    PageUp,
    PageDown,
    ZoomOut,
    ZoomIn,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaPlayPause,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaFastForward,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaFastPlayback,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaNext,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaPrevious,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    MediaMute,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    VolumeUp,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    VolumeDown,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    Call,
    #[prefix("ArkUI_KeyIntension_ARKUI_KEY_INTENTION_")]
    Camera,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_TransitionEdge, "ArkUI_TransitionEdge_ARKUI_TRANSITION_EDGE_")]
/// Edge used by movement transition effects.
pub enum TransitionEdge {
    Top,
    Bottom,
    Start,
    End,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_NodeCustomEventType,
    "ArkUI_NodeCustomEventType_ARKUI_NODE_CUSTOM_EVENT_"
)]
/// Event types emitted by custom node callbacks.
pub enum NodeCustomEventType {
    OnMeasure,
    OnLayout,
    OnDraw,
    OnForegroundDraw,
    OnOverlayDraw,
    #[cfg(feature = "api-20")]
    OnDrawFront,
    #[cfg(feature = "api-20")]
    OnDrawBehind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_LengthMetricUnit,
    "ArkUI_LengthMetricUnit_ARKUI_LENGTH_METRIC_UNIT_"
)]
/// Length units supported by metric-based APIs.
pub enum LengthMetricUnit {
    Default,
    Px,
    Vp,
    Fp,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_DragStatus, "ArkUI_DragStatus_ARKUI_DRAG_STATUS_")]
/// Real-time drag status in drag lifecycle events.
pub enum DragStatus {
    Unknown,
    Started,
    Ended,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_DragPreviewScaleMode,
    "ArkUI_DragPreviewScaleMode_ARKUI_DRAG_PREVIEW_SCALE_"
)]
/// Scaling mode for drag preview rendering.
pub enum DragPreviewScaleMode {
    Auto,
    Disabled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_LevelMode, "ArkUI_LevelMode_ARKUI_LEVEL_MODE_")]
#[cfg(feature = "api-19")]
/// Level mode used by ArkUI rendering/effect APIs.
pub enum LevelMode {
    Overlay,
    Embedded,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_ImmersiveMode, "ArkUI_ImmersiveMode_ARKUI_IMMERSIVE_MODE_")]
#[cfg(feature = "api-19")]
/// Immersive mode behavior for system UI overlays.
pub enum ImmersiveMode {
    Default,
    Extend,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_ShadowStyle, "ArkUI_ShadowStyle_ARKUI_SHADOW_STYLE_")]
#[cfg(feature = "api-19")]
/// Predefined shadow style presets.
pub enum ShadowStyle {
    OuterDefaultXs,
    OuterDefaultSm,
    OuterDefaultMd,
    OuterDefaultLg,
    OuterFloatingSm,
    OuterFloatingMd,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(ArkUI_BlurStyle, "ArkUI_BlurStyle_ARKUI_BLUR_STYLE_")]
#[cfg(feature = "api-19")]
/// Blur style presets for background/foreground blur.
pub enum BlurStyle {
    Thin,
    Regular,
    Thick,
    BackgroundThin,
    BackgroundRegular,
    BackgroundThick,
    BackgroundUltraThick,
    None,
    ComponentUltraThin,
    ComponentThin,
    ComponentRegular,
    ComponentThick,
    ComponentUltraThick,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_KeyboardAvoidMode,
    "ArkUI_KeyboardAvoidMode_ARKUI_KEYBOARD_AVOID_MODE_"
)]
#[cfg(feature = "api-19")]
/// Keyboard avoidance behavior for focused content.
pub enum KeyboardAvoidMode {
    Default,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_HoverModeAreaType,
    "ArkUI_HoverModeAreaType_ARKUI_HOVER_MODE_AREA_TYPE_"
)]
#[cfg(feature = "api-19")]
/// Area type used by hover-mode APIs.
pub enum HoverModeAreaType {
    Top,
    Bottom,
}
