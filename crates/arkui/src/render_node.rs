//! An API-20 drawing render node with a retained content modifier.
use std::{
    ffi::c_void,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
};

use crate::{check_arkui_status, ArkUIError, ArkUINode, ArkUIResult};
use ohos_arkui_sys as sys;
use ohos_drawing_binding::Canvas;

struct Drawing(Box<dyn Fn(&Canvas)>);

unsafe extern "C" fn draw(context: *mut sys::ArkUI_DrawContext, data: *mut c_void) {
    let (Some(context), Some(drawing)) = (NonNull::new(context), unsafe {
        data.cast::<Drawing>().as_ref()
    }) else {
        return;
    };
    if let Some(raw) =
        NonNull::new(unsafe { sys::OH_ArkUI_DrawContext_GetCanvas(context.as_ptr()) })
    {
        // ArkUI lends its canvas for this callback only. A user panic may not cross C.
        let canvas = unsafe { Canvas::from_raw_borrowed(raw.cast()) };
        if catch_unwind(AssertUnwindSafe(|| (drawing.0)(&canvas))).is_err() {
            log::error!("ArkUI render node draw callback panicked");
        }
    }
}
unsafe extern "C" fn detached(_: *mut sys::ArkUI_DrawContext, _: *mut c_void) {}

/// Owns its native node, modifier, invalidation property and draw callback.
/// Attach and detach on the UI thread. The frame node owns the position in the UI tree.
pub struct DrawingRenderNode {
    node: NonNull<sys::ArkUI_RenderNode>,
    modifier: Option<NonNull<sys::ArkUI_RenderContentModifier>>,
    property: Option<NonNull<sys::ArkUI_FloatProperty>>,
    drawing: Box<Drawing>,
    revision: std::cell::Cell<bool>,
}

impl DrawingRenderNode {
    /// Creates an unclipped render node whose callback records into the SDK canvas.
    pub fn new(paint: impl Fn(&Canvas) + 'static) -> ArkUIResult<Self> {
        let node = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateNode() }).ok_or_else(
            || {
                ArkUIError::new(
                    ohos_arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                    "could not create ArkUI render node",
                )
            },
        )?;
        let mut layer = Self {
            node,
            modifier: None,
            property: None,
            drawing: Box::new(Drawing(Box::new(paint))),
            revision: Default::default(),
        };
        let modifier =
            NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateContentModifier() })
                .ok_or_else(|| {
                    ArkUIError::new(
                        ohos_arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                        "could not create ArkUI content modifier",
                    )
                })?;
        layer.modifier = Some(modifier);
        let property =
            NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateFloatProperty(0.0) })
                .ok_or_else(|| {
                    ArkUIError::new(
                        ohos_arkui_input_binding::ArkUIErrorCode::ParamInvalid,
                        "could not create ArkUI drawing property",
                    )
                })?;
        layer.property = Some(property);
        unsafe {
            check_arkui_status!(sys::OH_ArkUI_RenderNodeUtils_SetClipToFrame(
                node.as_ptr(),
                0
            ))?;
            check_arkui_status!(sys::OH_ArkUI_RenderNodeUtils_SetClipToBounds(
                node.as_ptr(),
                0
            ))?;
            check_arkui_status!(sys::OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw(
                modifier.as_ptr(),
                (&mut *layer.drawing as *mut Drawing).cast(),
                Some(draw)
            ))?;
            check_arkui_status!(sys::OH_ArkUI_RenderNodeUtils_AttachContentModifier(
                node.as_ptr(),
                modifier.as_ptr()
            ))?;
            // Properties acquire their target from the modifier's attached node.
            check_arkui_status!(sys::OH_ArkUI_RenderNodeUtils_AttachFloatProperty(
                modifier.as_ptr(),
                property.as_ptr()
            ))?;
        }
        Ok(layer)
    }
    /// Adds this node to an ArkUI frame node.
    pub fn attach(&self, parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AddRenderNode(parent.raw_handle(), self.node.as_ptr())
        })
    }
    /// Removes this node from an ArkUI frame node.
    pub fn detach(&self, parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_RemoveRenderNode(parent.raw_handle(), self.node.as_ptr())
        })
    }
    /// Sets the origin in physical pixels.
    pub fn position(&self, x: i32, y: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetPosition(self.node.as_ptr(), x, y)
        })
    }
    /// Sets the size in physical pixels.
    pub fn size(&self, width: i32, height: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetSize(self.node.as_ptr(), width, height)
        })
    }
    /// Marks the retained content for a new drawing.
    pub fn invalidate(&self) -> ArkUIResult<()> {
        let property = self.property.expect("fully initialized render node");
        let next = !self.revision.get();
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetFloatPropertyValue(
                property.as_ptr(),
                if next { 1.0 } else { 0.0 },
            )
        })?;
        self.revision.set(next);
        Ok(())
    }
}

impl Drop for DrawingRenderNode {
    fn drop(&mut self) {
        // The drawing closure stays alive through unregistration and disposal.
        unsafe {
            if let Some(m) = self.modifier {
                sys::OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw(
                    m.as_ptr(),
                    std::ptr::null_mut(),
                    Some(detached),
                );
            }
            sys::OH_ArkUI_RenderNodeUtils_DisposeNode(self.node.as_ptr());
            if let Some(m) = self.modifier {
                sys::OH_ArkUI_RenderNodeUtils_DisposeContentModifier(m.as_ptr());
            }
            if let Some(p) = self.property {
                sys::OH_ArkUI_RenderNodeUtils_DisposeFloatProperty(p.as_ptr());
            }
        }
    }
}
