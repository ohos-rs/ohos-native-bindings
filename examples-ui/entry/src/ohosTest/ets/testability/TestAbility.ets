import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { window } from "@kit.ArkUI";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { abilityDelegatorRegistry } from "@kit.TestKit";
import { Hypium } from "@ohos/hypium";
import { BusinessError } from "@kit.BasicServicesKit";
import testsuite from "../test/List.test";

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

  onWindowStageCreate(_windowStage: window.WindowStage): void {
    // `aa test` starts this ability in a FRESH process: any app instance
    // started earlier is killed and its .so state does not carry over. The
    // ohosTest ArkTS pipeline (arkdown) cannot register router pages for the
    // entry_test module either (loadContent fails with "page not found").
    // NOTE: `aa test` also starts this ability in a fresh process and does
    // not deliver a window for a second UIAbility started from here
    // (startAbility resolves but SCB never activates EntryAbility), so
    // host-page surface tests cannot run inside the test process.
    // Surface-dependent behavior is verified on the main app's
    // OhosTestHost page; the ohosTest suite pins the deterministic
    // no-surface baseline of each binding.
    hilog.info(0x0000, TAG, "TestAbility onWindowStageCreate");
    const args = abilityDelegatorRegistry.getArguments();
    Hypium.hypiumTest(this.abilityDelegator, args, testsuite);
  }
}
