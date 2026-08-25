import { AbilityConstant, UIAbility, Want } from "@kit.AbilityKit";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { window } from "@kit.ArkUI";

const DOMAIN = 0x0000;

export default class EntryAbility extends UIAbility {
  onCreate(_want: Want, _launchParam: AbilityConstant.LaunchParam): void {
    hilog.info(DOMAIN, "testTag", "%{public}s", "Ability onCreate");
  }

  onWindowStageCreate(windowStage: window.WindowStage): void {
    hilog.info(DOMAIN, "testTag", "%{public}s", "Ability onWindowStageCreate");
    // Start the automated XComponent / NodeContent / Web host immediately.
    // Device-side E2E drivers inspect these live surfaces before aa test starts
    // its isolated Hypium process; no navigation or button click is required.
    windowStage.loadContent("pages/OhosTestHost", (err) => {
      if (err.code) {
        hilog.error(
          DOMAIN,
          "testTag",
          "Failed to load content. Cause: %{public}s",
          JSON.stringify(err),
        );
        return;
      }
      hilog.info(DOMAIN, "testTag", "Succeeded in loading the content.");
    });
  }

  onForeground(): void {
    hilog.info(DOMAIN, "testTag", "%{public}s", "Ability onForeground");
  }

  onBackground(): void {
    hilog.info(DOMAIN, "testTag", "%{public}s", "Ability onBackground");
  }
}
