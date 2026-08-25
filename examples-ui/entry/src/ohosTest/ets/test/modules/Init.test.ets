import { describe, expect, it } from "@ohos/hypium";
import { canIUse, smoke } from "libinit_test.so";

export default function initTest(): void {
  describe("init", () => {
    it("canIUse sensor capability", 0, () => {
      expect(canIUse("SystemCapability.Sensors.Sensor")).assertTrue();
    });
    it("canIUse rejects garbage", 0, () => {
      expect(canIUse("this.capability.does.not.exist")).assertFalse();
    });
    it("smoke", 0, () => {
      expect(smoke().length > 0).assertTrue();
    });
  });
}
