use bitflags::bitflags;
use ohos_accessibility_sys::*;
use ohos_enum_derive::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_Accessibility_ActionType,
    "ArkUI_Accessibility_ActionType_ARKUI_ACCESSIBILITY_NATIVE_ACTION_TYPE_"
)]
pub enum ActionType {
    Invalid,
    Click,
    LongClick,
    GainAccessibilityFocus,
    ClearAccessibilityFocus,
    ScrollForward,
    ScrollBackward,
    Copy,
    Paste,
    Cut,
    SelectText,
    SetText,
    SetCursorPosition,
    #[cfg(feature = "api-15")]
    NextHtmlItem,
    #[cfg(feature = "api-15")]
    PreviousHtmlItem,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AccessibilityEventType,
    "ArkUI_AccessibilityEventType_ARKUI_ACCESSIBILITY_NATIVE_EVENT_TYPE_"
)]
pub enum EventType {
    Invalid,
    Clicked,
    LongClicked,
    Selected,
    TextUpdate,
    PageStateUpdate,
    PageContentUpdate,
    Scrolled,
    AccessibilityFocused,
    AccessibilityFocusCleared,
    RequestAccessibilityFocus,
    PageOpen,
    PageClose,
    #[suffix("ANNOUNCE_FOR_ACCESSIBILITY")]
    Announce,
    FocusNodeUpdate,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AccessibilityFocusType,
    "ArkUI_AccessibilityFocusType_ARKUI_ACCESSIBILITY_NATIVE_FOCUS_TYPE_"
)]
pub enum FocusType {
    Invalid,
    Input,
    Accessibility,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(
    ArkUI_AccessibilityFocusMoveDirection,
    "ArkUI_AccessibilityFocusMoveDirection_ARKUI_ACCESSIBILITY_NATIVE_DIRECTION_"
)]
pub enum FocusMoveDirection {
    Invalid,
    Up,
    Down,
    Left,
    Right,
    Forward,
    Backward,
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
