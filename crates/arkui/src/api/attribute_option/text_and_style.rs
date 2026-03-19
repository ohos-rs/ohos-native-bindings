//! Module api::attribute_option::text_and_style wrappers and related types.

use std::{os::raw::c_void, ptr::NonNull};

#[cfg(feature = "api-22")]
use std::{ffi::CString, os::raw::c_char};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys::*;

use super::base::{non_null_or_panic, with_cstring};
use crate::{ArkUIError, ArkUIResult};

#[cfg(any(feature = "api-20", feature = "api-22"))]
use super::base::c_char_ptr_to_string;
#[cfg(any(feature = "api-20", feature = "api-22"))]
use crate::check_arkui_status;

#[cfg(feature = "api-22")]
/// Single text menu item descriptor.
pub struct TextMenuItem {
    raw: NonNull<ArkUI_TextMenuItem>,
}

#[cfg(feature = "api-22")]
impl TextMenuItem {
    pub fn new() -> ArkUIResult<Self> {
        let item = unsafe { OH_ArkUI_TextMenuItem_Create() };
        NonNull::new(item)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextMenuItem_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TextMenuItem {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextMenuItem) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextMenuItem"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextMenuItem {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_TextMenuItem_Dispose(self.raw()) }
    }

    pub fn set_content(&mut self, content: &str) -> ArkUIResult<()> {
        let content = CString::new(content).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "string contains interior NUL bytes",
            )
        })?;
        unsafe {
            check_arkui_status!(OH_ArkUI_TextMenuItem_SetContent(
                self.raw(),
                content.as_ptr()
            ))
        }
    }

    pub fn get_content(&self) -> ArkUIResult<String> {
        text_menu_item_read_string(|buffer, buffer_size, write_length| unsafe {
            OH_ArkUI_TextMenuItem_GetContent(self.raw(), buffer, buffer_size, write_length)
        })
    }

    pub fn set_icon(&mut self, icon: &str) -> ArkUIResult<()> {
        let icon = CString::new(icon).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "string contains interior NUL bytes",
            )
        })?;
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItem_SetIcon(self.raw(), icon.as_ptr())) }
    }

    pub fn get_icon(&self) -> ArkUIResult<String> {
        text_menu_item_read_string(|buffer, buffer_size, write_length| unsafe {
            OH_ArkUI_TextMenuItem_GetIcon(self.raw(), buffer, buffer_size, write_length)
        })
    }

    pub fn set_label_info(&mut self, label_info: &str) -> ArkUIResult<()> {
        let label_info = CString::new(label_info).map_err(|_| {
            ArkUIError::new(
                ArkUIErrorCode::ParamInvalid,
                "string contains interior NUL bytes",
            )
        })?;
        unsafe {
            check_arkui_status!(OH_ArkUI_TextMenuItem_SetLabelInfo(
                self.raw(),
                label_info.as_ptr()
            ))
        }
    }

    pub fn get_label_info(&self) -> ArkUIResult<String> {
        text_menu_item_read_string(|buffer, buffer_size, write_length| unsafe {
            OH_ArkUI_TextMenuItem_GetLabelInfo(self.raw(), buffer, buffer_size, write_length)
        })
    }

    pub fn set_id(&mut self, id: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItem_SetId(self.raw(), id)) }
    }

    pub fn get_id(&self) -> ArkUIResult<i32> {
        let mut id = 0;
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItem_GetId(self.raw(), &mut id)) }?;
        Ok(id)
    }
}

#[cfg(feature = "api-22")]
/// Owned array wrapper for text menu items.
pub struct TextMenuItemArray {
    raw: NonNull<ArkUI_TextMenuItemArray>,
}

#[cfg(feature = "api-22")]
impl TextMenuItemArray {
    pub(crate) fn raw(&self) -> *mut ArkUI_TextMenuItemArray {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextMenuItemArray) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextMenuItemArray"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextMenuItemArray {
        self.raw.as_ptr()
    }

    pub fn get_size(&self) -> ArkUIResult<i32> {
        let mut size = 0;
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItemArray_GetSize(self.raw(), &mut size)) }?;
        Ok(size)
    }

    pub fn get_item(&self, index: i32) -> ArkUIResult<TextMenuItem> {
        let mut item = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_TextMenuItemArray_GetItem(
                self.raw(),
                index,
                &mut item
            ))
        }?;
        NonNull::new(item)
            .map(|raw| TextMenuItem::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextMenuItemArray_GetItem returned null",
                )
            })
    }

    pub fn insert(&mut self, item: &TextMenuItem, index: i32) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextMenuItemArray_Insert(
                self.raw(),
                item.raw(),
                index
            ))
        }
    }

    pub fn erase(&mut self, index: i32) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItemArray_Erase(self.raw(), index)) }
    }

    pub fn clear(&mut self) -> ArkUIResult<()> {
        unsafe { check_arkui_status!(OH_ArkUI_TextMenuItemArray_Clear(self.raw())) }
    }
}

#[cfg(feature = "api-22")]
fn text_menu_item_read_string<F>(mut getter: F) -> ArkUIResult<String>
where
    F: FnMut(*mut c_char, i32, *mut i32) -> u32,
{
    let mut write_length = 0;
    let mut buffer = vec![0u8; 256];
    let mut status = getter(
        buffer.as_mut_ptr().cast(),
        buffer.len() as i32,
        &mut write_length,
    );
    if write_length > buffer.len() as i32 {
        buffer.resize(write_length as usize, 0);
        status = getter(
            buffer.as_mut_ptr().cast(),
            buffer.len() as i32,
            &mut write_length,
        );
    }
    check_arkui_status!(status)?;
    let mut end = (write_length as usize).min(buffer.len());
    if end == 0 {
        end = buffer.iter().position(|v| *v == 0).unwrap_or(0);
    } else if buffer.get(end.saturating_sub(1)).copied() == Some(0) {
        end -= 1;
    }
    Ok(String::from_utf8_lossy(&buffer[..end]).into_owned())
}

#[cfg(feature = "api-22")]
struct TextEditMenuCreateCallbackContext {
    callback: Box<dyn Fn(&mut TextMenuItemArray)>,
}

#[cfg(feature = "api-22")]
struct TextEditMenuPrepareCallbackContext {
    callback: Box<dyn Fn(&mut TextMenuItemArray)>,
}

#[cfg(feature = "api-22")]
struct TextEditMenuItemClickCallbackContext {
    callback: Box<dyn Fn(&TextMenuItem, i32, i32) -> bool>,
}

#[cfg(feature = "api-22")]
/// Options for configuring text edit context menus.
pub struct TextEditMenuOptions {
    raw: NonNull<ArkUI_TextEditMenuOptions>,
    on_create_menu_callback: Option<*mut TextEditMenuCreateCallbackContext>,
    on_prepare_menu_callback: Option<*mut TextEditMenuPrepareCallbackContext>,
    on_menu_item_click_callback: Option<*mut TextEditMenuItemClickCallbackContext>,
}

#[cfg(feature = "api-22")]
impl TextEditMenuOptions {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_TextEditMenuOptions_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextEditMenuOptions_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TextEditMenuOptions {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextEditMenuOptions) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextEditMenuOptions"),
            on_create_menu_callback: None,
            on_prepare_menu_callback: None,
            on_menu_item_click_callback: None,
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextEditMenuOptions {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        let _ = self.clear_on_create_menu_callback();
        let _ = self.clear_on_prepare_menu_callback();
        let _ = self.clear_on_menu_item_click_callback();
        unsafe { OH_ArkUI_TextEditMenuOptions_Dispose(self.raw()) }
    }

    pub fn register_on_create_menu_callback<T: Fn(&mut TextMenuItemArray) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_create_menu_callback()?;
        let callback = Box::into_raw(Box::new(TextEditMenuCreateCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_TextEditMenuOptions_RegisterOnCreateMenuCallback(
                self.raw(),
                callback.cast(),
                Some(text_edit_menu_create_callback_trampoline),
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_create_menu_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_create_menu_callback(&mut self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextEditMenuOptions_RegisterOnCreateMenuCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
            ))
        }?;
        if let Some(callback) = self.on_create_menu_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub fn register_on_prepare_menu_callback<T: Fn(&mut TextMenuItemArray) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_prepare_menu_callback()?;
        let callback = Box::into_raw(Box::new(TextEditMenuPrepareCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(OH_ArkUI_TextEditMenuOptions_RegisterOnPrepareMenuCallback(
                self.raw(),
                callback.cast(),
                Some(text_edit_menu_prepare_callback_trampoline),
            ))
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_prepare_menu_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_prepare_menu_callback(&mut self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextEditMenuOptions_RegisterOnPrepareMenuCallback(
                self.raw(),
                std::ptr::null_mut(),
                None,
            ))
        }?;
        if let Some(callback) = self.on_prepare_menu_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub fn register_on_menu_item_click_callback<
        T: Fn(&TextMenuItem, i32, i32) -> bool + 'static,
    >(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_menu_item_click_callback()?;
        let callback = Box::into_raw(Box::new(TextEditMenuItemClickCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(
                OH_ArkUI_TextEditMenuOptions_RegisterOnMenuItemClickCallback(
                    self.raw(),
                    callback.cast(),
                    Some(text_edit_menu_item_click_callback_trampoline),
                )
            )
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_menu_item_click_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_menu_item_click_callback(&mut self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                OH_ArkUI_TextEditMenuOptions_RegisterOnMenuItemClickCallback(
                    self.raw(),
                    std::ptr::null_mut(),
                    None,
                )
            )
        }?;
        if let Some(callback) = self.on_menu_item_click_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn text_edit_menu_create_callback_trampoline(
    items: *mut ArkUI_TextMenuItemArray,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut TextEditMenuCreateCallbackContext) };
    let mut items = TextMenuItemArray::from_raw(items);
    (callback.callback)(&mut items);
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn text_edit_menu_prepare_callback_trampoline(
    items: *mut ArkUI_TextMenuItemArray,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut TextEditMenuPrepareCallbackContext) };
    let mut items = TextMenuItemArray::from_raw(items);
    (callback.callback)(&mut items);
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn text_edit_menu_item_click_callback_trampoline(
    item: *const ArkUI_TextMenuItem,
    start: i32,
    end: i32,
    user_data: *mut c_void,
) -> bool {
    if user_data.is_null() {
        return false;
    }
    let Some(item) = NonNull::new(item.cast_mut()) else {
        return false;
    };
    let callback = unsafe { &*(user_data as *mut TextEditMenuItemClickCallbackContext) };
    let item = TextMenuItem::from_raw(item.as_ptr());
    (callback.callback)(&item, start, end)
}

#[cfg(feature = "api-22")]
struct TextSelectionMenuCallbackContext {
    callback: Box<dyn Fn(i32, i32)>,
}

#[cfg(feature = "api-22")]
/// Options for configuring text selection menus.
pub struct TextSelectionMenuOptions {
    raw: NonNull<ArkUI_TextSelectionMenuOptions>,
    on_menu_show_callback: Option<*mut TextSelectionMenuCallbackContext>,
    on_menu_hide_callback: Option<*mut TextSelectionMenuCallbackContext>,
}

#[cfg(feature = "api-22")]
impl TextSelectionMenuOptions {
    pub fn new() -> ArkUIResult<Self> {
        let option = unsafe { OH_ArkUI_TextSelectionMenuOptions_Create() };
        NonNull::new(option)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextSelectionMenuOptions_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_TextSelectionMenuOptions {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextSelectionMenuOptions) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextSelectionMenuOptions"),
            on_menu_show_callback: None,
            on_menu_hide_callback: None,
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextSelectionMenuOptions {
        self.raw.as_ptr()
    }

    pub fn dispose(mut self) {
        let _ = self.clear_on_menu_show_callback();
        let _ = self.clear_on_menu_hide_callback();
        unsafe { OH_ArkUI_TextSelectionMenuOptions_Dispose(self.raw()) }
    }

    pub fn set_span_type(&mut self, span_type: crate::TextSpanType) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_SetSpanType(
                self.raw(),
                span_type.into()
            ))
        }
    }

    pub fn get_span_type(&self) -> ArkUIResult<crate::TextSpanType> {
        let mut span_type = ArkUI_TextSpanType_ARKUI_TEXT_SPAN_TYPE_DEFAULT;
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_GetSpanType(
                self.raw(),
                &mut span_type
            ))
        }?;
        Ok(span_type.into())
    }

    pub fn set_content_node(&mut self, node: &crate::ArkUINode) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_SetContentNode(
                self.raw(),
                node.raw()
            ))
        }
    }

    pub fn get_content_node(&self) -> ArkUIResult<Option<crate::ArkUINode>> {
        let mut node = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_GetContentNode(
                self.raw(),
                &mut node
            ))
        }?;
        Ok(crate::ArkUINode::from_raw_handle(node))
    }

    pub fn set_response_type(&mut self, response_type: crate::TextResponseType) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_SetResponseType(
                self.raw(),
                response_type.into()
            ))
        }
    }

    pub fn get_response_type(&self) -> ArkUIResult<crate::TextResponseType> {
        let mut response_type = ArkUI_TextResponseType_ARKUI_TEXT_RESPONSE_TYPE_DEFAULT;
        unsafe {
            check_arkui_status!(OH_ArkUI_TextSelectionMenuOptions_GetResponseType(
                self.raw(),
                &mut response_type
            ))
        }?;
        Ok(response_type.into())
    }

    pub fn register_on_menu_show_callback<T: Fn(i32, i32) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_menu_show_callback()?;
        let callback = Box::into_raw(Box::new(TextSelectionMenuCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(
                OH_ArkUI_TextSelectionMenuOptions_RegisterOnMenuShowCallback(
                    self.raw(),
                    callback.cast(),
                    Some(text_selection_menu_callback_trampoline),
                )
            )
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_menu_show_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_menu_show_callback(&mut self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                OH_ArkUI_TextSelectionMenuOptions_RegisterOnMenuShowCallback(
                    self.raw(),
                    std::ptr::null_mut(),
                    None,
                )
            )
        }?;
        if let Some(callback) = self.on_menu_show_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }

    pub fn register_on_menu_hide_callback<T: Fn(i32, i32) + 'static>(
        &mut self,
        callback: T,
    ) -> ArkUIResult<()> {
        self.clear_on_menu_hide_callback()?;
        let callback = Box::into_raw(Box::new(TextSelectionMenuCallbackContext {
            callback: Box::new(callback),
        }));
        let result = unsafe {
            check_arkui_status!(
                OH_ArkUI_TextSelectionMenuOptions_RegisterOnMenuHideCallback(
                    self.raw(),
                    callback.cast(),
                    Some(text_selection_menu_callback_trampoline),
                )
            )
        };
        if let Err(err) = result {
            unsafe {
                drop(Box::from_raw(callback));
            }
            return Err(err);
        }
        self.on_menu_hide_callback = Some(callback);
        Ok(())
    }

    pub fn clear_on_menu_hide_callback(&mut self) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(
                OH_ArkUI_TextSelectionMenuOptions_RegisterOnMenuHideCallback(
                    self.raw(),
                    std::ptr::null_mut(),
                    None,
                )
            )
        }?;
        if let Some(callback) = self.on_menu_hide_callback.take() {
            unsafe {
                drop(Box::from_raw(callback));
            }
        }
        Ok(())
    }
}

#[cfg(feature = "api-22")]
unsafe extern "C" fn text_selection_menu_callback_trampoline(
    start: i32,
    end: i32,
    user_data: *mut c_void,
) {
    if user_data.is_null() {
        return;
    }
    let callback = unsafe { &*(user_data as *mut TextSelectionMenuCallbackContext) };
    (callback.callback)(start, end);
}

/// Wrapper for native styled-string object.
pub struct StyledString {
    raw: NonNull<ArkUI_StyledString>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing typography style object.
pub struct DrawingTypographyStyle {
    raw: NonNull<c_void>,
}

impl DrawingTypographyStyle {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_TypographyStyle`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub(crate) fn raw(&self) -> *mut OH_Drawing_TypographyStyle {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing font collection object.
pub struct DrawingFontCollection {
    raw: NonNull<c_void>,
}

impl DrawingFontCollection {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_FontCollection`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub(crate) fn raw(&self) -> *mut OH_Drawing_FontCollection {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing text style object.
pub struct DrawingTextStyle {
    raw: NonNull<c_void>,
}

impl DrawingTextStyle {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_TextStyle`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub(crate) fn raw(&self) -> *mut OH_Drawing_TextStyle {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing placeholder span object.
pub struct DrawingPlaceholderSpan {
    raw: NonNull<c_void>,
}

impl DrawingPlaceholderSpan {
    /// # Safety
    /// The pointer must be a valid `OH_Drawing_PlaceholderSpan`.
    pub unsafe fn from_raw(raw: *mut c_void) -> Option<Self> {
        NonNull::new(raw).map(|raw| Self { raw })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub(crate) fn raw(&self) -> *mut OH_Drawing_PlaceholderSpan {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing typography object.
pub struct DrawingTypography {
    raw: NonNull<c_void>,
}

impl DrawingTypography {
    pub(crate) fn from_raw(raw: *mut OH_Drawing_Typography) -> ArkUIResult<Self> {
        NonNull::new(raw.cast())
            .map(|raw| Self { raw })
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_StyledString_CreateTypography returned null",
                )
            })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }

    pub(crate) fn raw(&self) -> *mut OH_Drawing_Typography {
        self.raw.as_ptr().cast()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for drawing text box object.
pub struct DrawingTextBox {
    raw: NonNull<c_void>,
}

impl DrawingTextBox {
    pub(crate) fn from_raw(raw: *mut OH_Drawing_TextBox) -> ArkUIResult<Self> {
        NonNull::new(raw.cast())
            .map(|raw| Self { raw })
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextLayoutManager_GetRectsForRange returned null",
                )
            })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
/// Wrapper for text position-and-affinity object.
pub struct DrawingPositionAndAffinity {
    raw: NonNull<c_void>,
}

impl DrawingPositionAndAffinity {
    pub(crate) fn from_raw(raw: *mut OH_Drawing_PositionAndAffinity) -> ArkUIResult<Self> {
        NonNull::new(raw.cast())
            .map(|raw| Self { raw })
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_TextLayoutManager_GetGlyphPositionAtCoordinate returned null",
                )
            })
    }

    pub fn as_raw(&self) -> NonNull<c_void> {
        self.raw
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Wrapper for drawing font metrics object.
pub struct DrawingFontMetrics {
    pub flags: u32,
    pub top: f32,
    pub ascent: f32,
    pub descent: f32,
    pub bottom: f32,
    pub leading: f32,
    pub avg_char_width: f32,
    pub max_char_width: f32,
    pub x_min: f32,
    pub x_max: f32,
    pub x_height: f32,
    pub cap_height: f32,
    pub underline_thickness: f32,
    pub underline_position: f32,
    pub strikeout_thickness: f32,
    pub strikeout_position: f32,
}

impl From<OH_Drawing_Font_Metrics> for DrawingFontMetrics {
    fn from(value: OH_Drawing_Font_Metrics) -> Self {
        Self {
            flags: value.flags,
            top: value.top,
            ascent: value.ascent,
            descent: value.descent,
            bottom: value.bottom,
            leading: value.leading,
            avg_char_width: value.avgCharWidth,
            max_char_width: value.maxCharWidth,
            x_min: value.xMin,
            x_max: value.xMax,
            x_height: value.xHeight,
            cap_height: value.capHeight,
            underline_thickness: value.underlineThickness,
            underline_position: value.underlinePosition,
            strikeout_thickness: value.strikeoutThickness,
            strikeout_position: value.strikeoutPosition,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// Wrapper for drawing line metrics object.
pub struct DrawingLineMetrics {
    pub ascender: f64,
    pub descender: f64,
    pub cap_height: f64,
    pub x_height: f64,
    pub width: f64,
    pub height: f64,
    pub x: f64,
    pub y: f64,
    pub start_index: usize,
    pub end_index: usize,
    pub first_char_metrics: DrawingFontMetrics,
}

impl From<OH_Drawing_LineMetrics> for DrawingLineMetrics {
    fn from(value: OH_Drawing_LineMetrics) -> Self {
        Self {
            ascender: value.ascender,
            descender: value.descender,
            cap_height: value.capHeight,
            x_height: value.xHeight,
            width: value.width,
            height: value.height,
            x: value.x,
            y: value.y,
            start_index: value.startIndex,
            end_index: value.endIndex,
            first_char_metrics: value.firstCharMetrics.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Width policy used when querying text rects.
pub enum TextRectWidthStyle {
    Tight,
    Max,
}

impl From<TextRectWidthStyle> for OH_Drawing_RectWidthStyle {
    fn from(value: TextRectWidthStyle) -> Self {
        match value {
            TextRectWidthStyle::Tight => OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_TIGHT,
            TextRectWidthStyle::Max => OH_Drawing_RectWidthStyle_RECT_WIDTH_STYLE_MAX,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
/// Height policy used when querying text rects.
pub enum TextRectHeightStyle {
    Tight,
    Max,
    IncludeLineSpaceMiddle,
    IncludeLineSpaceTop,
    IncludeLineSpaceBottom,
    Struct,
}

impl From<TextRectHeightStyle> for OH_Drawing_RectHeightStyle {
    fn from(value: TextRectHeightStyle) -> Self {
        match value {
            TextRectHeightStyle::Tight => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_TIGHT,
            TextRectHeightStyle::Max => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_MAX,
            TextRectHeightStyle::IncludeLineSpaceMiddle => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEMIDDLE
            }
            TextRectHeightStyle::IncludeLineSpaceTop => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACETOP
            }
            TextRectHeightStyle::IncludeLineSpaceBottom => {
                OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_INCLUDELINESPACEBOTTOM
            }
            TextRectHeightStyle::Struct => OH_Drawing_RectHeightStyle_RECT_HEIGHT_STYLE_STRUCT,
        }
    }
}

impl StyledString {
    pub fn new(
        style: &DrawingTypographyStyle,
        collection: &DrawingFontCollection,
    ) -> ArkUIResult<Self> {
        let handle = unsafe { OH_ArkUI_StyledString_Create(style.raw(), collection.raw()) };
        NonNull::new(handle)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_StyledString_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_StyledString {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_StyledString) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_StyledString"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_StyledString {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_StyledString_Destroy(self.raw()) }
    }

    pub fn push_text_style(&mut self, style: &DrawingTextStyle) {
        unsafe { OH_ArkUI_StyledString_PushTextStyle(self.raw(), style.raw()) }
    }

    pub fn add_text(&mut self, content: &str) -> ArkUIResult<()> {
        with_cstring(content, |content_ptr| unsafe {
            OH_ArkUI_StyledString_AddText(self.raw(), content_ptr)
        })
    }

    pub fn pop_text_style(&mut self) {
        unsafe { OH_ArkUI_StyledString_PopTextStyle(self.raw()) }
    }

    pub fn create_typography(&self) -> ArkUIResult<DrawingTypography> {
        let typography = unsafe { OH_ArkUI_StyledString_CreateTypography(self.raw()) };
        NonNull::new(typography)
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_StyledString_CreateTypography returned null",
                )
            })
            .and_then(|raw| DrawingTypography::from_raw(raw.as_ptr()))
    }

    pub fn add_placeholder(&mut self, placeholder: &DrawingPlaceholderSpan) {
        unsafe { OH_ArkUI_StyledString_AddPlaceholder(self.raw(), placeholder.raw()) }
    }
}

#[cfg(feature = "api-14")]
/// Wrapper for styled-string descriptor object.
pub struct StyledStringDescriptor {
    raw: NonNull<ArkUI_StyledString_Descriptor>,
}

#[cfg(feature = "api-14")]
impl StyledStringDescriptor {
    pub fn new() -> ArkUIResult<Self> {
        let descriptor = unsafe { OH_ArkUI_StyledString_Descriptor_Create() };
        NonNull::new(descriptor)
            .map(|raw| Self::from_raw(raw.as_ptr()))
            .ok_or_else(|| {
                ArkUIError::new(
                    ArkUIErrorCode::ParamInvalid,
                    "OH_ArkUI_StyledString_Descriptor_Create returned null",
                )
            })
    }

    pub(crate) fn raw(&self) -> *mut ArkUI_StyledString_Descriptor {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_StyledString_Descriptor) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_StyledString_Descriptor"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_StyledString_Descriptor {
        self.raw.as_ptr()
    }

    pub fn destroy(self) {
        unsafe { OH_ArkUI_StyledString_Descriptor_Destroy(self.raw()) }
    }

    pub fn convert_to_html(&self) -> Option<String> {
        c_char_ptr_to_string(unsafe { OH_ArkUI_ConvertToHtml(self.raw()) })
    }

    pub fn unmarshall_from_buffer(&mut self, buffer: &mut [u8]) -> ArkUIResult<()> {
        unsafe {
            check_arkui_status!(OH_ArkUI_UnmarshallStyledStringDescriptor(
                buffer.as_mut_ptr(),
                buffer.len(),
                self.raw()
            ))
        }
    }

    pub fn marshall_to_buffer(&self) -> ArkUIResult<Vec<u8>> {
        let mut buffer = vec![0u8; 256];
        let mut result_size = 0usize;
        let mut status = unsafe {
            OH_ArkUI_MarshallStyledStringDescriptor(
                buffer.as_mut_ptr(),
                buffer.len(),
                self.raw(),
                &mut result_size,
            )
        };
        if result_size > buffer.len() {
            buffer.resize(result_size, 0);
            status = unsafe {
                OH_ArkUI_MarshallStyledStringDescriptor(
                    buffer.as_mut_ptr(),
                    buffer.len(),
                    self.raw(),
                    &mut result_size,
                )
            };
        }
        check_arkui_status!(status)?;
        buffer.truncate(result_size);
        Ok(buffer)
    }
}

#[cfg(feature = "api-22")]
/// Wrapper for text layout manager object.
pub struct TextLayoutManager {
    raw: NonNull<ArkUI_TextLayoutManager>,
}

#[cfg(feature = "api-22")]
impl TextLayoutManager {
    pub(crate) fn raw(&self) -> *mut ArkUI_TextLayoutManager {
        self.raw.as_ptr()
    }

    pub(crate) fn from_raw(raw: *mut ArkUI_TextLayoutManager) -> Self {
        Self {
            raw: non_null_or_panic(raw, "ArkUI_TextLayoutManager"),
        }
    }

    pub(crate) fn into_raw(self) -> *mut ArkUI_TextLayoutManager {
        self.raw.as_ptr()
    }

    pub fn dispose(self) {
        unsafe { OH_ArkUI_TextLayoutManager_Dispose(self.raw()) }
    }

    pub fn get_line_count(&self) -> ArkUIResult<i32> {
        let mut line_count = 0;
        unsafe {
            check_arkui_status!(OH_ArkUI_TextLayoutManager_GetLineCount(
                self.raw(),
                &mut line_count
            ))
        }?;
        Ok(line_count)
    }

    pub fn get_rects_for_range(
        &self,
        start: i32,
        end: i32,
        width_style: TextRectWidthStyle,
        height_style: TextRectHeightStyle,
    ) -> ArkUIResult<DrawingTextBox> {
        let mut text_boxes = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_TextLayoutManager_GetRectsForRange(
                self.raw(),
                start,
                end,
                width_style.into(),
                height_style.into(),
                &mut text_boxes
            ))
        }?;
        DrawingTextBox::from_raw(text_boxes)
    }

    pub fn get_glyph_position_at_coordinate(
        &self,
        dx: f64,
        dy: f64,
    ) -> ArkUIResult<DrawingPositionAndAffinity> {
        let mut position = std::ptr::null_mut();
        unsafe {
            check_arkui_status!(OH_ArkUI_TextLayoutManager_GetGlyphPositionAtCoordinate(
                self.raw(),
                dx,
                dy,
                &mut position
            ))
        }?;
        DrawingPositionAndAffinity::from_raw(position)
    }

    pub fn get_line_metrics(&self, line_number: i32) -> ArkUIResult<DrawingLineMetrics> {
        let mut metrics = std::mem::MaybeUninit::<OH_Drawing_LineMetrics>::uninit();
        unsafe {
            check_arkui_status!(OH_ArkUI_TextLayoutManager_GetLineMetrics(
                self.raw(),
                line_number,
                metrics.as_mut_ptr(),
            ))?;
            Ok(metrics.assume_init().into())
        }
    }
}
