import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { window } from "@kit.ArkUI";

export default class AccessibilityXComponentTestAbility extends UIAbility {
  onCreate(_want: Want, _launchParam: AbilityConstant.LaunchParam): void {}

  onWindowStageCreate(windowStage: window.WindowStage): void {
    windowStage.loadContent("pages/AccessibilityXComponentTestHost");
  }
}
