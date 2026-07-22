use once_cell::sync::Lazy;

use crate::SysConfig;

/// Game Controller Kit (`@library libohgame_controller.z.so`,
/// `SystemCapability.Game.GameController`).
///
/// All five headers share `GameController_ErrorCode` and the opaque event/device handles,
/// and all declare the same `@library`, so they form a single sys crate.
///
/// Every symbol is `@since 21`, so the whole crate is gated behind `feature = "api-21"`.
///
/// The enum variants are unprefixed (`UP`, `DOWN`, `UNKNOWN`, `DPAD`, `OFFLINE`, ...), so the
/// allowlist names the enum *types*; bindgen emits their constants along with the type.
/// Matching the variants directly would need patterns broad enough to drag in libc items.
pub const GAME_CONTROLLER: Lazy<SysConfig> = Lazy::new(|| SysConfig {
    name: "ohos-game-controller-sys",
    headers: vec![
        "GameControllerKit/game_controller_type.h",
        "GameControllerKit/game_device_event.h",
        "GameControllerKit/game_device.h",
        "GameControllerKit/game_pad_event.h",
        "GameControllerKit/game_pad.h",
    ],
    white_list: vec![
        "OH_GameDevice_.*",
        "OH_GamePad_.*",
        "GameController_.*",
        "GameDevice_.*",
        "GamePad_.*",
    ],
    block_list: vec![],
    dynamic_library: vec!["ohgame_controller.z"],
    extra: "",
});
