use napi_derive_ohos::napi;
use ohos_init_binding::canIUse;

fn line(cap: &str) -> String {
    format!("{cap}={}", canIUse(cap))
}

/// Query a SystemCapability via `canIUse`.
#[napi]
pub fn can_i_use(cap: String) -> bool {
    canIUse(&cap)
}

#[napi]
pub fn smoke() -> String {
    [
        line("SystemCapability.Sensors.Sensor"),
        line("SystemCapability.Sensors.MiscDevice.Vibrator"),
        line("SystemCapability.Multimedia.Camera.Core"),
        line("SystemCapability.Graphic.Graphic2D.NativeDrawing"),
        line("SystemCapability.Communication.NetStack"),
        line("SystemCapability.Security.Huks.Core"),
        line("this.capability.does.not.exist"),
    ]
    .join("\n")
}
