import { describe, expect, it } from "@ohos/hypium";
import { currentQos, resetQos, setQos, smoke } from "libqos_test.so";

export default function qosTest(): void {
  describe("qos", () => {
    it("set get reset thread qos", 0, () => {
      setQos("UserInteractive");
      expect(currentQos()).assertContain("UserInteractive");
      resetQos();
    });
    it("smoke", 0, () => {
      const result: string = smoke();
      const levels: string[] = [
        "Background",
        "Utility",
        "Default",
        "UserInitiated",
        "DeadlineRequest",
        "UserInteractive",
      ];
      levels.forEach((level: string): void => {
        expect(result).assertContain(`set ${level} -> got ${level}`);
      });
    });
  });
}
