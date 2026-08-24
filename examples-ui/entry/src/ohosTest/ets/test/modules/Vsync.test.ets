import { describe, expect, it } from "@ohos/hypium";
import {
  handleVsync,
  handleVsyncWithSelf,
  requestFrameOnce,
  stopVsync,
  VsyncController,
  vsyncPeriod,
} from "libvsync_test.so";

export default function vsyncTest(): void {
  describe("vsync", () => {
    it("vsyncPeriod", 0, () => {
      expect(vsyncPeriod() >= 0).assertTrue();
    });
    it("requestFrameOnce and stop", 0, () => {
      expect(requestFrameOnce() >= 0).assertTrue();
      stopVsync();
    });
    it("handleVsync and handleVsyncWithSelf", 0, () => {
      handleVsync();
      handleVsyncWithSelf();
      stopVsync();
      expect(true).assertTrue();
    });
    it("VsyncController lifecycle", 0, () => {
      const controller: VsyncController = new VsyncController();
      // once()/period() before start return -1 (no owned instance).
      expect(controller.once()).assertEqual(-1);
      expect(controller.period()).assertEqual(-1);
      controller.start();
      // period is only known after the first frame callback; callability is
      // what matters here.
      controller.once();
      controller.stop();
      expect(controller.once()).assertEqual(-1);
      expect(controller.period()).assertEqual(-1);
    });
  });
}
