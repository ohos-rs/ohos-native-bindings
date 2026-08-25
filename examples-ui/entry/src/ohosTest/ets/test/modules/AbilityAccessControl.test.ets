import { describe, expect, it } from "@ohos/hypium";
import { checkPermission, smoke } from "libability_access_control_test.so";

export default function abilityAccessControlTest(): void {
  describe("ability_access_control", () => {
    it("checkPermission internet", 0, () => {
      expect(checkPermission("ohos.permission.INTERNET")).assertTrue();
    });
    it("smoke", 0, () => {
      expect(smoke()).assertContain("INTERNET=");
    });
  });
}
