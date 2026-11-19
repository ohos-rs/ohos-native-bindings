//! General-purpose ArkUI RenderNode, ContentModifier, and render property bindings (API 20).
//!
//! All types in this module are bound to the ArkUI UI thread and wrap the C APIs
//! declared in `arkui/native_render.h`.

use std::{
    ffi::c_void,
    marker::PhantomData,
    panic::{catch_unwind, AssertUnwindSafe},
    ptr::NonNull,
    rc::Rc,
};

use ohos_arkui_input_binding::ArkUIErrorCode;
use ohos_arkui_sys as sys;
#[cfg(feature = "drawing")]
use ohos_drawing_binding::Canvas;

pub use crate::api::node_custom_event::DrawContext;
use crate::{check_arkui_status, ArkUIError, ArkUINode, ArkUIResult};

fn init_error(message: &'static str) -> ArkUIError {
    ArkUIError::new(ArkUIErrorCode::ParamInvalid, message)
}

/// Owned ArkUI render node (`ArkUI_RenderNodeHandle`, API 20).
///
/// Must be created, mutated, attached, detached, and dropped on the owning UI thread.
pub struct RenderNode {
    raw: NonNull<sys::ArkUI_RenderNode>,
    _not_send: PhantomData<Rc<()>>,
}

impl RenderNode {
    /// Creates an empty render node (`OH_ArkUI_RenderNodeUtils_CreateNode`).
    pub fn new() -> ArkUIResult<Self> {
        let raw = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateNode() })
            .ok_or_else(|| init_error("could not create ArkUI render node"))?;
        Ok(Self {
            raw,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> sys::ArkUI_RenderNodeHandle {
        self.raw.as_ptr()
    }

    /// Attaches this render node to a custom ArkUI frame node (`OH_ArkUI_RenderNodeUtils_AddRenderNode`).
    pub fn attach_to(&self, parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AddRenderNode(parent.raw_handle(), self.raw.as_ptr())
        })
    }

    /// Detaches this render node from its parent ArkUI frame node (`OH_ArkUI_RenderNodeUtils_RemoveRenderNode`).
    pub fn detach_from(&self, parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_RemoveRenderNode(parent.raw_handle(), self.raw.as_ptr())
        })
    }

    /// Clears all child render nodes mounted directly on a custom ArkUI frame node.
    pub fn clear_frame_node_children(parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_ClearRenderNodeChildren(parent.raw_handle())
        })
    }

    /// Triggers re-rendering of render nodes mounted on `parent` (`OH_ArkUI_RenderNodeUtils_Invalidate`).
    pub fn invalidate_frame_node(parent: &ArkUINode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_Invalidate(parent.raw_handle())
        })
    }

    /// Appends a child render node (`OH_ArkUI_RenderNodeUtils_AddChild`).
    pub fn add_child(&self, child: &RenderNode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AddChild(self.raw.as_ptr(), child.raw.as_ptr())
        })
    }

    /// Inserts `child` after `sibling` (or at the front when `sibling` is `None`).
    pub fn insert_child_after(
        &self,
        child: &RenderNode,
        sibling: Option<&RenderNode>,
    ) -> ArkUIResult<()> {
        let sibling_ptr = sibling.map_or(std::ptr::null_mut(), |node| node.raw.as_ptr());
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_InsertChildAfter(
                self.raw.as_ptr(),
                child.raw.as_ptr(),
                sibling_ptr,
            )
        })
    }

    /// Removes `child` from this render node (`OH_ArkUI_RenderNodeUtils_RemoveChild`).
    pub fn remove_child(&self, child: &RenderNode) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_RemoveChild(self.raw.as_ptr(), child.raw.as_ptr())
        })
    }

    /// Removes all child render nodes (`OH_ArkUI_RenderNodeUtils_ClearChildren`).
    pub fn clear_children(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_ClearChildren(self.raw.as_ptr())
        })
    }

    /// Returns the number of child render nodes.
    pub fn children_count(&self) -> ArkUIResult<i32> {
        let mut count = 0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetChildrenCount(self.raw.as_ptr(), &mut count)
        })?;
        Ok(count)
    }

    /// Sets the position in physical pixels (`OH_ArkUI_RenderNodeUtils_SetPosition`).
    pub fn set_position(&self, x: i32, y: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetPosition(self.raw.as_ptr(), x, y)
        })
    }

    /// Gets the position in physical pixels.
    pub fn position(&self) -> ArkUIResult<(i32, i32)> {
        let (mut x, mut y) = (0, 0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetPosition(self.raw.as_ptr(), &mut x, &mut y)
        })?;
        Ok((x, y))
    }

    /// Sets the size in physical pixels (`OH_ArkUI_RenderNodeUtils_SetSize`).
    pub fn set_size(&self, width: i32, height: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetSize(self.raw.as_ptr(), width, height)
        })
    }

    /// Gets the size in physical pixels.
    pub fn size(&self) -> ArkUIResult<(i32, i32)> {
        let (mut width, mut height) = (0, 0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetSize(self.raw.as_ptr(), &mut width, &mut height)
        })?;
        Ok((width, height))
    }

    /// Sets the ARGB background color (`OH_ArkUI_RenderNodeUtils_SetBackgroundColor`).
    pub fn set_background_color(&self, color: u32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetBackgroundColor(self.raw.as_ptr(), color)
        })
    }

    /// Gets the ARGB background color.
    pub fn background_color(&self) -> ArkUIResult<u32> {
        let mut color = 0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetBackgroundColor(self.raw.as_ptr(), &mut color)
        })?;
        Ok(color)
    }

    /// Sets whether content is clipped to the node frame (`OH_ArkUI_RenderNodeUtils_SetClipToFrame`).
    pub fn set_clip_to_frame(&self, clip: bool) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetClipToFrame(self.raw.as_ptr(), i32::from(clip))
        })
    }

    /// Gets whether content is clipped to the node frame.
    pub fn clip_to_frame(&self) -> ArkUIResult<bool> {
        let mut clip = 0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetClipToFrame(self.raw.as_ptr(), &mut clip)
        })?;
        Ok(clip != 0)
    }

    /// Sets whether children are clipped to the node bounds (`OH_ArkUI_RenderNodeUtils_SetClipToBounds`).
    pub fn set_clip_to_bounds(&self, clip: bool) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetClipToBounds(self.raw.as_ptr(), i32::from(clip))
        })
    }

    /// Gets whether children are clipped to the node bounds.
    pub fn clip_to_bounds(&self) -> ArkUIResult<bool> {
        let mut clip = 0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetClipToBounds(self.raw.as_ptr(), &mut clip)
        })?;
        Ok(clip != 0)
    }

    /// Sets the node opacity in `[0.0, 1.0]` (`OH_ArkUI_RenderNodeUtils_SetOpacity`).
    pub fn set_opacity(&self, opacity: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetOpacity(self.raw.as_ptr(), opacity)
        })
    }

    /// Gets the node opacity.
    pub fn opacity(&self) -> ArkUIResult<f32> {
        let mut opacity = 0.0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetOpacity(self.raw.as_ptr(), &mut opacity)
        })?;
        Ok(opacity)
    }

    /// Sets the transform pivot in normalized coordinates (`[0.0, 1.0]`).
    pub fn set_pivot(&self, x: f32, y: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetPivot(self.raw.as_ptr(), x, y)
        })
    }

    /// Gets the transform pivot.
    pub fn pivot(&self) -> ArkUIResult<(f32, f32)> {
        let (mut x, mut y) = (0.0, 0.0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetPivot(self.raw.as_ptr(), &mut x, &mut y)
        })?;
        Ok((x, y))
    }

    /// Sets the scale factors.
    pub fn set_scale(&self, x: f32, y: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetScale(self.raw.as_ptr(), x, y)
        })
    }

    /// Gets the scale factors.
    pub fn scale(&self) -> ArkUIResult<(f32, f32)> {
        let (mut x, mut y) = (0.0, 0.0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetScale(self.raw.as_ptr(), &mut x, &mut y)
        })?;
        Ok((x, y))
    }

    /// Sets the translation offset in physical pixels.
    pub fn set_translation(&self, x: f32, y: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetTranslation(self.raw.as_ptr(), x, y)
        })
    }

    /// Gets the translation offset in physical pixels.
    pub fn translation(&self) -> ArkUIResult<(f32, f32)> {
        let (mut x, mut y) = (0.0, 0.0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetTranslation(self.raw.as_ptr(), &mut x, &mut y)
        })?;
        Ok((x, y))
    }

    /// Sets the 3D rotation angles in degrees.
    pub fn set_rotation(&self, x: f32, y: f32, z: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetRotation(self.raw.as_ptr(), x, y, z)
        })
    }

    /// Gets the 3D rotation angles in degrees.
    pub fn rotation(&self) -> ArkUIResult<(f32, f32, f32)> {
        let (mut x, mut y, mut z) = (0.0, 0.0, 0.0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetRotation(self.raw.as_ptr(), &mut x, &mut y, &mut z)
        })?;
        Ok((x, y, z))
    }

    /// Sets the 4x4 transformation matrix (`OH_ArkUI_RenderNodeUtils_SetTransform`).
    pub fn set_transform(&self, matrix: &[f32; 16]) -> ArkUIResult<()> {
        let mut copy = *matrix;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetTransform(self.raw.as_ptr(), copy.as_mut_ptr())
        })
    }

    /// Sets the shadow color.
    pub fn set_shadow_color(&self, color: u32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetShadowColor(self.raw.as_ptr(), color)
        })
    }

    /// Sets the shadow offset in physical pixels.
    pub fn set_shadow_offset(&self, x: i32, y: i32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetShadowOffset(self.raw.as_ptr(), x, y)
        })
    }

    /// Sets the shadow alpha in `[0.0, 1.0]`.
    pub fn set_shadow_alpha(&self, alpha: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetShadowAlpha(self.raw.as_ptr(), alpha)
        })
    }

    /// Sets the shadow elevation.
    pub fn set_shadow_elevation(&self, elevation: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetShadowElevation(self.raw.as_ptr(), elevation)
        })
    }

    /// Sets the shadow blur radius.
    pub fn set_shadow_radius(&self, radius: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetShadowRadius(self.raw.as_ptr(), radius)
        })
    }

    /// Attaches a content modifier to this render node (`OH_ArkUI_RenderNodeUtils_AttachContentModifier`).
    ///
    /// Keep `modifier` alive for as long as its draw callback or attached properties are needed.
    pub fn attach_content_modifier(&self, modifier: &RenderContentModifier) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AttachContentModifier(
                self.raw.as_ptr(),
                modifier.raw.as_ptr(),
            )
        })
    }

    /// Sets a background blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn set_background_blur_option(&self, option: &RenderBlurStyleOption) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetBackgroundBlurOption(
                self.raw.as_ptr(),
                option.raw.as_ptr(),
            )
        })
    }

    /// Resets the background blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn reset_background_blur_option(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_ResetBackgroundBlurOption(self.raw.as_ptr())
        })
    }

    /// Sets a foreground blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn set_foreground_blur_option(&self, option: &RenderBlurStyleOption) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetForegroundBlurOption(
                self.raw.as_ptr(),
                option.raw.as_ptr(),
            )
        })
    }

    /// Resets the foreground blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn reset_foreground_blur_option(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_ResetForegroundBlurOption(self.raw.as_ptr())
        })
    }

    /// Sets a content blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn set_content_blur_option(&self, option: &RenderBlurStyleOption) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetContentBlurOption(
                self.raw.as_ptr(),
                option.raw.as_ptr(),
            )
        })
    }

    /// Resets the content blur style option (API 26).
    #[cfg(feature = "api-26")]
    pub fn reset_content_blur_option(&self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_ResetContentBlurOption(self.raw.as_ptr())
        })
    }
}

impl Drop for RenderNode {
    fn drop(&mut self) {
        unsafe {
            let _ = sys::OH_ArkUI_RenderNodeUtils_DisposeNode(self.raw.as_ptr());
        }
    }
}

struct DrawHandler(Box<dyn Fn(DrawContext)>);

unsafe extern "C" fn on_draw_trampoline(context: *mut sys::ArkUI_DrawContext, data: *mut c_void) {
    let (Some(context), Some(handler)) = (DrawContext::from_raw(context), unsafe {
        data.cast::<DrawHandler>().as_ref()
    }) else {
        return;
    };
    if catch_unwind(AssertUnwindSafe(|| (handler.0)(context))).is_err() {
        log::error!("ArkUI render content modifier draw callback panicked");
    }
}

unsafe extern "C" fn detached_on_draw(_: *mut sys::ArkUI_DrawContext, _: *mut c_void) {}

/// Owned render content modifier (`ArkUI_RenderContentModifierHandle`, API 20).
///
/// Detaches its draw callback before disposing the native handle on drop.
pub struct RenderContentModifier {
    raw: NonNull<sys::ArkUI_RenderContentModifier>,
    handler: Option<NonNull<DrawHandler>>,
    _not_send: PhantomData<Rc<()>>,
}

impl RenderContentModifier {
    /// Creates a content modifier (`OH_ArkUI_RenderNodeUtils_CreateContentModifier`).
    pub fn new() -> ArkUIResult<Self> {
        let raw = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateContentModifier() })
            .ok_or_else(|| init_error("could not create ArkUI render content modifier"))?;
        Ok(Self {
            raw,
            handler: None,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> sys::ArkUI_RenderContentModifierHandle {
        self.raw.as_ptr()
    }

    /// Registers a UI-thread draw callback (`OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw`).
    pub fn set_on_draw(&mut self, callback: impl Fn(DrawContext) + 'static) -> ArkUIResult<()> {
        let next = NonNull::from(Box::leak(Box::new(DrawHandler(Box::new(callback)))));
        let result = check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw(
                self.raw.as_ptr(),
                next.as_ptr().cast(),
                Some(on_draw_trampoline),
            )
        });
        if let Err(error) = result {
            // SAFETY: registration failed so ArkUI does not retain `next`.
            unsafe { drop(Box::from_raw(next.as_ptr())) };
            return Err(error);
        }
        // Replace only after native registration succeeds so the previous handler stays
        // valid if registration fails.
        if let Some(previous) = self.handler.replace(next) {
            // SAFETY: ArkUI now points to `next`, so `previous` is no longer referenced.
            unsafe { drop(Box::from_raw(previous.as_ptr())) };
        }
        Ok(())
    }

    /// Registers a UI-thread draw callback that receives a borrowed [`Canvas`] for each draw pass.
    #[cfg(feature = "drawing")]
    pub fn set_on_draw_canvas(&mut self, paint: impl Fn(&Canvas) + 'static) -> ArkUIResult<()> {
        self.set_on_draw(move |context| {
            if let Some(raw) = context.canvas() {
                // SAFETY: ArkUI lends its canvas for the duration of this draw callback only.
                let canvas = unsafe { Canvas::from_raw_borrowed(raw) };
                paint(&canvas);
            }
        })
    }

    /// Clears the registered draw callback so subsequent native draws become a no-op.
    pub fn clear_on_draw(&mut self) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw(
                self.raw.as_ptr(),
                std::ptr::null_mut(),
                Some(detached_on_draw),
            )
        })?;
        if let Some(previous) = self.handler.take() {
            // SAFETY: native callback was detached above on the owning UI thread.
            unsafe { drop(Box::from_raw(previous.as_ptr())) };
        }
        Ok(())
    }

    /// Attaches a float property (`OH_ArkUI_RenderNodeUtils_AttachFloatProperty`).
    pub fn attach_float_property(&self, property: &FloatProperty) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AttachFloatProperty(
                self.raw.as_ptr(),
                property.raw.as_ptr(),
            )
        })
    }

    /// Attaches a 2D vector property (`OH_ArkUI_RenderNodeUtils_AttachVector2Property`).
    pub fn attach_vector2_property(&self, property: &Vector2Property) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AttachVector2Property(
                self.raw.as_ptr(),
                property.raw.as_ptr(),
            )
        })
    }

    /// Attaches a color property (`OH_ArkUI_RenderNodeUtils_AttachColorProperty`).
    pub fn attach_color_property(&self, property: &ColorProperty) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_AttachColorProperty(
                self.raw.as_ptr(),
                property.raw.as_ptr(),
            )
        })
    }
}

impl Drop for RenderContentModifier {
    fn drop(&mut self) {
        let handler = self.handler.take();
        unsafe {
            if handler.is_some() {
                let _ = sys::OH_ArkUI_RenderNodeUtils_SetContentModifierOnDraw(
                    self.raw.as_ptr(),
                    std::ptr::null_mut(),
                    Some(detached_on_draw),
                );
            }
            sys::OH_ArkUI_RenderNodeUtils_DisposeContentModifier(self.raw.as_ptr());
            if let Some(previous) = handler {
                drop(Box::from_raw(previous.as_ptr()));
            }
        }
    }
}

/// Owned float render property (`ArkUI_FloatPropertyHandle`, API 20).
pub struct FloatProperty {
    raw: NonNull<sys::ArkUI_FloatProperty>,
    _not_send: PhantomData<Rc<()>>,
}

impl FloatProperty {
    /// Creates a float property initialized to `value`.
    pub fn new(value: f32) -> ArkUIResult<Self> {
        let raw = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateFloatProperty(value) })
            .ok_or_else(|| init_error("could not create ArkUI float property"))?;
        Ok(Self {
            raw,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> sys::ArkUI_FloatPropertyHandle {
        self.raw.as_ptr()
    }

    /// Updates the property value (`OH_ArkUI_RenderNodeUtils_SetFloatPropertyValue`).
    pub fn set(&self, value: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetFloatPropertyValue(self.raw.as_ptr(), value)
        })
    }

    /// Reads the current property value (`OH_ArkUI_RenderNodeUtils_GetFloatPropertyValue`).
    pub fn get(&self) -> ArkUIResult<f32> {
        let mut value = 0.0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetFloatPropertyValue(self.raw.as_ptr(), &mut value)
        })?;
        Ok(value)
    }
}

impl Drop for FloatProperty {
    fn drop(&mut self) {
        unsafe { sys::OH_ArkUI_RenderNodeUtils_DisposeFloatProperty(self.raw.as_ptr()) };
    }
}

/// Owned 2D vector render property (`ArkUI_Vector2PropertyHandle`, API 20).
pub struct Vector2Property {
    raw: NonNull<sys::ArkUI_Vector2Property>,
    _not_send: PhantomData<Rc<()>>,
}

impl Vector2Property {
    /// Creates a 2D vector property initialized to `(x, y)`.
    pub fn new(x: f32, y: f32) -> ArkUIResult<Self> {
        let raw =
            NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateVector2Property(x, y) })
                .ok_or_else(|| init_error("could not create ArkUI vector2 property"))?;
        Ok(Self {
            raw,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> sys::ArkUI_Vector2PropertyHandle {
        self.raw.as_ptr()
    }

    /// Updates the property value.
    pub fn set(&self, x: f32, y: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetVector2PropertyValue(self.raw.as_ptr(), x, y)
        })
    }

    /// Reads the current property value.
    pub fn get(&self) -> ArkUIResult<(f32, f32)> {
        let (mut x, mut y) = (0.0, 0.0);
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetVector2PropertyValue(self.raw.as_ptr(), &mut x, &mut y)
        })?;
        Ok((x, y))
    }
}

impl Drop for Vector2Property {
    fn drop(&mut self) {
        unsafe { sys::OH_ArkUI_RenderNodeUtils_DisposeVector2Property(self.raw.as_ptr()) };
    }
}

/// Owned color render property (`ArkUI_ColorPropertyHandle`, API 20).
pub struct ColorProperty {
    raw: NonNull<sys::ArkUI_ColorProperty>,
    _not_send: PhantomData<Rc<()>>,
}

impl ColorProperty {
    /// Creates a color property initialized to ARGB `value`.
    pub fn new(value: u32) -> ArkUIResult<Self> {
        let raw = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateColorProperty(value) })
            .ok_or_else(|| init_error("could not create ArkUI color property"))?;
        Ok(Self {
            raw,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> sys::ArkUI_ColorPropertyHandle {
        self.raw.as_ptr()
    }

    /// Updates the property value.
    pub fn set(&self, value: u32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetColorPropertyValue(self.raw.as_ptr(), value)
        })
    }

    /// Reads the current property value.
    pub fn get(&self) -> ArkUIResult<u32> {
        let mut value = 0;
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_GetColorPropertyValue(self.raw.as_ptr(), &mut value)
        })?;
        Ok(value)
    }
}

impl Drop for ColorProperty {
    fn drop(&mut self) {
        unsafe { sys::OH_ArkUI_RenderNodeUtils_DisposeColorProperty(self.raw.as_ptr()) };
    }
}

/// Owned blur style option for [`RenderNode`] blur attributes (`ArkUI_RenderBlurStyleOption`, API 26).
#[cfg(feature = "api-26")]
pub struct RenderBlurStyleOption {
    raw: NonNull<sys::ArkUI_RenderBlurStyleOption>,
    _not_send: PhantomData<Rc<()>>,
}

#[cfg(feature = "api-26")]
impl RenderBlurStyleOption {
    /// Creates a blur style option (`OH_ArkUI_RenderNodeUtils_CreateBlurStyleOption`).
    pub fn new() -> ArkUIResult<Self> {
        let raw = NonNull::new(unsafe { sys::OH_ArkUI_RenderNodeUtils_CreateBlurStyleOption() })
            .ok_or_else(|| init_error("could not create ArkUI render blur style option"))?;
        Ok(Self {
            raw,
            _not_send: PhantomData,
        })
    }

    /// Returns the underlying native handle.
    pub fn raw(&self) -> *mut sys::ArkUI_RenderBlurStyleOption {
        self.raw.as_ptr()
    }

    /// Sets the blur radius (`OH_ArkUI_RenderNodeUtils_SetBlurStyleOptionRadius`).
    pub fn set_radius(&self, radius: f32) -> ArkUIResult<()> {
        check_arkui_status!(unsafe {
            sys::OH_ArkUI_RenderNodeUtils_SetBlurStyleOptionRadius(self.raw.as_ptr(), radius)
        })
    }
}

#[cfg(feature = "api-26")]
impl Drop for RenderBlurStyleOption {
    fn drop(&mut self) {
        unsafe { sys::OH_ArkUI_RenderNodeUtils_DisposeBlurStyleOption(self.raw.as_ptr()) };
    }
}
