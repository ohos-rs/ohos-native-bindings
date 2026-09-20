import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { window } from "@kit.ArkUI";

/** Dedicated main-process entry for the ArkWeb binding E2E. */
export default class ArkWebE2ETestAbility extends UIAbility {
  onCreate(_want: Want, _launchParam: AbilityConstant.LaunchParam): void {}

  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent("pages/ArkWebE2ETestHost");
  }
}
