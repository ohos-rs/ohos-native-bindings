//! Dialog builder and lifecycle callback wrappers.

use std::{any::Any, cell::RefCell, os::raw::c_void, rc::Rc};

use crate::api::ARK_UI_NATIVE_DIALOG_API_1;
use crate::{Alignment, ArkUINode, ArkUIResult, DismissReason};

use ohos_arkui_sys::ArkUI_NativeDialogHandle;

pub(crate) struct InnerDialogDismissData {
    pub dismiss_handle: Option<fn(DialogDismissData) -> Option<bool>>,
    pub data: Option<*mut c_void>,
    pub(crate) data_owner: Option<Rc<dyn Any>>,
    pub(crate) callback: Option<Rc<dyn Fn(DismissReason) -> bool>>,
}

/// OnDismiss callback data
pub struct DialogDismissData {
    /// dismiss reason
    pub dismiss_reason: DismissReason,
    /// Borrowed custom data valid only during this callback.
    /// The callback must not take or free ownership of this pointer.
    pub data: Option<*mut c_void>,
}

/// High-level dialog wrapper for creating and controlling native dialogs.
pub struct Dialog {
    pub(crate) raw: ArkUI_NativeDialogHandle,
    pub(crate) inner_dismiss_data: Rc<RefCell<InnerDialogDismissData>>,
    content: RefCell<Option<DialogContent>>,
}

struct DialogContent {
    node: Rc<RefCell<ArkUINode>>,
    attached: bool,
    owned: bool,
}

impl Dialog {
    /// Create a new dialog controller.
    pub fn new() -> ArkUIResult<Self> {
        let dialog_controller = ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.create())?;
        let data = Rc::new(RefCell::new(InnerDialogDismissData {
            dismiss_handle: None,
            data: None,
            data_owner: None,
            callback: None,
        }));
        Ok(Dialog {
            raw: dialog_controller,
            inner_dismiss_data: data,
            content: RefCell::default(),
        })
    }

    fn register_dismiss(&self) -> ArkUIResult<()> {
        // Rc's allocation is stable even when the Dialog wrapper moves.
        let context = Rc::as_ptr(&self.inner_dismiss_data);
        ARK_UI_NATIVE_DIALOG_API_1
            .with(|api| api.register_dismiss(self.raw, context.cast_mut().cast()))
    }

    /// Retains a shared owned node until its native dialog edge is removed.
    /// Explicit node disposal is rejected while the dialog retains it.
    pub fn content_node(&self, node: &Rc<RefCell<ArkUINode>>) -> ArkUIResult<()> {
        self.set_content(node.clone(), false)
    }

    fn set_content(&self, node: Rc<RefCell<ArkUINode>>, owned: bool) -> ArkUIResult<()> {
        if self.raw.is_null() || self.content.borrow().is_some() {
            return Err(invalid(
                "remove existing content before attaching a new dialog node",
            ));
        }
        if !node.borrow().is_owned() || node.borrow().raw().is_null() {
            return Err(invalid("native retention requires an owned node"));
        }
        ArkUINode::bind_rc(&node);
        let raw = node.borrow().raw();
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.set_content(self.raw, raw))?;
        *self.content.borrow_mut() = Some(DialogContent {
            node,
            attached: true,
            owned,
        });
        Ok(())
    }

    /// Detaches the controller's content before its owner disposes the node.
    pub fn remove_content(&self) -> ArkUIResult<()> {
        let attached = self
            .content
            .borrow()
            .as_ref()
            .is_some_and(|content| content.attached);
        if attached {
            ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.remove_content(self.raw))?;
            if let Some(content) = self.content.borrow_mut().as_mut() {
                content.attached = false;
            }
        }
        {
            // Borrow the stored owner: cloning it here would itself block disposal.
            let content = self.content.borrow();
            if let Some(content) = content.as_ref().filter(|content| content.owned) {
                if content.node.borrow().is_owned() {
                    content.node.borrow_mut().dispose()?;
                }
            }
        }
        self.content.borrow_mut().take();
        Ok(())
    }

    /// Uses the content's exact size and decoration instead of a system card.
    pub fn custom_style(&self, enabled: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.custom_style(self.raw, enabled))
    }

    /// Disables system animations so the content can animate its own presentation.
    pub fn custom_animation(&self, enabled: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.custom_animation(self.raw, enabled))
    }

    /// Installs a UI-thread closure; true blocks dismissal synchronously.
    /// Panics are logged and allow dismissal.
    pub fn dismiss_handler(
        &self,
        handler: impl Fn(DismissReason) -> bool + 'static,
    ) -> ArkUIResult<()> {
        let mut data = self.inner_dismiss_data.borrow_mut();
        data.dismiss_handle = None;
        data.callback = Some(Rc::new(handler));
        data.data = None;
        data.data_owner = None;
        drop(data);
        self.register_dismiss()
    }

    /// Releases a closed dialog controller. Repeated calls are harmless.
    pub fn dispose(&mut self) -> ArkUIResult<()> {
        self.remove_content()?;
        if self.raw.is_null() {
            return Ok(());
        }
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.dispose(self.raw))?;
        self.raw = std::ptr::null_mut();
        Ok(())
    }

    /// Set dialog content node.
    pub fn content<T: Into<ArkUINode>>(&self, content: T) -> ArkUIResult<()> {
        let node = content.into().into_rc();
        if let Err(error) = self.set_content(node.clone(), true) {
            let cleanup = node.borrow_mut().dispose();
            if let Err(cleanup_error) = cleanup {
                // Native code may still own an edge after a failed mutation.
                log::error!("Dialog content cleanup failed; retaining node: {cleanup_error:?}");
                std::mem::forget(node);
            }
            return Err(error);
        }
        Ok(())
    }

    /// Show the dialog in current window.
    pub fn show(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.show(self.raw, false))?;
        Ok(())
    }

    /// Show the dialog in sub-window mode.
    pub fn show_with_sub_window(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.show(self.raw, true))?;
        Ok(())
    }

    /// Close the dialog.
    pub fn close(&self) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.close(self.raw))?;
        Ok(())
    }

    pub fn modal_mode(&self, modal_mode: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.set_modal_mode(self.raw, modal_mode))?;
        Ok(())
    }

    pub fn auto_cancel(&self, auto_cancel: bool) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.set_auto_cancel(self.raw, auto_cancel))?;
        Ok(())
    }

    pub fn background_color(&self, color: u32) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| api.set_background_color(self.raw, color))?;
        Ok(())
    }

    /// Set content alignment, offset_x and offset_y will be set with 0.0
    pub fn content_alignment(&self, alignment: Alignment) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1
            .with(|api| api.set_content_alignment(self.raw, alignment, 0.0, 0.0))?;
        Ok(())
    }

    /// Set content alignment with offset_x and offset_y.
    pub fn content_alignment_with_offset(
        &self,
        alignment: Alignment,
        offset_x: f32,
        offset_y: f32,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1
            .with(|api| api.set_content_alignment(self.raw, alignment, offset_x, offset_y))?;
        Ok(())
    }

    pub fn corner_radius(
        &self,
        top_left: f32,
        top_right: f32,
        bottom_left: f32,
        bottom_right: f32,
    ) -> ArkUIResult<()> {
        ARK_UI_NATIVE_DIALOG_API_1.with(|api| {
            api.set_corner_radius(self.raw, top_left, top_right, bottom_left, bottom_right)
        })?;
        Ok(())
    }

    pub fn on_will_dismiss(
        &self,
        callback: fn(DialogDismissData) -> Option<bool>,
    ) -> ArkUIResult<()> {
        self.inner_dismiss_data.borrow_mut().dismiss_handle = Some(callback);
        self.inner_dismiss_data.borrow_mut().callback = None;

        self.register_dismiss()?;
        Ok(())
    }

    /// For user custom data, you can use this method
    /// And you need to convert raw pointer to your data type after callback
    pub fn on_will_dismiss_with_data<T: 'static>(
        &self,
        data: T,
        callback: fn(data: DialogDismissData) -> Option<bool>,
    ) -> ArkUIResult<()> {
        self.inner_dismiss_data.borrow_mut().dismiss_handle = Some(callback);
        self.inner_dismiss_data.borrow_mut().callback = None;

        let owner = Rc::new(data);
        let pointer = Rc::as_ptr(&owner).cast_mut().cast::<c_void>();
        let mut inner = self.inner_dismiss_data.borrow_mut();
        inner.callback = None;
        inner.data = Some(pointer);
        inner.data_owner = Some(owner);
        drop(inner);

        self.register_dismiss()?;
        Ok(())
    }
}

impl Drop for Dialog {
    fn drop(&mut self) {
        if !self.raw.is_null() {
            if let Err(error) = self.close() {
                log::error!("Dialog Drop close failed: {error:?}");
            }
            if let Err(error) = self.dispose() {
                // The native controller can still reach its callback/content.
                // Retain the complete owners when native retirement fails.
                log::error!("Dialog Drop disposal failed; retaining native owners: {error:?}");
                std::mem::forget(self.inner_dismiss_data.clone());
                if let Some(content) = self.content.get_mut().take() {
                    std::mem::forget(content);
                }
            }
        }
    }
}

fn invalid(message: &str) -> crate::ArkUIError {
    crate::ArkUIError::new(
        crate::arkui_input_binding::ArkUIErrorCode::ParamInvalid,
        message,
    )
}
