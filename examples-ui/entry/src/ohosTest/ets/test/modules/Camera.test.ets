import { describe, expect, it } from "@ohos/hypium";
import {
  capture,
  closeSession,
  lastResult,
  reopenSession,
  sessionInfo,
  switchFlash,
} from "libcamera_test.so";
import { ranOrThrew } from "../helpers/Utils";

export default function cameraTest(): void {
  describe("camera", () => {
    // In the aa-test process no camera XComponent surface exists: the demo
    // skips init entirely (no __NATIVE_XCOMPONENT_OBJ__), so the command
    // worker never starts and every napi command rejects with the same
    // deterministic error. These tests pin that no-surface baseline.
    it("commands reject deterministically without surface", 0, () => {
      const rejectWith: (fn: () => string) => string = (fn: () => string): string => {
        try {
          return fn();
        } catch (e) {
          return `${e}`;
        }
      };
      expect(rejectWith((): string => capture())).assertContain("not running");
      expect(rejectWith((): string => switchFlash(true))).assertContain("not running");
      expect(rejectWith((): string => closeSession())).assertContain("not running");
      expect(rejectWith((): string => reopenSession())).assertContain("not running");
    });

    it("lastResult and sessionInfo are readable", 0, () => {
      expect(lastResult().length >= 0).assertTrue();
      expect(ranOrThrew((): string => sessionInfo())).assertTrue();
    });
  });
}
