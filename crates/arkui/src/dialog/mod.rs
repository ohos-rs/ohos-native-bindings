use std::{cell::RefCell, os::raw::c_void, rc::Rc};

use crate::{Alignment, ArkUINode, ArkUIResult, DismissReason, ARK_UI_NATIVE_DIALOG_API_1};

use ohos_arkui_sys::ArkUI_NativeDialogHandle;

pub(crate) struct InnerDialogDismissData {
    pub dismiss_handle: Option<Rc<RefCell<fn(DialogDismissData) -> ()>>>,
    pub data: Option<Rc<RefCell<*mut c_void>>>,
}

/// OnDismiss callback data
pub struct DialogDismissData {
    /// dismiss reason
    pub dismiss_reason: DismissReason,
    /// User custom data,if not and it will be None
    pub data: Option<Rc<RefCell<*mut c_void>>>,
}

pub struct Dialog {
    pub(crate) raw: ArkUI_NativeDialogHandle,
    pub(crate) inner_dismiss_data: Rc<RefCell<InnerDialogDismissData>>,
}

impl Dialog {
    pub fn new() -> ArkUIResult<Self> {
        let dialog_controller = ARK_UI_NATIVE_DIALOG_API_1.create()?;
        Ok(Dialog {
            raw: dialog_controller,
            inner_dismiss_data: Rc::new(RefCell::new(InnerDialogDismissData {
                dismiss_handle: None,
                data: None,
            })),
        })
    }

    pub fn set_content<T: Into<ArkUINode>>(&self, content: T) -> ArkUIResult<()> {
        let node: ArkUINode = content.into();
        ARK_UI_NATIVE_DIALOG_API_1.set_content(self.raw, node.raw())?;
        Ok(())
    }

    pub fn show(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.show(self.raw, false)?;
        Ok(())
    }

    pub fn show_with_sub_window(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.show(self.raw, true)?;
        Ok(())
    }

    pub fn close(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.close(self.raw)?;
        Ok(())
    }

    pub fn set_modal_mode(&self, modal_mode: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.set_modal_mode(self.raw, modal_mode)?;
        Ok(())
    }

    pub fn set_auto_cancel(&self, auto_cancel: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.set_auto_cancel(self.raw, auto_cancel)?;
        Ok(())
    }

    pub fn set_background_color(&self, color: u32) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.set_background_color(self.raw, color)?;
        Ok(())
    }

    /// Set content alignment, offset_x and offset_y will be set with 0.0
    pub fn set_content_alignment(&self, alignment: Alignment) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.set_content_alignment(self.raw, alignment, 0.0, 0.0)?;
        Ok(())
    }

    /// Set content alignment with offset_x and offset_y.
    pub fn set_content_alignment_with_offset(
        &self,
        alignment: Alignment,
        offset_x: f32,
        offset_y: f32,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1
            .set_content_alignment(self.raw, alignment, offset_x, offset_y)?;
        Ok(())
    }

    pub fn set_corner_radius(
        &self,
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.set_corner_radius(
            self.raw,
            top_left,
            top_right,
            bottom_left,
            bottom_right,
        )?;
        Ok(())
    }

    pub fn on_will_dismiss(&self, callback: fn(DialogDismissData) -> ()) -> ArkUIResult<()> {
        self.inner_dismiss_data.borrow_mut().dismiss_handle = Some(Rc::new(RefCell::new(callback)));

        ARK_UI_NATIVE_DIALOG_API_1.register_dismiss(self.raw, self.inner_dismiss_data.clone())?;
        Ok(())
    }

    /// For user custom data, you can use this method
    /// And you need to convert raw pointer to your data type after callback
    pub fn on_will_dismiss_with_data<T: 'static>(
        &self,
        data: T,
        callback: fn(data: DialogDismissData) -> (),
    ) -> ArkUIResult<()> {
        self.inner_dismiss_data.borrow_mut().dismiss_handle = Some(Rc::new(RefCell::new(callback)));

        self.inner_dismiss_data.borrow_mut().data = Some(Rc::new(RefCell::new(Box::into_raw(
            Box::new(data),
        )
            as *mut c_void)));

        ARK_UI_NATIVE_DIALOG_API_1.register_dismiss(self.raw, self.inner_dismiss_data.clone())?;
        Ok(())
    }
}
