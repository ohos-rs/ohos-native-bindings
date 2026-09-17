import { describe, expect, it } from "@ohos/hypium";
import { window } from "@kit.ArkUI";
import { UIAbility } from "@kit.AbilityKit";
import { abilityDelegatorRegistry } from "@kit.TestKit";
import { delay } from "../helpers/Utils";
import { apiReport, callbackReport, layoutSmoke, smoke } from "libwindow_manager_test.so";

async function testWindowId(): Promise<number> {
  for (let attempt: number = 0; attempt < 50; attempt++) {
    try {
      const ability: UIAbility = await abilityDelegatorRegistry
        .getAbilityDelegator()
        .getCurrentTopAbility();
      const currentWindow: window.Window = await window.getLastWindow(ability.context);
      await delay(500);
      return currentWindow.getWindowProperties().id;
    } catch {
      await delay(100);
    }
  }
  throw new Error("ohosTest window did not become available");
}

export default function windowManagerTest(): void {
  describe("window_manager", () => {
    it("reads visible window layouts", 0, () => {
      const result: string = layoutSmoke(0);
      expect(result).assertContain("layout_error=801:");
    });
    it("reads the active test window", 0, async () => {
      const windowId: number = await testWindowId();
      const result: string = smoke(windowId);
      expect(result).assertContain(`id=${windowId};`);
      expect(result).assertContain("shown=true;");
    });
    it("pins the API-26 QEMU capability matrix", 0, async () => {
      const result: string = apiReport(await testWindowId());
      console.info(`WINDOW_MANAGER_API_REPORT ${result}`);
      const expected: string =
        "properties=ok;avoid_area=ok;is_shown=ok;show=ok;" +
        "status_bar_enabled=ok;status_bar_color=ok;navigation_bar_enabled=ok;" +
        "touchable=ok;focusable=ok;background_color=ok;" +
        "brightness=ok;keep_screen_on=ok;privacy_mode=err:201;snapshot=ok;" +
        "register_key_filter=err:2000;get_key_filter=none;" +
        "unregister_key_filter=err:2000;register_mouse_filter=err:2000;" +
        "get_mouse_filter=none;unregister_mouse_filter=err:2000;" +
        "register_touch_filter=err:2000;get_touch_filter=none;" +
        "unregister_touch_filter=err:2000;visible_layouts=err:801;" +
        "inject_touch_event=err:1300003;inject_touch_event_up=err:1300003;" +
        "main_windows=err:801;" +
        "main_window_snapshots=err:801;lock_cursor=err:801;unlock_cursor=err:801;" +
        "density_info=some;default_density=ok;system_density=ok;custom_density=ok;" +
        "register_density_callback=err:801;unregister_density_callback=err:1300016;" +
        "register_frame_callback=ok;frame_redraw=ok";
      expect(result).assertEqual(expected);
    });
    it("records unavailable asynchronous callbacks", 0, async () => {
      await delay(500);
      const result: string = callbackReport(await testWindowId());
      console.info(`WINDOW_MANAGER_CALLBACK_REPORT ${result}`);
      expect(result).assertEqual(
        "snapshot_callbacks=0;snapshot_releases=0;frame_callbacks=0;" +
          "frame_metrics_ok_mask=0;unregister_frame_callback=err:1300016",
      );
    });
  });
}
