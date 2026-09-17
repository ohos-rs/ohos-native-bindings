import { describe, expect, it } from "@ohos/hypium";
import { delay, testWindowId } from "../helpers/Utils";
import { apiReport, callbackReport, layoutSmoke, smoke } from "libwindow_manager_test.so";

export default function windowManagerTest(): void {
  describe("window_manager", () => {
    it("reads visible window layouts", 0, () => {
      const result: string = layoutSmoke(0);
      expect(result).assertContain("layout_error=801:");
    });
    it("records the no-UIContent property failure", 0, () => {
      const windowId: number = testWindowId();
      let error: string = "";
      try {
        smoke(windowId);
      } catch (cause) {
        error = `${cause}`;
      }
      expect(error).assertContain("1300003");
    });
    it("pins the API-26 QEMU capability matrix", 0, () => {
      const result: string = apiReport(testWindowId());
      console.info(`WINDOW_MANAGER_API_REPORT ${result}`);
      const expected: string =
        "properties=err:1300003;avoid_area=ok;is_shown=ok;show=ok;" +
        "status_bar_enabled=ok;status_bar_color=ok;navigation_bar_enabled=ok;" +
        "touchable=ok;focusable=ok;background_color=known_qemu_abort:no_uicontent;" +
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
        "register_frame_callback=ok";
      expect(result).assertEqual(expected);
    });
    it("records unavailable asynchronous callbacks", 0, async () => {
      await delay(500);
      const result: string = callbackReport(testWindowId());
      console.info(`WINDOW_MANAGER_CALLBACK_REPORT ${result}`);
      expect(result).assertEqual(
        "snapshot_callbacks=0;snapshot_releases=0;frame_callbacks=0;" +
        "frame_metrics_ok_mask=0;unregister_frame_callback=err:1300016",
      );
    });
    it("reports invalid-window errors without accepting success", 0, () => {
      const result: string = apiReport(-1);
      console.info(`WINDOW_MANAGER_INVALID_REPORT ${result}`);
      const expected: string =
        "properties=err:1300002;avoid_area=err:1300002;is_shown=err:1300002;" +
        "show=err:1300002;status_bar_enabled=err:1300002;status_bar_color=err:1300002;" +
        "navigation_bar_enabled=err:1300002;touchable=err:1300002;focusable=err:1300002;" +
        "background_color=known_qemu_abort:no_uicontent;brightness=err:1300002;" +
        "keep_screen_on=err:1300002;privacy_mode=err:1300002;snapshot=ok;" +
        "register_key_filter=err:1000;get_key_filter=err:1000;" +
        "unregister_key_filter=err:1000;register_mouse_filter=err:1000;" +
        "get_mouse_filter=err:1000;unregister_mouse_filter=err:1000;" +
        "register_touch_filter=err:1000;get_touch_filter=err:1000;" +
        "unregister_touch_filter=err:1000;visible_layouts=err:801;" +
        "inject_touch_event=err:401;inject_touch_event_up=err:401;main_windows=err:801;" +
        "main_window_snapshots=err:801;" +
        "lock_cursor=err:401;unlock_cursor=err:401;density_info=err:1300016;" +
        "register_density_callback=err:1300002;unregister_density_callback=err:1300016;" +
        "register_frame_callback=err:1300002";
      expect(result).assertEqual(expected);
    });
  });
}
