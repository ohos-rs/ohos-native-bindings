import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { window } from "@kit.ArkUI";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { abilityDelegatorRegistry } from "@kit.TestKit";
import { Hypium } from "@ohos/hypium";
import testsuite from "../test/List.test";
import { setTestWindowId } from "../test/helpers/Utils";

const TAG: string = "bindingsOhosTest";

export default class TestAbility extends UIAbility {
  private abilityDelegator: abilityDelegatorRegistry.AbilityDelegator =
    abilityDelegatorRegistry.getAbilityDelegator();

  onCreate(want: Want, launchParam: AbilityConstant.LaunchParam): void {
    hilog.info(0x0000, TAG, "TestAbility onCreate %{public}s", JSON.stringify(want));
    hilog.info(0x0000, TAG, "launchParam %{public}s", JSON.stringify(launchParam));
  }

  onDestroy(): void {
    hilog.info(0x0000, TAG, "TestAbility onDestroy");
  }

  onWindowStageCreate(windowStage: window.WindowStage): void {
    // `aa test` starts this ability in a fresh process. Arkdown does not
    // package router pages for entry_test, so this window intentionally has
    // no UIContent. Window-manager tests record APIs that require UIContent
    // separately instead of treating a platform abort as a passing test.
    hilog.info(0x0000, TAG, "TestAbility onWindowStageCreate");
    setTestWindowId(windowStage.getMainWindowSync().getWindowProperties().id);
    const args = abilityDelegatorRegistry.getArguments();
    Hypium.hypiumTest(this.abilityDelegator, args, testsuite);
  }
}
