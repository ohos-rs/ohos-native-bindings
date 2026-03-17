use ohos_arkui_sys::*;

use ohos_enum_macro::EnumFrom;

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[enum_from_config(ArkUI_NodeAttributeType, "ArkUI_NodeAttributeType_NODE_")]
pub enum ArkUINodeAttributeType {
    Width,
    Height,
    BackgroundColor,
    BackgroundImage,
    Padding,
    Id,
    Enabled,
    Margin,
    Translate,
    Scale,
    Rotate,
    Brightness,
    Saturation,
    Blur,
    LinearGradient,
    Alignment,
    Opacity,
    BorderWidth,
    BorderRadius,
    BorderColor,
    BorderStyle,
    ZIndex,
    Visibility,
    Clip,
    ClipShape,
    Transform,
    HitTestBehavior,
    Position,
    Shadow,
    CustomShadow,
    BackgroundImageSize,
    BackgroundImageSizeWithStyle,
    BackgroundBlurStyle,
    TransformCenter,
    OpacityTransition,
    RotateTransition,
    ScaleTransition,
    TranslateTransition,
    MoveTransition,
    Focusable,
    DefaultFocus,
    ResponseRegion,
    Overlay,
    SweepGradient,
    RadialGradient,
    Mask,
    BlendMode,
    Direction,
    ConstraintSize,
    GrayScale,
    Invert,
    Sepia,
    Contrast,
    ForegroundColor,
    Offset,
    MarkAnchor,
    BackgroundImagePosition,
    AlignRules,
    AlignSelf,
    FlexGrow,
    FlexShrink,
    FlexBasis,
    AccessibilityGroup,
    AccessibilityText,
    AccessibilityMode,
    AccessibilityDescription,
    FocusStatus,
    AspectRatio,
    LayoutWeight,
    DisplayPriority,
    OutlineWidth,
    WidthPercent,
    HeightPercent,
    PaddingPercent,
    MarginPercent,
    GeometryTransition,
    RelativeLayoutChainMode,
    RenderFit,
    OutlineColor,
    Size,
    RenderGroup,
    ColorBlend,
    ForegroundBlurStyle,
    LayoutRect,
    FocusOnTouch,
    BorderWidthPercent,
    BorderRadiusPercent,
    AccessibilityId,
    AccessibilityActions,
    AccessibilityRole,
    AccessibilityState,
    AccessibilityValue,
    ExpandSafeArea,
    VisibleAreaChangeRatio,
    Transition,
    UniqueId,
    FocusBox,
    ClickDistance,
    TextContent,
    FontColor,
    FontSize,
    FontStyle,
    FontWeight,
    TextLineHeight,
    TextDecoration,
    TextCase,
    TextLetterSpacing,
    TextMaxLines,
    TextAlign,
    TextOverflow,
    FontFamily,
    TextCopyOption,
    TextBaselineOffset,
    TextTextShadow,
    TextMinFontSize,
    TextMaxFontSize,
    TextFont,
    TextHeightAdaptivePolicy,
    TextIndent,
    TextWordBreak,
    TextEllipsisMode,
    TextLineSpacing,
    FontFeature,
    TextEnableDataDetector,
    TextEnableDataDetectorConfig,
    TextSelectedBackgroundColor,
    TextContentWithStyledString,
    TextHalfLeading,
    SpanContent,
    SpanTextBackgroundStyle,
    SpanBaselineOffset,
    ImageSpanSrc,
    ImageSpanVerticalAlignment,
    ImageSpanAlt,
    ImageSrc,
    ImageObjectFit,
    ImageInterpolation,
    ImageObjectRepeat,
    ImageColorFilter,
    ImageAutoResize,
    ImageAlt,
    ImageDraggable,
    ImageRenderMode,
    ImageFitOriginalSize,
    ImageFillColor,
    ImageResizable,
    ToggleSelectedColor,
    ToggleSwitchPointColor,
    ToggleValue,
    ToggleUnselectedColor,
    LoadingProgressColor,
    LoadingProgressEnableLoading,
    TextInputPlaceholder,
    TextInputText,
    TextInputCaretColor,
    TextInputCaretStyle,
    TextInputShowUnderline,
    TextInputMaxLength,
    TextInputEnterKeyType,
    TextInputPlaceholderColor,
    TextInputPlaceholderFont,
    TextInputEnableKeyboardOnFocus,
    TextInputType,
    TextInputSelectedBackgroundColor,
    TextInputShowPasswordIcon,
    TextInputEditing,
    TextInputCancelButton,
    TextInputTextSelection,
    TextInputUnderlineColor,
    TextInputEnableAutoFill,
    TextInputContentType,
    TextInputPasswordRules,
    TextInputSelectAll,
    TextInputInputFilter,
    TextInputStyle,
    TextInputCaretOffset,
    TextInputContentRect,
    TextInputContentLineCount,
    TextInputSelectionMenuHidden,
    TextInputBlurOnSubmit,
    TextInputCustomKeyboard,
    TextInputWordBreak,
    TextInputShowKeyboardOnFocus,
    TextInputNumberOfLines,
    TextAreaPlaceholder,
    TextAreaText,
    TextAreaMaxLength,
    TextAreaPlaceholderColor,
    TextAreaPlaceholderFont,
    TextAreaCaretColor,
    TextAreaEditing,
    TextAreaType,
    TextAreaShowCounter,
    TextAreaSelectionMenuHidden,
    TextAreaBlurOnSubmit,
    TextAreaInputFilter,
    TextAreaSelectedBackgroundColor,
    TextAreaEnterKeyType,
    TextAreaEnableKeyboardOnFocus,
    TextAreaCaretOffset,
    TextAreaContentRect,
    TextAreaContentLineCount,
    TextAreaTextSelection,
    TextAreaEnableAutoFill,
    TextAreaContentType,
    TextAreaShowKeyboardOnFocus,
    TextAreaNumberOfLines,
    ButtonLabel,
    ButtonType,
    ProgressValue,
    ProgressTotal,
    ProgressColor,
    ProgressType,
    CheckboxSelect,
    CheckboxSelectColor,
    CheckboxUnselectColor,
    CheckboxMark,
    CheckboxShape,
    XComponentId,
    XComponentType,
    XComponentSurfaceSize,
    DatePickerLunar,
    DatePickerStart,
    DatePickerEnd,
    DatePickerSelected,
    DatePickerDisappearTextStyle,
    DatePickerTextStyle,
    DatePickerSelectedTextStyle,
    TimePickerSelected,
    TimePickerUseMilitaryTime,
    TimePickerDisappearTextStyle,
    TimePickerTextStyle,
    TimePickerSelectedTextStyle,
    TextPickerOptionRange,
    TextPickerOptionSelected,
    TextPickerOptionValue,
    TextPickerDisappearTextStyle,
    TextPickerTextStyle,
    TextPickerSelectedTextStyle,
    TextPickerSelectedIndex,
    TextPickerCanLoop,
    TextPickerDefaultPickerItemHeight,
    CalendarPickerHintRadius,
    CalendarPickerSelectedDate,
    CalendarPickerEdgeAlignment,
    CalendarPickerTextStyle,
    SliderBlockColor,
    SliderTrackColor,
    SliderSelectedColor,
    SliderShowSteps,
    SliderBlockStyle,
    SliderValue,
    SliderMinValue,
    SliderMaxValue,
    SliderStep,
    SliderDirection,
    SliderReverse,
    SliderStyle,
    SliderTrackThickness,
    RadioChecked,
    RadioStyle,
    RadioValue,
    RadioGroup,
    ImageAnimatorImages,
    ImageAnimatorState,
    ImageAnimatorDuration,
    ImageAnimatorReverse,
    ImageAnimatorFixedSize,
    ImageAnimatorFillMode,
    ImageAnimatorIteration,
    StackAlignContent,
    ScrollBarDisplayMode,
    ScrollBarWidth,
    ScrollBarColor,
    ScrollScrollDirection,
    ScrollEdgeEffect,
    ScrollEnableScrollInteraction,
    ScrollFriction,
    ScrollSnap,
    ScrollNestedScroll,
    ScrollOffset,
    ScrollEdge,
    ScrollEnablePaging,
    ScrollPage,
    ScrollBy,
    ListDirection,
    ListSticky,
    ListSpace,
    ListCachedCount,
    ListScrollToIndex,
    ListAlignListItem,
    ListChildrenMainSize,
    ListInitialIndex,
    ListDivider,
    SwiperLoop,
    SwiperAutoPlay,
    SwiperShowIndicator,
    SwiperInterval,
    SwiperVertical,
    SwiperDuration,
    SwiperCurve,
    SwiperItemSpace,
    SwiperIndex,
    SwiperDisplayCount,
    SwiperDisableSwipe,
    SwiperShowDisplayArrow,
    SwiperEdgeEffectMode,
    SwiperNodeAdapter,
    SwiperCachedCount,
    SwiperPrevMargin,
    SwiperNextMargin,
    SwiperIndicator,
    SwiperNestedScroll,
    SwiperSwipeToIndex,
    SwiperIndicatorInteractive,
    ListItemSwipeAction,
    ListItemGroupSetHeader,
    ListItemGroupSetFooter,
    ListItemGroupSetDivider,
    ListItemGroupChildrenMainSize,
    ColumnAlignItems,
    ColumnJustifyContent,
    RowAlignItems,
    RowJustifyContent,
    FlexOption,
    RefreshRefreshing,
    RefreshContent,
    RefreshPullDownRatio,
    RefreshOffset,
    RefreshPullToRefresh,
    WaterFlowLayoutDirection,
    WaterFlowColumnTemplate,
    WaterFlowRowTemplate,
    WaterFlowColumnGap,
    WaterFlowRowGap,
    WaterFlowSectionOption,
    WaterFlowNodeAdapter,
    WaterFlowCachedCount,
    WaterFlowFooter,
    WaterFlowScrollToIndex,
    WaterFlowItemConstraintSize,
    RelativeContainerGuideLine,
    RelativeContainerBarrier,
    GridColumnTemplate,
    GridRowTemplate,
    GridColumnGap,
    GridRowGap,
    GridNodeAdapter,
    GridCachedCount,
    #[cfg(feature = "api-21")]
    AllowForceDark,
    #[cfg(feature = "api-15")]
    BackdropBlur,
    #[cfg(feature = "api-19")]
    BackgroundImageResizableWithSlice,
    #[cfg(feature = "api-18")]
    ButtonMaxFontScale,
    #[cfg(feature = "api-18")]
    ButtonMinFontScale,
    #[cfg(feature = "api-19")]
    CalendarPickerDisabledDateRange,
    #[cfg(feature = "api-18")]
    CalendarPickerEnd,
    #[cfg(feature = "api-19")]
    CalendarPickerMarkToday,
    #[cfg(feature = "api-18")]
    CalendarPickerStart,
    #[cfg(feature = "api-15")]
    CheckboxGroup,
    #[cfg(feature = "api-15")]
    CheckboxGroupMark,
    #[cfg(feature = "api-15")]
    CheckboxGroupName,
    #[cfg(feature = "api-15")]
    CheckboxGroupSelectedColor,
    #[cfg(feature = "api-15")]
    CheckboxGroupSelectAll,
    #[cfg(feature = "api-15")]
    CheckboxGroupShape,
    #[cfg(feature = "api-15")]
    CheckboxGroupUnselectedColor,
    #[cfg(feature = "api-15")]
    CheckboxName,
    #[cfg(feature = "api-20")]
    DatePickerCanLoop,
    #[cfg(feature = "api-18")]
    DatePickerEnableHapticFeedback,
    #[cfg(feature = "api-18")]
    DatePickerMode,
    #[cfg(feature = "api-20")]
    EmbeddedComponentOption,
    #[cfg(feature = "api-20")]
    EmbeddedComponentWant,
    #[cfg(feature = "api-22")]
    GridAlignItems,
    #[cfg(feature = "api-22")]
    GridColumnTemplateItemfillpolicy,
    #[cfg(feature = "api-20")]
    GridFocusWrapMode,
    #[cfg(feature = "api-22")]
    GridItemStyle,
    #[cfg(feature = "api-22")]
    GridLayoutOptions,
    #[cfg(feature = "api-20")]
    GridSyncLoad,
    #[cfg(feature = "api-21")]
    HeightLayoutpolicy,
    #[cfg(feature = "api-22")]
    ImageAltError,
    #[cfg(feature = "api-22")]
    ImageAltPlaceholder,
    #[cfg(feature = "api-21")]
    ImageContentTransition,
    #[cfg(feature = "api-21")]
    ImageCopyOption,
    #[cfg(feature = "api-21")]
    ImageDynamicRangeMode,
    #[cfg(feature = "api-21")]
    ImageEnableAnalyzer,
    #[cfg(feature = "api-21")]
    ImageHdrBrightness,
    #[cfg(feature = "api-21")]
    ImageImageMatrix,
    #[cfg(feature = "api-21")]
    ImageMatchTextDirection,
    #[cfg(feature = "api-21")]
    ImageOrientation,
    #[cfg(feature = "api-21")]
    ImageSourceSize,
    #[cfg(feature = "api-13")]
    ImageSpanBaselineOffset,
    #[cfg(feature = "api-22")]
    ImageSpanColorFilter,
    #[cfg(feature = "api-22")]
    ImageSpanSupportSvg2,
    #[cfg(feature = "api-21")]
    ImageSupportSvg2,
    #[cfg(feature = "api-20")]
    ImageSyncLoad,
    #[cfg(feature = "api-15")]
    ImmutableFontWeight,
    #[cfg(feature = "api-20")]
    ListFocusWrapMode,
    #[cfg(feature = "api-15")]
    ListItemGroupNodeAdapter,
    #[cfg(feature = "api-15")]
    ListLanes,
    #[cfg(feature = "api-22")]
    ListLanesItemfillpolicy,
    #[cfg(feature = "api-15")]
    ListMaintainVisibleContentPosition,
    ListNodeAdapter,
    #[cfg(feature = "api-15")]
    ListScrollSnapAlign,
    #[cfg(feature = "api-22")]
    ListScrollSnapAnimationSpeed,
    #[cfg(feature = "api-15")]
    ListScrollToIndexInGroup,
    #[cfg(feature = "api-19")]
    ListStackFromEnd,
    #[cfg(feature = "api-20")]
    ListSyncLoad,
    #[cfg(feature = "api-18")]
    NextFocus,
    #[cfg(feature = "api-21")]
    PixelRound,
    #[cfg(feature = "api-21")]
    PositionEdges,
    #[cfg(feature = "api-15")]
    ProgressLinearStyle,
    #[cfg(feature = "api-20")]
    RefreshMaxPullDownDistance,
    #[cfg(feature = "api-20")]
    RotateAngle,
    #[cfg(feature = "api-15")]
    ScrollBackToTop,
    #[cfg(feature = "api-20")]
    ScrollBarMargin,
    #[cfg(feature = "api-18")]
    ScrollClipContent,
    #[cfg(feature = "api-15")]
    ScrollContentEndOffset,
    #[cfg(feature = "api-15")]
    ScrollContentStartOffset,
    #[cfg(feature = "api-20")]
    ScrollEnableBouncesZoom,
    #[cfg(feature = "api-14")]
    ScrollFadingEdge,
    #[cfg(feature = "api-13")]
    ScrollFling,
    #[cfg(feature = "api-18")]
    ScrollFlingSpeedLimit,
    #[cfg(feature = "api-20")]
    ScrollMaxZoomScale,
    #[cfg(feature = "api-20")]
    ScrollMinZoomScale,
    #[cfg(feature = "api-14")]
    ScrollSize,
    #[cfg(feature = "api-20")]
    ScrollZoomScale,
    #[cfg(feature = "api-21")]
    SliderBlockLinearGradientColor,
    #[cfg(feature = "api-18")]
    SliderEnableHapticFeedback,
    #[cfg(feature = "api-20")]
    SliderPrefix,
    #[cfg(feature = "api-21")]
    SliderSelectedLinearGradientColor,
    #[cfg(feature = "api-20")]
    SliderSuffix,
    #[cfg(feature = "api-21")]
    SliderTrackLinearGradientColor,
    #[cfg(feature = "api-19")]
    SwiperAutoFill,
    #[cfg(feature = "api-22")]
    SwiperItemfillpolicy,
    #[cfg(feature = "api-20")]
    SwiperMaintainVisibleContentPosition,
    #[cfg(feature = "api-15")]
    SwiperPageFlipMode,
    #[cfg(feature = "api-14")]
    TabStop,
    #[cfg(feature = "api-22")]
    TextAreaBarState,
    #[cfg(feature = "api-22")]
    TextAreaCustomKeyboard,
    #[cfg(feature = "api-15")]
    TextAreaEnablePreviewText,
    #[cfg(feature = "api-22")]
    TextAreaEnableSelectedDataDetector,
    #[cfg(feature = "api-18")]
    TextAreaHalfLeading,
    #[cfg(feature = "api-15")]
    TextAreaKeyboardAppearance,
    #[cfg(feature = "api-15")]
    TextAreaLetterSpacing,
    #[cfg(feature = "api-20")]
    TextAreaLineHeight,
    #[cfg(feature = "api-20")]
    TextAreaLineSpacing,
    #[cfg(feature = "api-20")]
    TextAreaMaxLines,
    #[cfg(feature = "api-20")]
    TextAreaMaxLinesWithScroll,
    #[cfg(feature = "api-20")]
    TextAreaMinLines,
    #[cfg(feature = "api-22")]
    TextAreaScrollBarColor,
    #[cfg(feature = "api-22")]
    TextBindSelectionMenu,
    #[cfg(feature = "api-21")]
    TextContentAlign,
    #[cfg(feature = "api-22")]
    TextEditMenuOptions,
    #[cfg(feature = "api-22")]
    TextEnableSelectedDataDetector,
    #[cfg(feature = "api-20")]
    TextInputEnableFillAnimation,
    #[cfg(feature = "api-15")]
    TextInputEnablePreviewText,
    #[cfg(feature = "api-22")]
    TextInputEnableSelectedDataDetector,
    #[cfg(feature = "api-18")]
    TextInputHalfLeading,
    #[cfg(feature = "api-15")]
    TextInputKeyboardAppearance,
    #[cfg(feature = "api-15")]
    TextInputLetterSpacing,
    #[cfg(feature = "api-20")]
    TextInputLineHeight,
    #[cfg(feature = "api-22")]
    TextInputShowCounter,
    #[cfg(feature = "api-22")]
    TextLayoutManager,
    #[cfg(feature = "api-20")]
    TextLinearGradient,
    #[cfg(feature = "api-20")]
    TextLineCount,
    #[cfg(feature = "api-22")]
    TextLineHeightMultiple,
    #[cfg(feature = "api-22")]
    TextMaxLineHeight,
    #[cfg(feature = "api-22")]
    TextMinLines,
    #[cfg(feature = "api-22")]
    TextMinLineHeight,
    #[cfg(feature = "api-20")]
    TextOptimizeTrailingSpace,
    #[cfg(feature = "api-18")]
    TextPickerColumnWidths,
    #[cfg(feature = "api-18")]
    TextPickerEnableHapticFeedback,
    #[cfg(feature = "api-20")]
    TextPickerSelectedBackgroundStyle,
    #[cfg(feature = "api-20")]
    TextRadialGradient,
    #[cfg(feature = "api-20")]
    TextVerticalAlign,
    #[cfg(feature = "api-18")]
    TimePickerEnableCascade,
    #[cfg(feature = "api-18")]
    TimePickerEnd,
    #[cfg(feature = "api-18")]
    TimePickerStart,
    #[cfg(feature = "api-20")]
    TranslateWithPercent,
    #[cfg(feature = "api-17")]
    VisibleAreaApproximateChangeRatio,
    #[cfg(feature = "api-22")]
    WaterFlowColumnTemplateItemfillpolicy,
    #[cfg(feature = "api-18")]
    WaterFlowLayoutMode,
    #[cfg(feature = "api-20")]
    WaterFlowSyncLoad,
    #[cfg(feature = "api-21")]
    WidthLayoutpolicy,
    #[cfg(feature = "api-18")]
    XComponentEnableAnalyzer,
    #[cfg(feature = "api-18")]
    XComponentSurfaceRect,
}
