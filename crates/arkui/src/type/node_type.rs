use ohos_arkui_sys::*;
use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_NodeType, "ArkUI_NodeType_ARKUI_NODE_")]
pub enum ArkUINodeType {
    Custom,
    Text,
    Span,
    ImageSpan,
    Image,
    Toggle,
    LoadingProgress,
    TextInput,
    TextArea,
    Button,
    Progress,
    Checkbox,
    XComponent,
    DatePicker,
    TimePicker,
    TextPicker,
    CalendarPicker,
    Slider,
    Radio,
    ImageAnimator,
    #[cfg(feature = "api-18")]
    XComponentTexture,
    #[cfg(feature = "api-15")]
    CheckboxGroup,
    Stack,
    Swiper,
    Scroll,
    List,
    ListItem,
    ListItemGroup,
    Column,
    Row,
    Flex,
    Refresh,
    WaterFlow,
    FlowItem,
    RelativeContainer,
    Grid,
    GridItem,
    CustomSpan,
    #[cfg(feature = "api-20")]
    EmbeddedComponent,
    #[cfg(feature = "api-20")]
    Undefined,
}
