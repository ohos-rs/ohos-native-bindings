//! Module api::custom_dialog wrappers and related types.

use std::{
    os::raw::c_void,
    ptr::NonNull,
    sync::{Mutex, OnceLock},
};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use crate::{check_arkui_status, ArkUIError, ArkUIResult, DismissReason};

fn non_null_or_panic<T>(ptr: *mut T, name: &'static str) -> NonNull<T> {
    NonNull::new(ptr).unwrap_or_else(|| panic!("{name} pointer is null"))
}

#[derive(Clone, Copy, Debug, PartialEq)]
/// Rectangle used as custom-dialog mask region.
pub struct CustomDialogMaskRect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl From<CustomDialogMaskRect> for ArkUI_Rect {
    fn from(value: CustomDialogMaskRect) -> Self {
        Self {
            x: value.x,
            y: value.y,
            width: value.width,
            height: value.height,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Border line style used by custom-dialog border edges.
pub enum CustomDialogBorderLineStyle {
    Solid,
    Dashed,
    Dotted,
}

impl From<CustomDialogBorderLineStyle> for i32 {
    fn from(value: CustomDialogBorderLineStyle) -> Self {
        match value {
            CustomDialogBorderLineStyle::Solid => ArkUI_BorderStyle_ARKUI_BORDER_STYLE_SOLID as i32,
            CustomDialogBorderLineStyle::Dashed => {
                ArkUI_BorderStyle_ARKUI_BORDER_STYLE_DASHED as i32
            }
            CustomDialogBorderLineStyle::Dotted => {
                ArkUI_BorderStyle_ARKUI_BORDER_STYLE_DOTTED as i32
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Per-edge border style configuration for custom dialogs.
pub struct CustomDialogBorderStyle {
    pub top: CustomDialogBorderLineStyle,
    pub right: CustomDialogBorderLineStyle,
    pub bottom: CustomDialogBorderLineStyle,
    pub left: CustomDialogBorderLineStyle,
}

impl CustomDialogBorderStyle {
    /// Creates a border style with the same value on all sides.
    pub fn uniform(style: CustomDialogBorderLineStyle) -> Self {
        Self {
            top: style,
            right: style,
            bottom: style,
            left: style,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Borrowed handle to a native custom-dialog instance.
pub struct NativeDialogHandle {
    raw: NonNull<c_void>,
}

impl NativeDialogHandle {
    /// Converts from a high-level [`crate::Dialog`] wrapper.
    pub fn from_dialog(dialog: &crate::Dialog) -> Option<Self> {
        Self::from_raw(dialog.raw)
    }

    pub(crate) fn from_raw(raw: ArkUI_NativeDialogHandle) -> Option<Self> {
        NonNull::new(raw.cast()).map(|raw| Self { raw })
    }

    pub(crate) fn raw(&self) -> ArkUI_NativeDialogHandle {
        self.raw.as_ptr().cast()
    }

    /// Returns current dialog state.
    pub fn state(&self) -> ArkUIResult<crate::DialogState> {
        let mut state = ArkUI_DialogState_DIALOG_UNINITIALIZED;
        unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_GetState(self.raw(), &mut state)) }?;
        Ok(state.into())
    }
}

/// Dismiss event object used by custom-dialog lifecycle callbacks.
pub struct CustomDialogDismissEvent {
    raw: NonNull<ArkUI_DialogDismissEvent>,
}

impl CustomDialogDismissEvent {
    fn from_raw(raw: *mut ArkUI_DialogDismissEvent) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    fn raw(&self) -> *mut ArkUI_DialogDismissEvent {
        self.raw.as_ptr()
    }

    /// Returns dismiss reason reported by ArkUI.
    pub fn dismiss_reason(&self) -> DismissReason {
        let reason = unsafe { OH_ArkUI_DialogDismissEvent_GetDismissReason(self.raw()) } as u32;
        reason.into()
    }

    /// Controls whether this dismiss operation should be blocked.
    pub fn set_should_block_dismiss(&mut self, block: bool) {
        unsafe { OH_ArkUI_DialogDismissEvent_SetShouldBlockDismiss(self.raw(), block) }
    }
}

struct CustomDialogDismissCallbackContext {
    callback: Box<dyn Fn(&mut CustomDialogDismissEvent)>,
}

struct CustomDialogVoidCallbackContext {
    callback: Box<dyn Fn()>,
}

struct CustomDialogIdCallbackContext {
    callback: Box<dyn Fn(i32)>,
}

const CUSTOM_DIALOG_ID_CALLBACK_SLOT_COUNT: usize = 16;
type CustomDialogIdCallbackSlots = [Option<usize>; CUSTOM_DIALOG_ID_CALLBACK_SLOT_COUNT];
type CustomDialogIdCallback = unsafe extern "C" fn(dialog_id: i32);
type CustomDialogDismissCallback = unsafe extern "C" fn(event: *mut ArkUI_DialogDismissEvent);
type CustomDialogVoidCallback = unsafe extern "C" fn(user_data: *mut c_void);

static CUSTOM_DIALOG_ID_CALLBACK_SLOTS: OnceLock<Mutex<CustomDialogIdCallbackSlots>> =
    OnceLock::new();

fn custom_dialog_id_callback_slots() -> &'static Mutex<CustomDialogIdCallbackSlots> {
    CUSTOM_DIALOG_ID_CALLBACK_SLOTS
        .get_or_init(|| Mutex::new([None; CUSTOM_DIALOG_ID_CALLBACK_SLOT_COUNT]))
}

/// Builder-style options wrapper for creating and operating custom dialogs.
pub struct CustomDialogOptions {
    raw: NonNull<ArkUI_CustomDialogOptions>,
    on_will_dismiss_callback: Option<NonNull<CustomDialogDismissCallbackContext>>,
    on_will_appear_callback: Option<NonNull<CustomDialogVoidCallbackContext>>,
    on_did_appear_callback: Option<NonNull<CustomDialogVoidCallbackContext>>,
    on_will_disappear_callback: Option<NonNull<CustomDialogVoidCallbackContext>>,
    on_did_disappear_callback: Option<NonNull<CustomDialogVoidCallbackContext>>,
}

impl CustomDialogOptions {
    /// Creates options from a dialog content node.
    pub fn new(content: &crate::ArkUINode) -> ArkUIResult<Self> {
        if content.raw().is_null() {
            return Err(ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "node handle is null",
            ));
        }

        let options = unsafe { OH_ArkUI_CustomDialog_CreateOptions(content.raw()) };
        let options = NonNull::new(options).ok_or_else(|| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "OH_ArkUI_CustomDialog_CreateOptions returned null",
            )
        })?;
        Ok(Self::from_non_null(options))
    }

    pub fn new_from_node(node: &crate::ArkUINode) -> ArkUIResult<Self> {
        Self::new(node)
    }

    fn from_non_null(raw: NonNull<ArkUI_CustomDialogOptions>) -> Self {
        Self {
            raw,
            on_will_dismiss_callback: None,
            on_will_appear_callback: None,
            on_did_appear_callback: None,
            on_will_disappear_callback: None,
            on_did_disappear_callback: None,
        }
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_CustomDialogOptions) -> Self {
        Self::from_non_null(non_null_or_panic(raw, "ArkUI_CustomDialogOptions"))
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_CustomDialogOptions {
        self.raw.as_ptr()
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_CustomDialogOptions {
        self.raw.as_ptr()
    }

    /// Disposes options and clears all registered callbacks.
    pub fn dispose(mut self) {
        let _ = self.clear_on_will_dismiss_callback();
        let _ = self.clear_on_will_appear_callback();
        let _ = self.clear_on_did_appear_callback();
        let _ = self.clear_on_will_disappear_callback();
        let _ = self.clear_on_did_disappear_callback();
        unsafe { OH_ArkUI_CustomDialog_DisposeOptions(self.raw()) }
    }

    /// Opens a custom dialog using these options.
    pub fn open(&self) -> ArkUIResult<()> {
        custom_dialog_open_dialog_raw(self.raw(), None)
    }

    /// Opens a custom dialog and returns dialog id through callback.
    pub fn open_with_callback<T: Fn(i32) + 'static>(&self, callback: T) -> ArkUIResult<()> {
        self.run_with_dialog_id_callback(callback, custom_dialog_open_dialog_raw)
    }

    /// Updates an existing custom dialog.
    pub fn update(&self) -> ArkUIResult<()> {
        custom_dialog_update_dialog_raw(self.raw(), None)
    }

    /// Updates an existing dialog and returns dialog id through callback.
    pub fn update_with_callback<T: Fn(i32) + 'static>(&self, callback: T) -> ArkUIResult<()> {
        self.run_with_dialog_id_callback(callback, custom_dialog_update_dialog_raw)
    }

    fn run_with_dialog_id_callback<T: Fn(i32) + 'static>(
        &self,
        callback: T,
        runner: fn(
            options: *mut ArkUI_CustomDialogOptions,
            callback: Option<CustomDialogIdCallback>,
        ) -> ArkUIResult<()>,
    ) -> ArkUIResult<()> {
        let callback = non_null_or_panic(
            Box::into_raw(Box::new(CustomDialogIdCallbackContext {
                callback: Box::new(callback),
            })),
            "CustomDialogIdCallbackContext",
        );
        let slot = {
            let mut callbacks = match custom_dialog_id_callback_slots().lock() {
                Ok(callbacks) => callbacks,
                Err(poisoned) => poisoned.into_inner(),
            };
            let reserved =
                reserve_custom_dialog_id_callback_slot(&mut callbacks, callback.as_ptr());
            if let Err(err) = reserved {
                unsafe {
                    drop(Box::from_raw(callback.as_ptr()));
                }
                return Err(err);
            }
            reserved?
        };
        let result = runner(self.raw(), CUSTOM_DIALOG_ID_CALLBACK_TRAMPOLINES[slot]);
        if let Err(err) = result {
            let callback = take_custom_dialog_id_callback_slot(slot);
            if let Some(callback) = callback {
                unsafe {
                    drop(Box::from_raw(
                        callback as *mut CustomDialogIdCallbackContext,
                    ));
                }
            }
            return Err(err);
        }
        Ok(())
    }

    pub fn close(dialog_id: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_CloseDialog(dialog_id)) }
    }

    pub fn set_level_mode(&mut self, level_mode: crate::LevelMode) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetLevelMode(
                self.raw(),
                level_mode.into()
            ))
        }
    }

    pub fn set_level_unique_id(&mut self, unique_id: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetLevelUniqueId(
                self.raw(),
                unique_id
            ))
        }
    }

    pub fn set_immersive_mode(&mut self, immersive_mode: crate::ImmersiveMode) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetImmersiveMode(
                self.raw(),
                immersive_mode.into()
            ))
        }
    }

    pub fn set_background_color(&mut self, background_color: u32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBackgroundColor(
                self.raw(),
                background_color
            ))
        }
    }

    pub fn set_corner_radius(
        &mut self,
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetCornerRadius(
                self.raw(),
                top_left,
                top_right,
                bottom_left,
                bottom_right
            ))
        }
    }

    pub fn set_border_width(
        &mut self,
        top: f32,
        right: f32,
        bottom: f32,
        left: f32,
        unit: crate::LengthMetricUnit,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBorderWidth(
                self.raw(),
                top,
                right,
                bottom,
                left,
                unit.into()
            ))
        }
    }

    pub fn set_border_color(
        &mut self,
        top: u32,
        right: u32,
        bottom: u32,
        left: u32,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBorderColor(
                self.raw(),
                top,
                right,
                bottom,
                left
            ))
        }
    }

    pub fn set_border_style(&mut self, style: CustomDialogBorderStyle) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBorderStyle(
                self.raw(),
                style.top.into(),
                style.right.into(),
                style.bottom.into(),
                style.left.into()
            ))
        }
    }

    pub fn set_width(&mut self, width: f32, unit: crate::LengthMetricUnit) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetWidth(
                self.raw(),
                width,
                unit.into()
            ))
        }
    }

    pub fn set_height(&mut self, height: f32, unit: crate::LengthMetricUnit) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetHeight(
                self.raw(),
                height,
                unit.into()
            ))
        }
    }

    pub fn set_shadow(&mut self, shadow: crate::ShadowStyle) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_SetShadow(self.raw(), shadow.into())) }
    }

    pub fn set_custom_shadow(
        &mut self,
        custom_shadow: crate::ArkUINodeAttributeItem,
    ) -> ArkUIResult<()> {
        let custom_shadow: ArkUI_AttributeItem = custom_shadow.into();
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetCustomShadow(
                self.raw(),
                &custom_shadow
            ))
        }
    }

    pub fn set_background_blur_style(&mut self, blur_style: crate::BlurStyle) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBackgroundBlurStyle(
                self.raw(),
                blur_style.into()
            ))
        }
    }

    pub fn set_alignment(
        &mut self,
        alignment: crate::Alignment,
        offset_x: f32,
        offset_y: f32,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetAlignment(
                self.raw(),
                alignment.into(),
                offset_x,
                offset_y
            ))
        }
    }

    pub fn set_modal_mode(&mut self, is_modal: bool) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_SetModalMode(self.raw(), is_modal)) }
    }

    pub fn set_auto_cancel(&mut self, auto_cancel: bool) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_SetAutoCancel(self.raw(), auto_cancel)) }
    }

    pub fn set_subwindow_mode(&mut self, show_in_subwindow: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetSubwindowMode(
                self.raw(),
                show_in_subwindow
            ))
        }
    }

    pub fn set_mask(
        &mut self,
        mask_color: u32,
        mask_rect: Option<CustomDialogMaskRect>,
    ) -> ArkUIResult<()> {
        let raw_rect = mask_rect.map(ArkUI_Rect::from);
        let raw_rect_ptr = raw_rect
            .as_ref()
            .map_or(std::ptr::null(), |rect| rect as *const ArkUI_Rect);
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetMask(
                self.raw(),
                mask_color,
                raw_rect_ptr
            ))
        }
    }

    pub fn set_keyboard_avoid_mode(
        &mut self,
        keyboard_avoid_mode: crate::KeyboardAvoidMode,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetKeyboardAvoidMode(
                self.raw(),
                keyboard_avoid_mode.into()
            ))
        }
    }

    pub fn set_hover_mode_enabled(&mut self, enabled: bool) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetHoverModeEnabled(
                self.raw(),
                enabled
            ))
        }
    }

    pub fn set_hover_mode_area(
        &mut self,
        hover_mode_area_type: crate::HoverModeAreaType,
    ) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetHoverModeArea(
                self.raw(),
                hover_mode_area_type.into()
            ))
        }
    }

    pub fn set_background_blur_style_options(
        &mut self,
        options: crate::ArkUINodeAttributeItem,
    ) -> ArkUIResult<()> {
        let options: ArkUI_AttributeItem = options.into();
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBackgroundBlurStyleOptions(
                self.raw(),
                &options
            ))
        }
    }

    pub fn set_background_effect(
        &mut self,
        effect: crate::ArkUINodeAttributeItem,
    ) -> ArkUIResult<()> {
        let effect: ArkUI_AttributeItem = effect.into();
        unsafe {
            check_arkui_status!(OH_ArkUI_CustomDialog_SetBackgroundEffect(
                self.raw(),
                &effect
            ))
        }
    }

    pub fn register_on_will_dismiss_callback<T: Fn(&mut CustomDialogDismissEvent) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_will_dismiss_callback()?;
        let callback = Box::into_raw(Box::new(CustomDialogDismissCallbackContext {
            callback: Box::new(callback),
        }));
        let callback = non_null_or_panic(callback, "CustomDialogDismissCallbackContext");
        let result = register_on_will_dismiss_callback_raw(
            self.raw(),
            callback.as_ptr().cast(),
            Some(custom_dialog_will_dismiss_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
            return Err(err);
        }
        self.on_will_dismiss_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_will_dismiss_callback(&mut self) -> ArkUIResult<()> {
        register_on_will_dismiss_callback_raw(self.raw(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_will_dismiss_callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        Ok(())
    }

    pub fn register_on_will_appear_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_will_appear_callback()?;
        let callback = Box::into_raw(Box::new(CustomDialogVoidCallbackContext {
            callback: Box::new(callback),
        }));
        let callback = non_null_or_panic(callback, "CustomDialogVoidCallbackContext");
        let result = register_on_will_appear_callback_raw(
            self.raw(),
            callback.as_ptr().cast(),
            Some(custom_dialog_void_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
            return Err(err);
        }
        self.on_will_appear_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_will_appear_callback(&mut self) -> ArkUIResult<()> {
        register_on_will_appear_callback_raw(self.raw(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_will_appear_callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        Ok(())
    }

    pub fn register_on_did_appear_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_did_appear_callback()?;
        let callback = Box::into_raw(Box::new(CustomDialogVoidCallbackContext {
            callback: Box::new(callback),
        }));
        let callback = non_null_or_panic(callback, "CustomDialogVoidCallbackContext");
        let result = register_on_did_appear_callback_raw(
            self.raw(),
            callback.as_ptr().cast(),
            Some(custom_dialog_void_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
            return Err(err);
        }
        self.on_did_appear_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_did_appear_callback(&mut self) -> ArkUIResult<()> {
        register_on_did_appear_callback_raw(self.raw(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_did_appear_callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        Ok(())
    }

    pub fn register_on_will_disappear_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_will_disappear_callback()?;
        let callback = Box::into_raw(Box::new(CustomDialogVoidCallbackContext {
            callback: Box::new(callback),
        }));
        let callback = non_null_or_panic(callback, "CustomDialogVoidCallbackContext");
        let result = register_on_will_disappear_callback_raw(
            self.raw(),
            callback.as_ptr().cast(),
            Some(custom_dialog_void_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
            return Err(err);
        }
        self.on_will_disappear_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_will_disappear_callback(&mut self) -> ArkUIResult<()> {
        register_on_will_disappear_callback_raw(self.raw(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_will_disappear_callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        Ok(())
    }

    pub fn register_on_did_disappear_callback<T: Fn() + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_did_disappear_callback()?;
        let callback = Box::into_raw(Box::new(CustomDialogVoidCallbackContext {
            callback: Box::new(callback),
        }));
        let callback = non_null_or_panic(callback, "CustomDialogVoidCallbackContext");
        let result = register_on_did_disappear_callback_raw(
            self.raw(),
            callback.as_ptr().cast(),
            Some(custom_dialog_void_callback_trampoline),
        );
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
            return Err(err);
        }
        self.on_did_disappear_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_did_disappear_callback(&mut self) -> ArkUIResult<()> {
        register_on_did_disappear_callback_raw(self.raw(), std::ptr::null_mut(), None)?;
        if let Some(callback) = self.on_did_disappear_callback.take() {
            unsafe {
                drop(Box::from_raw(callback.as_ptr()));
            }
        }
        Ok(())
    }
}

fn reserve_custom_dialog_id_callback_slot(
    slots: &mut CustomDialogIdCallbackSlots,
    callback: *mut CustomDialogIdCallbackContext,
) -> ArkUIResult<usize> {
    for (index, slot) in slots.iter_mut().enumerate() {
        if slot.is_none() {
            *slot = Some(callback as usize);
            return Ok(index);
        }
    }
    Err(ArkUIError::new(
        ArkUIErrorCode::ParamInvalid,
        "custom dialog callback slots exceeded limit",
    ))
}

fn take_custom_dialog_id_callback_slot(slot_index: usize) -> Option<usize> {
    let mut slots = match custom_dialog_id_callback_slots().lock() {
        Ok(slots) => slots,
        Err(poisoned) => poisoned.into_inner(),
    };
    if let Some(slot) = slots.get_mut(slot_index) {
        slot.take()
    } else {
        None
    }
}

macro_rules! define_custom_dialog_id_callback_trampoline {
    ($name:ident, $slot:expr) => {
        unsafe extern "C" fn $name(dialog_id: i32) {
            let callback = take_custom_dialog_id_callback_slot($slot);
            let Some(callback) = callback else {
                return;
            };
            let callback = unsafe { Box::from_raw(callback as *mut CustomDialogIdCallbackContext) };
            (callback.callback)(dialog_id);
        }
    };
}

define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_0, 0);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_1, 1);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_2, 2);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_3, 3);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_4, 4);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_5, 5);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_6, 6);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_7, 7);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_8, 8);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_9, 9);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_10, 10);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_11, 11);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_12, 12);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_13, 13);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_14, 14);
define_custom_dialog_id_callback_trampoline!(custom_dialog_id_callback_trampoline_15, 15);

const CUSTOM_DIALOG_ID_CALLBACK_TRAMPOLINES: [Option<CustomDialogIdCallback>;
    CUSTOM_DIALOG_ID_CALLBACK_SLOT_COUNT] = [
    Some(custom_dialog_id_callback_trampoline_0),
    Some(custom_dialog_id_callback_trampoline_1),
    Some(custom_dialog_id_callback_trampoline_2),
    Some(custom_dialog_id_callback_trampoline_3),
    Some(custom_dialog_id_callback_trampoline_4),
    Some(custom_dialog_id_callback_trampoline_5),
    Some(custom_dialog_id_callback_trampoline_6),
    Some(custom_dialog_id_callback_trampoline_7),
    Some(custom_dialog_id_callback_trampoline_8),
    Some(custom_dialog_id_callback_trampoline_9),
    Some(custom_dialog_id_callback_trampoline_10),
    Some(custom_dialog_id_callback_trampoline_11),
    Some(custom_dialog_id_callback_trampoline_12),
    Some(custom_dialog_id_callback_trampoline_13),
    Some(custom_dialog_id_callback_trampoline_14),
    Some(custom_dialog_id_callback_trampoline_15),
];

fn custom_dialog_open_dialog_raw(
    options: *mut ArkUI_CustomDialogOptions,
    callback: Option<CustomDialogIdCallback>,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_OpenDialog(options, callback)) }
}

fn custom_dialog_update_dialog_raw(
    options: *mut ArkUI_CustomDialogOptions,
    callback: Option<CustomDialogIdCallback>,
) -> ArkUIResult<()> {
    unsafe { check_arkui_status!(OH_ArkUI_CustomDialog_UpdateDialog(options, callback)) }
}

unsafe extern "C" fn custom_dialog_will_dismiss_callback_trampoline(
    event: *mut ArkUI_DialogDismissEvent,
) {
    let Some(mut event) = CustomDialogDismissEvent::from_raw(event) else {
        return;
    };
    let user_data = unsafe { OH_ArkUI_DialogDismissEvent_GetUserData(event.raw()) };
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut CustomDialogDismissCallbackContext) };
    (callback.callback)(&mut event);
}

unsafe extern "C" fn custom_dialog_void_callback_trampoline(user_data: *mut c_void) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut CustomDialogVoidCallbackContext) };
    (callback.callback)();
}

fn register_on_will_dismiss_callback_raw(
    options: *mut ArkUI_CustomDialogOptions,
    user_data: *mut c_void,
    callback: Option<CustomDialogDismissCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_CustomDialog_RegisterOnWillDismissCallback(
            options, user_data, callback
        ))
    }
}

fn register_on_will_appear_callback_raw(
    options: *mut ArkUI_CustomDialogOptions,
    user_data: *mut c_void,
    callback: Option<CustomDialogVoidCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_CustomDialog_RegisterOnWillAppearCallback(
            options, user_data, callback
        ))
    }
}

fn register_on_did_appear_callback_raw(
    options: *mut ArkUI_CustomDialogOptions,
    user_data: *mut c_void,
    callback: Option<CustomDialogVoidCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_CustomDialog_RegisterOnDidAppearCallback(
            options, user_data, callback
        ))
    }
}

fn register_on_will_disappear_callback_raw(
    options: *mut ArkUI_CustomDialogOptions,
    user_data: *mut c_void,
    callback: Option<CustomDialogVoidCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_CustomDialog_RegisterOnWillDisappearCallback(
            options, user_data, callback
        ))
    }
}

fn register_on_did_disappear_callback_raw(
    options: *mut ArkUI_CustomDialogOptions,
    user_data: *mut c_void,
    callback: Option<CustomDialogVoidCallback>,
) -> ArkUIResult<()> {
    unsafe {
        check_arkui_status!(OH_ArkUI_CustomDialog_RegisterOnDidDisappearCallback(
            options, user_data, callback
        ))
    }
}
