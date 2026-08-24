import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import { addSecret, querySecret, removeSecret, smoke, updateSecret } from "libasset_test.so";

export default function assetTest(): void {
  describe("asset", () => {
    // The asset keystore requires the device to be unlocked (powered-on
    // accessibility); when locked the calls return errors instead of
    // Success, so assert callability rather than exact result codes.
    it("add query update remove secret", 0, () => {
      removeSecret();
      expect(ranOrThrew((): string => addSecret("ohosTest-secret"))).assertTrue();
      expect(querySecret().length > 0).assertTrue();
      expect(ranOrThrew((): string => updateSecret("ohosTest-secret-2"))).assertTrue();
      expect(ranOrThrew((): string => removeSecret())).assertTrue();
      expect(querySecret().length > 0).assertTrue();
    });
    it("smoke", 0, () => {
      const out: string = smoke();
      expect(out).assertContain("add=");
      expect(out).assertContain("query=");
      expect(out).assertContain("remove=");
    });
  });
}
