// Host-only NDK test double. Destroyed editor storage remains inspectable so
// a stale native callback is reported deterministically instead of invoking UB.
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>
#include <stdlib.h>

typedef void (*Callback)(void);
typedef struct Editor {
    Callback callbacks[15];
    bool destroyed;
} Editor;
typedef struct Cursor { double rect[4]; } Cursor;
typedef struct Config {
    Cursor cursor;
    int32_t enter, input, start, end, window;
    bool preview;
} Config;

static Editor *listener;
static uint32_t show_error, hide_error, attach_error, detach_error, notify_error;
static uint32_t created, destroyed, dangling, invalid_calls, attaches, detaches;
static bool null_attach;

uint32_t ime_mock_stop(void) {
    if (!listener) return 0;
    if (listener->destroyed) { dangling++; return 1; }
    void (*status)(Editor *, uint32_t) = (void (*)(Editor *, uint32_t))listener->callbacks[4];
    if (status) status(listener, 1);
    return 0;
}
void ime_mock_reset(void) {
    show_error = hide_error = attach_error = detach_error = notify_error = 0;
    created = destroyed = dangling = invalid_calls = attaches = detaches = 0;
    null_attach = false;
}
void ime_mock_fail(uint32_t operation, uint32_t code) {
    uint32_t *errors[] = { &show_error, &hide_error, &attach_error, &detach_error, &notify_error };
    *errors[operation] = code;
}
void ime_mock_null_attach(void) { null_attach = true; }
uint32_t ime_mock_count(uint32_t counter) {
    uint32_t counts[] = { created, destroyed, dangling, invalid_calls, attaches, detaches };
    return counts[counter];
}
static uint32_t take_error(uint32_t *slot) { uint32_t result = *slot; *slot = 0; return result; }
static uint32_t validate(void *proxy) {
    if (proxy != listener || !listener || listener->destroyed) { invalid_calls++; return 12800009; }
    return 0;
}
void *OH_TextEditorProxy_Create(void) { created++; return calloc(1, sizeof(Editor)); }
void OH_TextEditorProxy_Destroy(Editor *editor) {
    if (!editor) return;
    if (editor->destroyed || editor == listener) dangling++;
    editor->destroyed = true;
    destroyed++;
}
#define SETTER(name, index) \
    uint32_t OH_TextEditorProxy_Set##name##Func(Editor *editor, Callback callback) { \
        editor->callbacks[index] = callback; return 0; \
    }
SETTER(GetTextConfig, 0)
SETTER(InsertText, 1)
SETTER(DeleteForward, 2)
SETTER(DeleteBackward, 3)
SETTER(SendKeyboardStatus, 4)
SETTER(SendEnterKey, 5)
SETTER(MoveCursor, 6)
SETTER(HandleSetSelection, 7)
SETTER(HandleExtendAction, 8)
SETTER(GetLeftTextOfCursor, 9)
SETTER(GetRightTextOfCursor, 10)
SETTER(GetTextIndexAtCursor, 11)
SETTER(ReceivePrivateCommand, 12)
SETTER(SetPreviewText, 13)
SETTER(FinishTextPreview, 14)
uint32_t OH_TextEditorProxy_SetCallbackInMainThread(Editor *editor, bool enabled) {
    (void)editor; (void)enabled; return 0;
}
void *OH_AttachOptions_Create(bool show) { bool *option = malloc(sizeof(bool)); *option = show; return option; }
void OH_AttachOptions_Destroy(void *option) { free(option); }
uint32_t OH_AttachOptions_IsShowKeyboard(bool *option, bool *show) { *show = *option; return 0; }
uint32_t OH_InputMethodController_Attach(Editor *editor, void *option, void **proxy) {
    (void)option;
    attaches++;
    // An outstanding OnInputStop may arrive while an editor is being replaced.
    ime_mock_stop();
    listener = editor;
    if (attach_error) return take_error(&attach_error);
    if (null_attach) { null_attach = false; return 0; }
    *proxy = editor;
    return 0;
}
uint32_t OH_InputMethodController_Detach(void *proxy) {
    detaches++;
    uint32_t error = validate(proxy);
    if (error) return error;
    if (detach_error) return take_error(&detach_error);
    ime_mock_stop();
    listener = NULL;
    return 0;
}
uint32_t OH_InputMethodProxy_ShowKeyboard(void *proxy) {
    uint32_t error = validate(proxy); return error ? error : take_error(&show_error);
}
uint32_t OH_InputMethodProxy_HideKeyboard(void *proxy) {
    uint32_t error = validate(proxy); return error ? error : take_error(&hide_error);
}
uint32_t OH_InputMethodProxy_NotifyConfigurationChange(void *proxy, int32_t enter, int32_t input) {
    (void)enter; (void)input; uint32_t error = validate(proxy); return error ? error : take_error(&notify_error);
}
uint32_t OH_InputMethodProxy_NotifySelectionChange(void *proxy, uint16_t *text, size_t len, int32_t start, int32_t end) {
    (void)text; (void)len; (void)start; (void)end;
    uint32_t error = validate(proxy); return error ? error : take_error(&notify_error);
}
uint32_t OH_InputMethodProxy_NotifyCursorUpdate(void *proxy, void *cursor) {
    (void)cursor; uint32_t error = validate(proxy); return error ? error : take_error(&notify_error);
}
void *OH_CursorInfo_Create(double left, double top, double width, double height) {
    Cursor *cursor = malloc(sizeof(Cursor)); *cursor = (Cursor){{left, top, width, height}}; return cursor;
}
void OH_CursorInfo_Destroy(void *cursor) { free(cursor); }
uint32_t OH_CursorInfo_SetRect(Cursor *cursor, double left, double top, double width, double height) {
    *cursor = (Cursor){{left, top, width, height}}; return 0;
}
uint32_t OH_CursorInfo_GetRect(Cursor *cursor, double *left, double *top, double *width, double *height) {
    *left = cursor->rect[0]; *top = cursor->rect[1]; *width = cursor->rect[2]; *height = cursor->rect[3]; return 0;
}
void *OH_TextConfig_Create(void) { return calloc(1, sizeof(Config)); }
void OH_TextConfig_Destroy(void *config) { free(config); }
uint32_t OH_TextConfig_GetCursorInfo(Config *config, Cursor **cursor) { *cursor = &config->cursor; return 0; }
#define CONFIG_FIELD(name, field, type) \
    uint32_t OH_TextConfig_Set##name(Config *config, type value) { config->field = value; return 0; } \
    uint32_t OH_TextConfig_Get##name(Config *config, type *value) { *value = config->field; return 0; }
CONFIG_FIELD(EnterKeyType, enter, int32_t)
CONFIG_FIELD(InputType, input, int32_t)
CONFIG_FIELD(WindowId, window, int32_t)
uint32_t OH_TextConfig_SetSelection(Config *config, int32_t start, int32_t end) { config->start = start; config->end = end; return 0; }
uint32_t OH_TextConfig_GetSelection(Config *config, int32_t *start, int32_t *end) { *start = config->start; *end = config->end; return 0; }
uint32_t OH_TextConfig_SetPreviewTextSupport(Config *config, bool preview) { config->preview = preview; return 0; }
uint32_t OH_TextConfig_IsPreviewTextSupported(Config *config, bool *preview) { *preview = config->preview; return 0; }
void OH_PrivateCommand_Destroy(void *command) { free(command); }
