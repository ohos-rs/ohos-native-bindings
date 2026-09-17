import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { window } from "@kit.ArkUI";

/** Provides a real UIContent window for the WindowManager QEMU probes. */
export default class WindowManagerTestAbility extends UIAbility {
  onCreate(_want: Want, _launchParam: AbilityConstant.LaunchParam): void {}

  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent("pages/WindowManagerTestHost");
  }
}
