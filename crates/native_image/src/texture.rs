//! NativeImage texture consumption through statically linked community EGL/GLES bindings.
use std::{marker::PhantomData, ptr::NonNull, rc::Rc};

use glow::HasContext;

// GL_OES_EGL_image_external is accepted by glow but has no exported enum.
const TEXTURE_EXTERNAL_OES: u32 = 0x8d65;
use khronos_egl as egl;

use crate::{check, validate_size, ConsumerSurface, Error, RgbaFrame};

fn failed(operation: &'static str) -> Error {
    Error {
        operation,
        code: -1,
    }
}
fn egl_error(error: egl::Error) -> Error {
    Error {
        operation: "EGL",
        code: error as i32,
    }
}

struct Context {
    egl: egl::Instance<egl::Static>,
    display: egl::Display,
    context: egl::Context,
    surface: egl::Surface,
    // EGL contexts must remain on their creating thread, even if glow is Send.
    _thread: PhantomData<Rc<()>>,
}
struct Current<'a> {
    context: &'a Context,
    previous: (
        Option<egl::Display>,
        Option<egl::Context>,
        Option<egl::Surface>,
        Option<egl::Surface>,
    ),
    previous_api: egl::Enum,
}
struct ClientApi<'a> {
    egl: &'a egl::Instance<egl::Static>,
    previous: egl::Enum,
    active: bool,
}
impl ClientApi<'_> {
    fn restore(&mut self) -> Result<(), Error> {
        self.egl.bind_api(self.previous).map_err(egl_error)?;
        self.active = false;
        Ok(())
    }
}
impl Drop for ClientApi<'_> {
    fn drop(&mut self) {
        if self.active {
            let _ = self.egl.bind_api(self.previous);
        }
    }
}
impl Drop for Current<'_> {
    fn drop(&mut self) {
        let (display, context, draw, read) = self.previous;
        let _ = self.context.egl.make_current(
            display.unwrap_or(self.context.display),
            draw,
            read,
            context,
        );
        let _ = self.context.egl.bind_api(self.previous_api);
    }
}
impl Context {
    fn new() -> Result<Self, Error> {
        let egl = egl::Instance::new(egl::Static);
        let previous = egl.query_api();
        egl.bind_api(egl::OPENGL_ES_API).map_err(egl_error)?;
        let mut client_api = ClientApi {
            egl: &egl,
            previous,
            active: true,
        };
        // SAFETY: EGL_DEFAULT_DISPLAY is the platform's null display sentinel.
        let display =
            unsafe { egl.get_display(egl::DEFAULT_DISPLAY) }.ok_or(failed("EGL display"))?;
        egl.initialize(display).map_err(egl_error)?;
        let config = egl
            .choose_first_config(
                display,
                &[
                    egl::SURFACE_TYPE,
                    egl::PBUFFER_BIT,
                    egl::RENDERABLE_TYPE,
                    egl::OPENGL_ES2_BIT,
                    egl::RED_SIZE,
                    8,
                    egl::GREEN_SIZE,
                    8,
                    egl::BLUE_SIZE,
                    8,
                    egl::ALPHA_SIZE,
                    8,
                    egl::NONE,
                ],
            )
            .map_err(egl_error)?
            .ok_or(failed("EGL RGBA config"))?;
        let context = egl
            .create_context(
                display,
                config,
                None,
                &[egl::CONTEXT_CLIENT_VERSION, 2, egl::NONE],
            )
            .map_err(egl_error)?;
        let surface = match egl.create_pbuffer_surface(
            display,
            config,
            &[egl::WIDTH, 1, egl::HEIGHT, 1, egl::NONE],
        ) {
            Ok(surface) => surface,
            Err(error) => {
                let _ = egl.destroy_context(display, context);
                return Err(egl_error(error));
            }
        };
        if let Err(error) = client_api.restore() {
            let _ = egl.destroy_surface(display, surface);
            let _ = egl.destroy_context(display, context);
            return Err(error);
        }
        drop(client_api);
        Ok(Self {
            egl,
            display,
            context,
            surface,
            _thread: PhantomData,
        })
    }
    fn enter(&self) -> Result<Current<'_>, Error> {
        let previous_api = self.egl.query_api();
        self.egl.bind_api(egl::OPENGL_ES_API).map_err(egl_error)?;
        let mut client_api = ClientApi {
            egl: &self.egl,
            previous: previous_api,
            active: true,
        };
        // EGL current state is per client API. Preserve the ES context before
        // replacing it, then restore the caller's selected API on exit.
        let previous = (
            self.egl.get_current_display(),
            self.egl.get_current_context(),
            self.egl.get_current_surface(egl::DRAW),
            self.egl.get_current_surface(egl::READ),
        );
        self.egl
            .make_current(
                self.display,
                Some(self.surface),
                Some(self.surface),
                Some(self.context),
            )
            .map_err(egl_error)?;
        client_api.active = false;
        Ok(Current {
            context: self,
            previous,
            previous_api,
        })
    }
}
impl Drop for Context {
    fn drop(&mut self) {
        let _ = self.egl.destroy_surface(self.display, self.surface);
        let _ = self.egl.destroy_context(self.display, self.context);
        // The default EGL display is shared with the embedding app. Terminating
        // it here would invalidate that app's independent rendering contexts.
    }
}

/// A thread-bound offscreen context shared by NativeImage readers on one worker.
///
/// ```compile_fail
/// fn needs_send<T: Send>() {}
/// needs_send::<ohos_native_image_binding::TextureContext>();
/// ```
pub struct TextureContext {
    context: Rc<Context>,
    gl: Rc<glow::Context>,
    max_texture_size: u32,
}
impl TextureContext {
    /// Creates an offscreen GLES context on the calling rendering worker.
    pub fn new() -> Result<Self, Error> {
        let context = Rc::new(Context::new()?);
        let current = context.enter()?;
        // SAFETY: the owned context is current. EGL supplies standard driver
        // entry points for glow; native API availability remains feature-gated.
        let gl = unsafe {
            glow::Context::from_loader_function(|name| {
                context
                    .egl
                    .get_proc_address(name)
                    .map_or(std::ptr::null(), |function| function as *const _)
            })
        };
        // SAFETY: the query runs in the current, owned GLES context.
        let max_texture_size = unsafe { gl.get_parameter_i32(glow::MAX_TEXTURE_SIZE) };
        let max_texture_size = u32::try_from(max_texture_size)
            .ok()
            .filter(|limit| *limit > 0)
            .ok_or(failed("GLES texture limit"))?;
        drop(current);
        Ok(Self {
            context,
            gl: Rc::new(gl),
            max_texture_size,
        })
    }
}
struct Objects {
    context: Rc<Context>,
    gl: Rc<glow::Context>,
    texture: Option<glow::Texture>,
    target: Option<glow::Texture>,
    framebuffer: Option<glow::Framebuffer>,
    program: Option<glow::Program>,
    vertices: Option<glow::Buffer>,
}
impl Drop for Objects {
    fn drop(&mut self) {
        if self.texture.is_none() {
            return;
        }
        if let Ok(_current) = self.context.enter() {
            // SAFETY: partial construction retains each allocated object in its creating context.
            unsafe {
                if let Some(object) = self.vertices {
                    self.gl.delete_buffer(object);
                }
                if let Some(object) = self.program {
                    self.gl.delete_program(object);
                }
                if let Some(object) = self.framebuffer {
                    self.gl.delete_framebuffer(object);
                }
                if let Some(object) = self.target {
                    self.gl.delete_texture(object);
                }
                if let Some(object) = self.texture {
                    self.gl.delete_texture(object);
                }
            }
        }
    }
}
/// A thread-bound NativeImage reader with an offscreen GLES context.
///
/// GPU sampling resolves the producer's texture layout and transform before
/// copying tightly packed RGBA pixels. This supports GPU-only buffer layouts
/// for which a CPU NativeBuffer mapping does not expose linear image pixels.
/// Create and use this owner on a rendering worker, never a UI callback.
pub struct TextureReader {
    surface: Option<ConsumerSurface>,
    gl: Rc<glow::Context>,
    context: Rc<Context>,
    texture: glow::Texture,
    target: glow::Texture,
    framebuffer: glow::Framebuffer,
    program: glow::Program,
    vertices: glow::Buffer,
    size: (u32, u32),
    allocated: (u32, u32),
    max_texture_size: u32,
}
impl TextureReader {
    /// Creates a live consumer. Its callback must only enqueue a worker wake,
    /// and must not call NativeImage interfaces or retire consumers.
    pub fn new(callback: impl Fn() + Send + Sync + 'static) -> Result<Self, Error> {
        Self::with_context(&TextureContext::new()?, callback)
    }
    /// Creates another consumer on the same rendering worker and GLES context.
    pub fn with_context(
        gpu: &TextureContext,
        callback: impl Fn() + Send + Sync + 'static,
    ) -> Result<Self, Error> {
        let context = gpu.context.clone();
        let current = context.enter()?;
        let gl = gpu.gl.clone();
        let mut objects = Objects {
            context: context.clone(),
            gl: gl.clone(),
            texture: None,
            target: None,
            framebuffer: None,
            program: None,
            vertices: None,
        };
        // SAFETY: all objects belong to the current owned context. The construction
        // guard deletes every allocated object if any later step fails.
        let (texture, target, framebuffer, program, vertices) = unsafe {
            let texture = gl
                .create_texture()
                .map_err(|_| failed("external texture"))?;
            objects.texture = Some(texture);
            gl.bind_texture(TEXTURE_EXTERNAL_OES, Some(texture));
            for parameter in [glow::TEXTURE_MIN_FILTER, glow::TEXTURE_MAG_FILTER] {
                gl.tex_parameter_i32(TEXTURE_EXTERNAL_OES, parameter, glow::LINEAR as i32);
            }
            for parameter in [glow::TEXTURE_WRAP_S, glow::TEXTURE_WRAP_T] {
                gl.tex_parameter_i32(TEXTURE_EXTERNAL_OES, parameter, glow::CLAMP_TO_EDGE as i32);
            }
            let target = gl
                .create_texture()
                .map_err(|_| failed("readback texture"))?;
            objects.target = Some(target);
            let framebuffer = gl
                .create_framebuffer()
                .map_err(|_| failed("readback framebuffer"))?;
            objects.framebuffer = Some(framebuffer);
            let program = gl
                .create_program()
                .map_err(|_| failed("readback program"))?;
            objects.program = Some(program);
            for (kind, source) in [
                (glow::VERTEX_SHADER, "attribute vec2 position; uniform mat4 transform; varying vec2 uv; void main() { gl_Position = vec4(position, 0.0, 1.0); uv = (transform * vec4(position * 0.5 + 0.5, 0.0, 1.0)).xy; }"),
                (glow::FRAGMENT_SHADER, "#extension GL_OES_EGL_image_external : require\nprecision mediump float; varying vec2 uv; uniform samplerExternalOES image; void main() { gl_FragColor = texture2D(image, uv); }"),
            ] {
                let shader = gl.create_shader(kind).map_err(|_| failed("readback shader"))?;
                gl.shader_source(shader, source); gl.compile_shader(shader);
                let compiled = gl.get_shader_compile_status(shader);
                if compiled { gl.attach_shader(program, shader); }
                gl.delete_shader(shader);
                if !compiled { return Err(failed("compile readback shader")); }
            }
            gl.bind_attrib_location(program, 0, "position");
            gl.link_program(program);
            if !gl.get_program_link_status(program) {
                return Err(failed("link readback program"));
            }
            let vertices = gl
                .create_buffer()
                .map_err(|_| failed("readback vertices"))?;
            objects.vertices = Some(vertices);
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(vertices));
            let points: [f32; 8] = [-1.0, -1.0, 1.0, -1.0, -1.0, 1.0, 1.0, 1.0];
            let bytes =
                std::slice::from_raw_parts(points.as_ptr().cast(), std::mem::size_of_val(&points));
            gl.buffer_data_u8_slice(glow::ARRAY_BUFFER, bytes, glow::STATIC_DRAW);
            (texture, target, framebuffer, program, vertices)
        };
        // SAFETY: the external texture is live in the current owned context.
        let raw = NonNull::new(unsafe {
            ohos_native_image_sys::OH_NativeImage_Create(texture.0.get(), TEXTURE_EXTERNAL_OES)
        })
        .ok_or(failed("texture consumer"))?;
        let surface = ConsumerSurface::with_raw(raw, callback, (1 << 8) | (1 << 10))?;
        objects.texture = None;
        objects.target = None;
        objects.framebuffer = None;
        objects.program = None;
        objects.vertices = None;
        drop(current);
        Ok(Self {
            surface: Some(surface),
            gl,
            context,
            texture,
            target,
            framebuffer,
            program,
            vertices,
            size: (0, 0),
            allocated: (0, 0),
            max_texture_size: gpu.max_texture_size,
        })
    }
    /// Identity accepted by ArkUI's texture rendering options.
    pub fn id(&mut self) -> Result<u64, Error> {
        self.surface.as_mut().unwrap().id()
    }
    /// Updates future dimensions within native signed and GLES texture limits.
    pub fn resize(&mut self, width: u32, height: u32) -> Result<(), Error> {
        self.layout(width, height)?;
        self.surface.as_mut().unwrap().resize(width, height)?;
        self.size = (width, height);
        Ok(())
    }
    fn layout(&self, width: u32, height: u32) -> Result<crate::layout::Layout, Error> {
        let layout = validate_size(width, height)?;
        if width > self.max_texture_size || height > self.max_texture_size {
            return Err(failed("GLES texture dimensions"));
        }
        Ok(layout)
    }
    /// Consume one pending frame and resolve its layout into linear RGBA.
    /// The system waits for the acquire fence on this rendering worker.
    pub fn read(&mut self) -> Result<Option<RgbaFrame>, Error> {
        let _current = self.context.enter()?;
        let (width, height) = self.size;
        let layout = self.layout(width, height)?;
        let surface = self.surface.as_mut().unwrap();
        // SAFETY: this exclusive consumer and its external texture share the
        // current owned context. It never uses CPU buffer-acquisition APIs.
        let code = unsafe {
            ohos_native_image_sys::OH_NativeImage_UpdateSurfaceImage(surface.raw.as_ptr())
        };
        if code == 40601000 {
            return Ok(None);
        }
        check("update texture", code)?;
        let mut transform = [0.0; 16];
        // SAFETY: the live image writes exactly sixteen floats to this array.
        check("texture transform", unsafe {
            ohos_native_image_sys::OH_NativeImage_GetTransformMatrixV2(
                surface.raw.as_ptr(),
                transform.as_mut_ptr(),
            )
        })?;
        let mut pixels = layout.allocate()?;
        // SAFETY: the context and every GL object are live on this thread;
        // storage and the readback slice have exactly width*height*4 bytes.
        unsafe {
            let gl = &self.gl;
            gl.bind_texture(glow::TEXTURE_2D, Some(self.target));
            if self.allocated != (width, height) {
                gl.tex_image_2d(
                    glow::TEXTURE_2D,
                    0,
                    glow::RGBA as i32,
                    width as i32,
                    height as i32,
                    0,
                    glow::RGBA,
                    glow::UNSIGNED_BYTE,
                    glow::PixelUnpackData::Slice(None),
                );
            }
            gl.bind_framebuffer(glow::FRAMEBUFFER, Some(self.framebuffer));
            gl.framebuffer_texture_2d(
                glow::FRAMEBUFFER,
                glow::COLOR_ATTACHMENT0,
                glow::TEXTURE_2D,
                Some(self.target),
                0,
            );
            if gl.check_framebuffer_status(glow::FRAMEBUFFER) != glow::FRAMEBUFFER_COMPLETE {
                return Err(failed("incomplete readback framebuffer"));
            }
            gl.viewport(0, 0, width as i32, height as i32);
            gl.disable(glow::BLEND);
            gl.disable(glow::SCISSOR_TEST);
            gl.use_program(Some(self.program));
            gl.uniform_matrix_4_f32_slice(
                gl.get_uniform_location(self.program, "transform").as_ref(),
                false,
                &transform,
            );
            gl.uniform_1_i32(gl.get_uniform_location(self.program, "image").as_ref(), 0);
            gl.active_texture(glow::TEXTURE0);
            gl.bind_texture(TEXTURE_EXTERNAL_OES, Some(self.texture));
            gl.bind_buffer(glow::ARRAY_BUFFER, Some(self.vertices));
            gl.enable_vertex_attrib_array(0);
            gl.vertex_attrib_pointer_f32(0, 2, glow::FLOAT, false, 0, 0);
            gl.draw_arrays(glow::TRIANGLE_STRIP, 0, 4);
            gl.pixel_store_i32(glow::PACK_ALIGNMENT, 1);
            gl.read_pixels(
                0,
                0,
                width as i32,
                height as i32,
                glow::RGBA,
                glow::UNSIGNED_BYTE,
                glow::PixelPackData::Slice(Some(&mut pixels)),
            );
            let code = gl.get_error();
            if code != glow::NO_ERROR {
                return Err(Error {
                    operation: "GLES readback",
                    code: code as i32,
                });
            }
        }
        self.allocated = (width, height);
        // GLES readback starts with the bottom row; consumers use top-left RGBA.
        let stride = layout.row_bytes;
        for y in 0..height as usize / 2 {
            let (top, bottom) = pixels.split_at_mut((height as usize - 1 - y) * stride);
            top[y * stride..(y + 1) * stride].swap_with_slice(&mut bottom[..stride]);
        }
        Ok(Some(RgbaFrame {
            width,
            height,
            pixels,
        }))
    }
}
impl Drop for TextureReader {
    fn drop(&mut self) {
        if let Ok(_current) = self.context.enter() {
            self.surface.take();
            // SAFETY: native image has retired before its texture; all objects
            // are deleted on their creating thread in their current context.
            unsafe {
                self.gl.delete_buffer(self.vertices);
                self.gl.delete_program(self.program);
                self.gl.delete_framebuffer(self.framebuffer);
                self.gl.delete_texture(self.target);
                self.gl.delete_texture(self.texture);
            }
        } else if let Some(mut surface) = self.surface.take() {
            let callback = surface.retire_callback();
            // The SDK image belongs to its creating EGL context. If entering
            // fails, retain that complete owner rather than destroy it in an
            // unrelated context. Retire Rust delivery before quarantining it.
            std::mem::forget((surface, self.context.clone(), self.gl.clone()));
            drop(callback);
        }
    }
}

#[cfg(all(test, not(target_env = "ohos")))]
mod tests {
    use super::*;
    use std::num::NonZeroU32;

    unsafe extern "C" {
        fn ohos_test_egl_reset();
        fn ohos_test_egl_fail(operation: i32);
        fn ohos_test_egl_destroyed() -> i32;
    }

    #[test]
    fn restores_selected_api_and_prior_es_context_on_success_and_failure() {
        // SAFETY: statically linked host substitutes keep all state thread-local.
        unsafe { ohos_test_egl_reset() };
        let egl = egl::Instance::new(egl::Static);
        // SAFETY: the host substitute accepts these nonnull identities as live handles.
        let (display, surface, es, gl) = unsafe {
            (
                egl::Display::from_ptr(1_usize as *mut _),
                egl::Surface::from_ptr(5_usize as *mut _),
                egl::Context::from_ptr(6_usize as *mut _),
                egl::Context::from_ptr(7_usize as *mut _),
            )
        };
        egl.make_current(display, Some(surface), Some(surface), Some(es))
            .unwrap();
        egl.bind_api(egl::OPENGL_API).unwrap();
        egl.make_current(display, Some(surface), Some(surface), Some(gl))
            .unwrap();
        let owned = Context::new().unwrap();
        assert_eq!(egl.query_api(), egl::OPENGL_API);
        assert_eq!(egl.get_current_context(), Some(gl));
        {
            let _current = owned.enter().unwrap();
            assert_eq!(egl.query_api(), egl::OPENGL_ES_API);
            assert_eq!(egl.get_current_context(), Some(owned.context));
        }
        assert_eq!(egl.query_api(), egl::OPENGL_API);
        assert_eq!(egl.get_current_context(), Some(gl));
        egl.bind_api(egl::OPENGL_ES_API).unwrap();
        assert_eq!(egl.get_current_context(), Some(es));
        egl.bind_api(egl::OPENGL_API).unwrap();
        // SAFETY: failure injection affects only this thread's next make-current call.
        unsafe { ohos_test_egl_fail(1) };
        assert!(owned.enter().is_err());
        assert_eq!(egl.query_api(), egl::OPENGL_API);
        egl.bind_api(egl::OPENGL_ES_API).unwrap();
        assert_eq!(egl.get_current_context(), Some(es));
        egl.bind_api(egl::OPENGL_API).unwrap();
        // SAFETY: failure injection affects only this thread's next creation call.
        unsafe { ohos_test_egl_fail(2) };
        assert!(Context::new().is_err());
        assert_eq!(egl.query_api(), egl::OPENGL_API);
        drop(owned);
        // SAFETY: reads thread-local host cleanup instrumentation.
        assert_eq!(unsafe { ohos_test_egl_destroyed() }, 2);
    }

    #[test]
    fn failed_drop_quarantines_native_image_with_its_context_and_retires_delivery() {
        let _serial = crate::tests::TEST_LOCK.lock().unwrap();
        // SAFETY: statically linked host substitutes keep all state thread-local.
        unsafe { ohos_test_egl_reset() };
        let gpu = TextureContext::new().unwrap();
        let context = Rc::downgrade(&gpu.context);
        let captured = std::sync::Arc::new(());
        let callback = std::sync::Arc::downgrade(&captured);
        let surface = ConsumerSurface::new(move || {
            let _ = &captured;
        })
        .unwrap();
        let name = NonZeroU32::new(1).unwrap();
        let reader = TextureReader {
            surface: Some(surface),
            gl: gpu.gl.clone(),
            context: gpu.context.clone(),
            texture: glow::NativeTexture(name),
            target: glow::NativeTexture(name),
            framebuffer: glow::NativeFramebuffer(name),
            program: glow::NativeProgram(name),
            vertices: glow::NativeBuffer(name),
            size: (0, 0),
            allocated: (0, 0),
            max_texture_size: gpu.max_texture_size,
        };
        assert!(reader.layout(4096, 2048).is_ok());
        assert!(reader.layout(4097, 1).is_err());
        let destroyed = crate::tests::native_destroyed();
        // SAFETY: the next production Drop entry must fail before native/GL teardown.
        unsafe { ohos_test_egl_fail(1) };
        drop(reader);
        drop(gpu);
        assert_eq!(crate::tests::native_destroyed(), destroyed);
        assert!(context.upgrade().is_some());
        assert!(callback.upgrade().is_none());
        // SAFETY: the complete context remains quarantined with its native image.
        assert_eq!(unsafe { ohos_test_egl_destroyed() }, 0);
    }
}
