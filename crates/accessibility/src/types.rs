use bitflags::bitflags;
use ohos_accessibility_sys::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum ActionType {
    Invalid = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_INVALID,
    Click = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CLICK,
    LongClick = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_LONG_CLICK,
    GainAccessibilityFocus = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_GAIN_ACCESSIBILITY_FOCUS,
    ClearAccessibilityFocus = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CLEAR_ACCESSIBILITY_FOCUS,
    ScrollForward =
        ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SCROLL_FORWARD,
    ScrollBackward =
        ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SCROLL_BACKWARD,
    Copy = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_COPY,
    Paste = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_PASTE,
    Cut = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CUT,
    SelectText = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SELECT_TEXT,
    SetText = ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SET_TEXT,
    SetCursorPosition =
        ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SET_CURSOR_POSITION,
    #[cfg(feature = "api-15")]
    NextHtmlItem =
        ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_NEXT_HTML_ITEM,
    #[cfg(feature = "api-15")]
    PreviousHtmlItem =
        ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_PREVIOUS_HTML_ITEM,
}

impl ActionType {
    #[allow(non_upper_case_globals)]
    pub fn from_raw(raw: ArkUI_Accessibility_ActionType) -> Option<Self> {
        Some(match raw {
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_INVALID => {
                Self::Invalid
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CLICK => {
                Self::Click
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_LONG_CLICK => {
                Self::LongClick
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_GAIN_ACCESSIBILITY_FOCUS => {
                Self::GainAccessibilityFocus
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CLEAR_ACCESSIBILITY_FOCUS => {
                Self::ClearAccessibilityFocus
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SCROLL_FORWARD => {
                Self::ScrollForward
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SCROLL_BACKWARD => {
                Self::ScrollBackward
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_COPY => {
                Self::Copy
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_PASTE => {
                Self::Paste
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_CUT => Self::Cut,
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SELECT_TEXT => {
                Self::SelectText
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SET_TEXT => {
                Self::SetText
            }
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_SET_CURSOR_POSITION => {
                Self::SetCursorPosition
            }
            #[cfg(feature = "api-15")]
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_NEXT_HTML_ITEM => {
                Self::NextHtmlItem
            }
            #[cfg(feature = "api-15")]
            ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_PREVIOUS_HTML_ITEM => {
                Self::PreviousHtmlItem
            }
            _ => return None,
        })
    }

    pub fn as_raw(self) -> ArkUI_Accessibility_ActionType {
        self as _
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum EventType {
    Invalid = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_INVALID,
    Clicked = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_CLICKED,
    LongClicked = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_LONG_CLICKED,
    Selected = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_SELECTED,
    TextUpdate = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_TEXT_UPDATE,
    PageStateUpdate =
        ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_PAGE_STATE_UPDATE,
    PageContentUpdate =
        ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_PAGE_CONTENT_UPDATE,
    Scrolled = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_SCROLLED,
    AccessibilityFocused =
        ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_ACCESSIBILITY_FOCUSED,
    AccessibilityFocusCleared = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_ACCESSIBILITY_FOCUS_CLEARED,
    RequestAccessibilityFocus = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_REQUEST_ACCESSIBILITY_FOCUS,
    PageOpen = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_PAGE_OPEN,
    PageClose = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_PAGE_CLOSE,
    Announce = ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_ANNOUNCE_FOR_ACCESSIBILITY,
    FocusNodeUpdate =
        ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_FOCUS_NODE_UPDATE,
}

impl EventType {
    pub fn as_raw(self) -> ArkUI_AccessibilityEventType {
        self as _
    }
}

bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct SearchMode: u32 {
        const CURRENT = ArkUI_AccessibilitySearchMode_ARKUI_ACCESSIBILITY_NATIVE_SEARCH_MODE_PREFETCH_CURRENT;
        const PREDECESSORS = ArkUI_AccessibilitySearchMode_ARKUI_ACCESSIBILITY_NATIVE_SEARCH_MODE_PREFETCH_PREDECESSORS;
        const SIBLINGS = ArkUI_AccessibilitySearchMode_ARKUI_ACCESSIBILITY_NATIVE_SEARCH_MODE_PREFETCH_SIBLINGS;
        const CHILDREN = ArkUI_AccessibilitySearchMode_ARKUI_ACCESSIBILITY_NATIVE_SEARCH_MODE_PREFETCH_CHILDREN;
        const RECURSIVE_CHILDREN = ArkUI_AccessibilitySearchMode_ARKUI_ACCESSIBILITY_NATIVE_SEARCH_MODE_PREFETCH_RECURSIVE_CHILDREN;
    }
}

impl SearchMode {
    pub fn from_raw(raw: ArkUI_AccessibilitySearchMode) -> Self {
        Self::from_bits_retain(raw)
    }

    pub fn as_raw(self) -> ArkUI_AccessibilitySearchMode {
        self.bits()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum FocusType {
    Invalid = ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_INVALID,
    Input = ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_INPUT,
    Accessibility =
        ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_ACCESSIBILITY,
}

impl FocusType {
    #[allow(non_upper_case_globals)]
    pub fn from_raw(raw: ArkUI_AccessibilityFocusType) -> Option<Self> {
        match raw {
            ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_INVALID => {
                Some(Self::Invalid)
            }
            ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_INPUT => {
                Some(Self::Input)
            }
            ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_ACCESSIBILITY => {
                Some(Self::Accessibility)
            }
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum FocusMoveDirection {
    Invalid = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_INVALID,
    Up = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_UP,
    Down = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_DOWN,
    Left = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_LEFT,
    Right = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_RIGHT,
    Forward = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_FORWARD,
    Backward = ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_BACKWARD,
}

impl FocusMoveDirection {
    #[allow(non_upper_case_globals)]
    pub fn from_raw(raw: ArkUI_AccessibilityFocusMoveDirection) -> Option<Self> {
        Some(match raw {
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_INVALID => {
                Self::Invalid
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_UP => {
                Self::Up
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_DOWN => {
                Self::Down
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_LEFT => {
                Self::Left
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_RIGHT => {
                Self::Right
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_FORWARD => {
                Self::Forward
            }
            ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_BACKWARD => {
                Self::Backward
            }
            _ => return None,
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct AccessibleRect {
    pub left: i32,
    pub top: i32,
    pub right: i32,
    pub bottom: i32,
}

impl From<AccessibleRect> for ArkUI_AccessibleRect {
    fn from(value: AccessibleRect) -> Self {
        Self {
            leftTopX: value.left,
            leftTopY: value.top,
            rightBottomX: value.right,
            rightBottomY: value.bottom,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct RangeInfo {
    pub min: f64,
    pub max: f64,
    pub current: f64,
}

impl From<RangeInfo> for ArkUI_AccessibleRangeInfo {
    fn from(value: RangeInfo) -> Self {
        Self {
            min: value.min,
            max: value.max,
            current: value.current,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GridInfo {
    pub rows: i32,
    pub columns: i32,
    pub selection_mode: i32,
}

impl From<GridInfo> for ArkUI_AccessibleGridInfo {
    fn from(value: GridInfo) -> Self {
        Self {
            rowCount: value.rows,
            columnCount: value.columns,
            selectionMode: value.selection_mode,
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct GridItemInfo {
    pub heading: bool,
    pub selected: bool,
    pub column_index: i32,
    pub row_index: i32,
    pub column_span: i32,
    pub row_span: i32,
}

impl From<GridItemInfo> for ArkUI_AccessibleGridItemInfo {
    fn from(value: GridItemInfo) -> Self {
        Self {
            heading: value.heading,
            selected: value.selected,
            columnIndex: value.column_index,
            rowIndex: value.row_index,
            columnSpan: value.column_span,
            rowSpan: value.row_span,
        }
    }
}
