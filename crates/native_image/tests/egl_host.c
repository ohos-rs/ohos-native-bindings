/* Host substitutes use the SDK's EGL declarations, never a copied ABI. */
#define EGL_NO_PLATFORM_SPECIFIC_TYPES
#include <EGL/egl.h>
#include <string.h>

static _Thread_local EGLenum api = EGL_OPENGL_ES_API;
static _Thread_local EGLContext contexts[2];
static _Thread_local EGLSurface draws[2], reads[2];
static _Thread_local EGLDisplay displays[2];
static _Thread_local EGLint error = EGL_SUCCESS;
static _Thread_local int failure, destroyed;
static int slot(void) { return api == EGL_OPENGL_ES_API ? 0 : 1; }

void ohos_test_egl_reset(void) {
    api = EGL_OPENGL_ES_API;
    memset(contexts, 0, sizeof(contexts));
    memset(draws, 0, sizeof(draws));
    memset(reads, 0, sizeof(reads));
    memset(displays, 0, sizeof(displays));
    error = EGL_SUCCESS;
    failure = destroyed = 0;
}
void ohos_test_egl_fail(int operation) { failure = operation; }
int ohos_test_egl_destroyed(void) { return destroyed; }

EGLint eglGetError(void) {
    EGLint result = error;
    error = EGL_SUCCESS;
    return result;
}
EGLenum eglQueryAPI(void) { return api; }
EGLBoolean eglBindAPI(EGLenum value) { api = value; return EGL_TRUE; }
EGLDisplay eglGetDisplay(EGLNativeDisplayType native) { return (EGLDisplay)1; }
EGLBoolean eglInitialize(EGLDisplay display, EGLint *major, EGLint *minor) {
    if (major) *major = 1;
    if (minor) *minor = 5;
    return EGL_TRUE;
}
EGLBoolean eglChooseConfig(EGLDisplay display, const EGLint *attributes,
                          EGLConfig *configs, EGLint size, EGLint *count) {
    *count = 1;
    if (configs && size) *configs = (EGLConfig)2;
    return EGL_TRUE;
}
EGLContext eglCreateContext(EGLDisplay display, EGLConfig config,
                            EGLContext shared, const EGLint *attributes) {
    if (failure == 2 || api != EGL_OPENGL_ES_API) {
        failure = 0;
        error = EGL_BAD_ACCESS;
        return EGL_NO_CONTEXT;
    }
    return (EGLContext)3;
}
EGLSurface eglCreatePbufferSurface(EGLDisplay display, EGLConfig config,
                                   const EGLint *attributes) {
    return (EGLSurface)4;
}
EGLBoolean eglDestroyContext(EGLDisplay display, EGLContext context) {
    destroyed++;
    return EGL_TRUE;
}
EGLBoolean eglDestroySurface(EGLDisplay display, EGLSurface surface) {
    destroyed++;
    return EGL_TRUE;
}
EGLDisplay eglGetCurrentDisplay(void) { return displays[slot()]; }
EGLContext eglGetCurrentContext(void) { return contexts[slot()]; }
EGLSurface eglGetCurrentSurface(EGLint kind) {
    return kind == EGL_DRAW ? draws[slot()] : reads[slot()];
}
EGLBoolean eglMakeCurrent(EGLDisplay display, EGLSurface draw,
                          EGLSurface read, EGLContext context) {
    if (failure == 1) {
        failure = 0;
        error = EGL_BAD_ACCESS;
        return EGL_FALSE;
    }
    contexts[slot()] = context;
    displays[slot()] = context ? display : EGL_NO_DISPLAY;
    draws[slot()] = draw;
    reads[slot()] = read;
    return EGL_TRUE;
}
static const unsigned char *hostGetString(unsigned int name) {
    return (const unsigned char *)(name == 0x1f02 ? "OpenGL ES 2.0" : "");
}
static void hostGetIntegerv(unsigned int name, int *value) { *value = 4096; }
__eglMustCastToProperFunctionPointerType eglGetProcAddress(const char *name) {
    if (!strcmp(name, "glGetString")) return (__eglMustCastToProperFunctionPointerType)hostGetString;
    if (!strcmp(name, "glGetIntegerv")) return (__eglMustCastToProperFunctionPointerType)hostGetIntegerv;
    return 0;
}
