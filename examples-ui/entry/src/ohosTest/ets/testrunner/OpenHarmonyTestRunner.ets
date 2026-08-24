import { UIAbility, Want } from "@kit.AbilityKit";
import { BusinessError } from "@kit.BasicServicesKit";
import { hilog } from "@kit.PerformanceAnalysisKit";
import { abilityDelegatorRegistry, TestRunner } from "@kit.TestKit";

const TAG: string = "bindingsOhosTest";
const TEST_ABILITY_NAME: string = "TestAbility";

async function onAbilityCreateCallback(data: UIAbility): Promise<void> {
  hilog.info(0x0000, TAG, "TestAbility created %{public}s", JSON.stringify(data) ?? "");
}

async function addAbilityMonitorCallback(error: BusinessError): Promise<void> {
  if (error?.code) {
    hilog.error(0x0000, TAG, "addAbilityMonitor failed %{public}s", JSON.stringify(error));
    return;
  }
  hilog.info(0x0000, TAG, "ability monitor registered");
}

export default class OpenHarmonyTestRunner implements TestRunner {
  onPrepare(): void {
    hilog.info(0x0000, TAG, "OpenHarmonyTestRunner onPrepare");
  }

  async onRun(): Promise<void> {
    hilog.info(0x0000, TAG, "OpenHarmonyTestRunner onRun");
    const abilityDelegator = abilityDelegatorRegistry.getAbilityDelegator();
    const args = abilityDelegatorRegistry.getArguments();
    const moduleName: string = args.parameters["-m"] ?? "entry_test";
    const bundleName: string = args.bundleName;

    abilityDelegator.addAbilityMonitor(
      {
        abilityName: TEST_ABILITY_NAME,
        moduleName: moduleName,
        onAbilityCreate: onAbilityCreateCallback,
      },
      addAbilityMonitorCallback,
    );

    const want: Want = {
      bundleName: bundleName,
      moduleName: moduleName,
      abilityName: TEST_ABILITY_NAME,
    };

    abilityDelegator.startAbility(want, (error: BusinessError) => {
      if (error?.code) {
        hilog.error(0x0000, TAG, "start TestAbility failed %{public}s", JSON.stringify(error));
        abilityDelegator.finishTest("Start TestAbility failed.", error.code, () => {});
        return;
      }
      hilog.info(0x0000, TAG, "TestAbility started");
    });
  }
}
