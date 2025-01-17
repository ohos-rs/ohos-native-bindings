/* automatically generated by rust-bindgen 0.65.1 */

#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]

pub type char16_t = ::std::os::raw::c_ushort;
pub const InputMethod_KeyboardStatus_IME_KEYBOARD_STATUS_NONE: InputMethod_KeyboardStatus = 0;
pub const InputMethod_KeyboardStatus_IME_KEYBOARD_STATUS_HIDE: InputMethod_KeyboardStatus = 1;
pub const InputMethod_KeyboardStatus_IME_KEYBOARD_STATUS_SHOW: InputMethod_KeyboardStatus = 2;
pub type InputMethod_KeyboardStatus = ::std::os::raw::c_uint;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_UNSPECIFIED: InputMethod_EnterKeyType = 0;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_NONE: InputMethod_EnterKeyType = 1;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_GO: InputMethod_EnterKeyType = 2;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_SEARCH: InputMethod_EnterKeyType = 3;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_SEND: InputMethod_EnterKeyType = 4;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_NEXT: InputMethod_EnterKeyType = 5;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_DONE: InputMethod_EnterKeyType = 6;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_PREVIOUS: InputMethod_EnterKeyType = 7;
pub const InputMethod_EnterKeyType_IME_ENTER_KEY_NEWLINE: InputMethod_EnterKeyType = 8;
pub type InputMethod_EnterKeyType = ::std::os::raw::c_uint;
pub const InputMethod_Direction_IME_DIRECTION_NONE: InputMethod_Direction = 0;
pub const InputMethod_Direction_IME_DIRECTION_UP: InputMethod_Direction = 1;
pub const InputMethod_Direction_IME_DIRECTION_DOWN: InputMethod_Direction = 2;
pub const InputMethod_Direction_IME_DIRECTION_LEFT: InputMethod_Direction = 3;
pub const InputMethod_Direction_IME_DIRECTION_RIGHT: InputMethod_Direction = 4;
pub type InputMethod_Direction = ::std::os::raw::c_uint;
pub const InputMethod_ExtendAction_IME_EXTEND_ACTION_SELECT_ALL: InputMethod_ExtendAction = 0;
pub const InputMethod_ExtendAction_IME_EXTEND_ACTION_CUT: InputMethod_ExtendAction = 3;
pub const InputMethod_ExtendAction_IME_EXTEND_ACTION_COPY: InputMethod_ExtendAction = 4;
pub const InputMethod_ExtendAction_IME_EXTEND_ACTION_PASTE: InputMethod_ExtendAction = 5;
pub type InputMethod_ExtendAction = ::std::os::raw::c_uint;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_NONE: InputMethod_TextInputType = -1;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_TEXT: InputMethod_TextInputType = 0;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_MULTILINE: InputMethod_TextInputType = 1;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_NUMBER: InputMethod_TextInputType = 2;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_PHONE: InputMethod_TextInputType = 3;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_DATETIME: InputMethod_TextInputType = 4;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_EMAIL_ADDRESS: InputMethod_TextInputType =
    5;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_URL: InputMethod_TextInputType = 6;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_VISIBLE_PASSWORD:
    InputMethod_TextInputType = 7;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_NUMBER_PASSWORD: InputMethod_TextInputType =
    8;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_SCREEN_LOCK_PASSWORD:
    InputMethod_TextInputType = 9;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_USER_NAME: InputMethod_TextInputType = 10;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_NEW_PASSWORD: InputMethod_TextInputType =
    11;
pub const InputMethod_TextInputType_IME_TEXT_INPUT_TYPE_NUMBER_DECIMAL: InputMethod_TextInputType =
    12;
pub type InputMethod_TextInputType = ::std::os::raw::c_int;
pub const InputMethod_CommandValueType_IME_COMMAND_VALUE_TYPE_NONE: InputMethod_CommandValueType =
    0;
pub const InputMethod_CommandValueType_IME_COMMAND_VALUE_TYPE_STRING: InputMethod_CommandValueType =
    1;
pub const InputMethod_CommandValueType_IME_COMMAND_VALUE_TYPE_BOOL: InputMethod_CommandValueType =
    2;
pub const InputMethod_CommandValueType_IME_COMMAND_VALUE_TYPE_INT32: InputMethod_CommandValueType =
    3;
pub type InputMethod_CommandValueType = ::std::os::raw::c_uint;
pub const InputMethod_ErrorCode_IME_ERR_OK: InputMethod_ErrorCode = 0;
pub const InputMethod_ErrorCode_IME_ERR_UNDEFINED: InputMethod_ErrorCode = 1;
pub const InputMethod_ErrorCode_IME_ERR_PARAMCHECK: InputMethod_ErrorCode = 401;
pub const InputMethod_ErrorCode_IME_ERR_PACKAGEMANAGER: InputMethod_ErrorCode = 12800001;
pub const InputMethod_ErrorCode_IME_ERR_IMENGINE: InputMethod_ErrorCode = 12800002;
pub const InputMethod_ErrorCode_IME_ERR_IMCLIENT: InputMethod_ErrorCode = 12800003;
pub const InputMethod_ErrorCode_IME_ERR_CONFIG_PERSIST: InputMethod_ErrorCode = 12800005;
pub const InputMethod_ErrorCode_IME_ERR_CONTROLLER: InputMethod_ErrorCode = 12800006;
pub const InputMethod_ErrorCode_IME_ERR_SETTINGS: InputMethod_ErrorCode = 12800007;
pub const InputMethod_ErrorCode_IME_ERR_IMMS: InputMethod_ErrorCode = 12800008;
pub const InputMethod_ErrorCode_IME_ERR_DETACHED: InputMethod_ErrorCode = 12800009;
pub const InputMethod_ErrorCode_IME_ERR_NULL_POINTER: InputMethod_ErrorCode = 12802000;
pub const InputMethod_ErrorCode_IME_ERR_QUERY_FAILED: InputMethod_ErrorCode = 12802001;
pub type InputMethod_ErrorCode = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_PrivateCommand {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_PrivateCommand_Create(
        key: *mut ::std::os::raw::c_char,
        keyLength: usize,
    ) -> *mut InputMethod_PrivateCommand;
}
extern "C" {
    pub fn OH_PrivateCommand_Destroy(command: *mut InputMethod_PrivateCommand);
}
extern "C" {
    pub fn OH_PrivateCommand_SetKey(
        command: *mut InputMethod_PrivateCommand,
        key: *mut ::std::os::raw::c_char,
        keyLength: usize,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_SetBoolValue(
        command: *mut InputMethod_PrivateCommand,
        value: bool,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_SetIntValue(
        command: *mut InputMethod_PrivateCommand,
        value: i32,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_SetStrValue(
        command: *mut InputMethod_PrivateCommand,
        value: *mut ::std::os::raw::c_char,
        valueLength: usize,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_GetKey(
        command: *mut InputMethod_PrivateCommand,
        key: *mut *const ::std::os::raw::c_char,
        keyLength: *mut usize,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_GetValueType(
        command: *mut InputMethod_PrivateCommand,
        type_: *mut InputMethod_CommandValueType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_GetBoolValue(
        command: *mut InputMethod_PrivateCommand,
        value: *mut bool,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_GetIntValue(
        command: *mut InputMethod_PrivateCommand,
        value: *mut i32,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_PrivateCommand_GetStrValue(
        command: *mut InputMethod_PrivateCommand,
        value: *mut *const ::std::os::raw::c_char,
        valueLength: *mut usize,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_CursorInfo {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_CursorInfo_Create(
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    ) -> *mut InputMethod_CursorInfo;
}
extern "C" {
    pub fn OH_CursorInfo_Destroy(cursorInfo: *mut InputMethod_CursorInfo);
}
extern "C" {
    pub fn OH_CursorInfo_SetRect(
        cursorInfo: *mut InputMethod_CursorInfo,
        left: f64,
        top: f64,
        width: f64,
        height: f64,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_CursorInfo_GetRect(
        cursorInfo: *mut InputMethod_CursorInfo,
        left: *mut f64,
        top: *mut f64,
        width: *mut f64,
        height: *mut f64,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_TextAvoidInfo {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_TextAvoidInfo_Create(positionY: f64, height: f64) -> *mut InputMethod_TextAvoidInfo;
}
extern "C" {
    pub fn OH_TextAvoidInfo_Destroy(info: *mut InputMethod_TextAvoidInfo);
}
extern "C" {
    pub fn OH_TextAvoidInfo_SetPositionY(
        info: *mut InputMethod_TextAvoidInfo,
        positionY: f64,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextAvoidInfo_SetHeight(
        info: *mut InputMethod_TextAvoidInfo,
        height: f64,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextAvoidInfo_GetPositionY(
        info: *mut InputMethod_TextAvoidInfo,
        positionY: *mut f64,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextAvoidInfo_GetHeight(
        info: *mut InputMethod_TextAvoidInfo,
        height: *mut f64,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_TextConfig {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_TextConfig_Create() -> *mut InputMethod_TextConfig;
}
extern "C" {
    pub fn OH_TextConfig_Destroy(config: *mut InputMethod_TextConfig);
}
extern "C" {
    pub fn OH_TextConfig_SetInputType(
        config: *mut InputMethod_TextConfig,
        inputType: InputMethod_TextInputType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_SetEnterKeyType(
        config: *mut InputMethod_TextConfig,
        enterKeyType: InputMethod_EnterKeyType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_SetPreviewTextSupport(
        config: *mut InputMethod_TextConfig,
        supported: bool,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_SetSelection(
        config: *mut InputMethod_TextConfig,
        start: i32,
        end: i32,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_SetWindowId(
        config: *mut InputMethod_TextConfig,
        windowId: i32,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetInputType(
        config: *mut InputMethod_TextConfig,
        inputType: *mut InputMethod_TextInputType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetEnterKeyType(
        config: *mut InputMethod_TextConfig,
        enterKeyType: *mut InputMethod_EnterKeyType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_IsPreviewTextSupported(
        config: *mut InputMethod_TextConfig,
        supported: *mut bool,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetCursorInfo(
        config: *mut InputMethod_TextConfig,
        cursorInfo: *mut *mut InputMethod_CursorInfo,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetTextAvoidInfo(
        config: *mut InputMethod_TextConfig,
        avoidInfo: *mut *mut InputMethod_TextAvoidInfo,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetSelection(
        config: *mut InputMethod_TextConfig,
        start: *mut i32,
        end: *mut i32,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextConfig_GetWindowId(
        config: *mut InputMethod_TextConfig,
        windowId: *mut i32,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_TextEditorProxy {
    _unused: [u8; 0],
}
pub type OH_TextEditorProxy_GetTextConfigFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        config: *mut InputMethod_TextConfig,
    ),
>;
pub type OH_TextEditorProxy_InsertTextFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        text: *const char16_t,
        length: usize,
    ),
>;
pub type OH_TextEditorProxy_DeleteForwardFunc = ::std::option::Option<
    unsafe extern "C" fn(textEditorProxy: *mut InputMethod_TextEditorProxy, length: i32),
>;
pub type OH_TextEditorProxy_DeleteBackwardFunc = ::std::option::Option<
    unsafe extern "C" fn(textEditorProxy: *mut InputMethod_TextEditorProxy, length: i32),
>;
pub type OH_TextEditorProxy_SendKeyboardStatusFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        keyboardStatus: InputMethod_KeyboardStatus,
    ),
>;
pub type OH_TextEditorProxy_SendEnterKeyFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        enterKeyType: InputMethod_EnterKeyType,
    ),
>;
pub type OH_TextEditorProxy_MoveCursorFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        direction: InputMethod_Direction,
    ),
>;
pub type OH_TextEditorProxy_HandleSetSelectionFunc = ::std::option::Option<
    unsafe extern "C" fn(textEditorProxy: *mut InputMethod_TextEditorProxy, start: i32, end: i32),
>;
pub type OH_TextEditorProxy_HandleExtendActionFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        action: InputMethod_ExtendAction,
    ),
>;
pub type OH_TextEditorProxy_GetLeftTextOfCursorFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        number: i32,
        text: *mut char16_t,
        length: *mut usize,
    ),
>;
pub type OH_TextEditorProxy_GetRightTextOfCursorFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        number: i32,
        text: *mut char16_t,
        length: *mut usize,
    ),
>;
pub type OH_TextEditorProxy_GetTextIndexAtCursorFunc = ::std::option::Option<
    unsafe extern "C" fn(textEditorProxy: *mut InputMethod_TextEditorProxy) -> i32,
>;
pub type OH_TextEditorProxy_ReceivePrivateCommandFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        privateCommand: *mut *mut InputMethod_PrivateCommand,
        size: usize,
    ) -> i32,
>;
pub type OH_TextEditorProxy_SetPreviewTextFunc = ::std::option::Option<
    unsafe extern "C" fn(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        text: *const char16_t,
        length: usize,
        start: i32,
        end: i32,
    ) -> i32,
>;
pub type OH_TextEditorProxy_FinishTextPreviewFunc =
    ::std::option::Option<unsafe extern "C" fn(textEditorProxy: *mut InputMethod_TextEditorProxy)>;
extern "C" {
    pub fn OH_TextEditorProxy_Create() -> *mut InputMethod_TextEditorProxy;
}
extern "C" {
    pub fn OH_TextEditorProxy_Destroy(proxy: *mut InputMethod_TextEditorProxy);
}
extern "C" {
    pub fn OH_TextEditorProxy_SetGetTextConfigFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getTextConfigFunc: OH_TextEditorProxy_GetTextConfigFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetInsertTextFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        insertTextFunc: OH_TextEditorProxy_InsertTextFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetDeleteForwardFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        deleteForwardFunc: OH_TextEditorProxy_DeleteForwardFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetDeleteBackwardFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        deleteBackwardFunc: OH_TextEditorProxy_DeleteBackwardFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetSendKeyboardStatusFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        sendKeyboardStatusFunc: OH_TextEditorProxy_SendKeyboardStatusFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetSendEnterKeyFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        sendEnterKeyFunc: OH_TextEditorProxy_SendEnterKeyFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetMoveCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        moveCursorFunc: OH_TextEditorProxy_MoveCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetHandleSetSelectionFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        handleSetSelectionFunc: OH_TextEditorProxy_HandleSetSelectionFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetHandleExtendActionFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        handleExtendActionFunc: OH_TextEditorProxy_HandleExtendActionFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetGetLeftTextOfCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getLeftTextOfCursorFunc: OH_TextEditorProxy_GetLeftTextOfCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetGetRightTextOfCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getRightTextOfCursorFunc: OH_TextEditorProxy_GetRightTextOfCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetGetTextIndexAtCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getTextIndexAtCursorFunc: OH_TextEditorProxy_GetTextIndexAtCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetReceivePrivateCommandFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        receivePrivateCommandFunc: OH_TextEditorProxy_ReceivePrivateCommandFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetSetPreviewTextFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        setPreviewTextFunc: OH_TextEditorProxy_SetPreviewTextFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_SetFinishTextPreviewFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        finishTextPreviewFunc: OH_TextEditorProxy_FinishTextPreviewFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetGetTextConfigFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getTextConfigFunc: *mut OH_TextEditorProxy_GetTextConfigFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetInsertTextFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        insertTextFunc: *mut OH_TextEditorProxy_InsertTextFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetDeleteForwardFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        deleteForwardFunc: *mut OH_TextEditorProxy_DeleteForwardFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetDeleteBackwardFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        deleteBackwardFunc: *mut OH_TextEditorProxy_DeleteBackwardFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetSendKeyboardStatusFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        sendKeyboardStatusFunc: *mut OH_TextEditorProxy_SendKeyboardStatusFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetSendEnterKeyFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        sendEnterKeyFunc: *mut OH_TextEditorProxy_SendEnterKeyFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetMoveCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        moveCursorFunc: *mut OH_TextEditorProxy_MoveCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetHandleSetSelectionFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        handleSetSelectionFunc: *mut OH_TextEditorProxy_HandleSetSelectionFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetHandleExtendActionFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        handleExtendActionFunc: *mut OH_TextEditorProxy_HandleExtendActionFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetGetLeftTextOfCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getLeftTextOfCursorFunc: *mut OH_TextEditorProxy_GetLeftTextOfCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetGetRightTextOfCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getRightTextOfCursorFunc: *mut OH_TextEditorProxy_GetRightTextOfCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetGetTextIndexAtCursorFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        getTextIndexAtCursorFunc: *mut OH_TextEditorProxy_GetTextIndexAtCursorFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetReceivePrivateCommandFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        receivePrivateCommandFunc: *mut OH_TextEditorProxy_ReceivePrivateCommandFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetSetPreviewTextFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        setPreviewTextFunc: *mut OH_TextEditorProxy_SetPreviewTextFunc,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_TextEditorProxy_GetFinishTextPreviewFunc(
        proxy: *mut InputMethod_TextEditorProxy,
        finishTextPreviewFunc: *mut OH_TextEditorProxy_FinishTextPreviewFunc,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_InputMethodProxy {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_InputMethodProxy_ShowKeyboard(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodProxy_HideKeyboard(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodProxy_NotifySelectionChange(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
        text: *mut char16_t,
        length: usize,
        start: ::std::os::raw::c_int,
        end: ::std::os::raw::c_int,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodProxy_NotifyConfigurationChange(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
        enterKey: InputMethod_EnterKeyType,
        textType: InputMethod_TextInputType,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodProxy_NotifyCursorUpdate(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
        cursorInfo: *mut InputMethod_CursorInfo,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodProxy_SendPrivateCommand(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
        privateCommand: *mut *mut InputMethod_PrivateCommand,
        size: usize,
    ) -> InputMethod_ErrorCode;
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct InputMethod_AttachOptions {
    _unused: [u8; 0],
}
extern "C" {
    pub fn OH_AttachOptions_Create(showKeyboard: bool) -> *mut InputMethod_AttachOptions;
}
extern "C" {
    pub fn OH_AttachOptions_Destroy(options: *mut InputMethod_AttachOptions);
}
extern "C" {
    pub fn OH_AttachOptions_IsShowKeyboard(
        options: *mut InputMethod_AttachOptions,
        showKeyboard: *mut bool,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodController_Attach(
        textEditorProxy: *mut InputMethod_TextEditorProxy,
        options: *mut InputMethod_AttachOptions,
        inputMethodProxy: *mut *mut InputMethod_InputMethodProxy,
    ) -> InputMethod_ErrorCode;
}
extern "C" {
    pub fn OH_InputMethodController_Detach(
        inputMethodProxy: *mut InputMethod_InputMethodProxy,
    ) -> InputMethod_ErrorCode;
}
