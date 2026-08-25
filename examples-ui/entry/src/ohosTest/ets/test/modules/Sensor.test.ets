import { describe, expect, it } from "@ohos/hypium";
import { sensorList, sensorTest, subscribeSensor, unsubscribeSensor } from "libsensor_test.so";

export default function sensorTestSuite(): void {
  describe("sensor", () => {
    it("subscribe unsubscribe accelerometer", 0, () => {
      subscribeSensor(1, 200000000);
      unsubscribeSensor();
      expect(true).assertTrue();
    });
    it("sensor_list", 0, () => {
      expect(sensorList().length > 0).assertTrue();
    });
    it("sensor_test subscribes accelerometer", 0, () => {
      sensorTest();
      expect(true).assertTrue();
    });
  });
}
