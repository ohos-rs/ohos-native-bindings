import { describe, expect, it } from "@ohos/hypium";
import {
  appId,
  appIdentifier,
  compatibleDeviceType,
  currentBundleInfo,
  mainElementName,
  smoke,
} from "libbundle_test.so";

export default function bundleTest(): void {
  describe("bundle", () => {
    it("currentBundleInfo", 0, () => {
      expect(currentBundleInfo()).assertContain("bundle_name=");
    });
    it("appId", 0, () => {
      expect(appId().length > 0).assertTrue();
    });
    it("appIdentifier", 0, () => {
      // Unsigned and locally test-signed HAPs do not have an appIdentifier.
      // The native API still succeeds and returns a stable string.
      const identifier: string = appIdentifier();
      expect(typeof identifier === "string").assertTrue();
      expect(appIdentifier()).assertEqual(identifier);
    });
    it("mainElementName", 0, () => {
      expect(mainElementName()).assertContain("EntryAbility");
    });
    it("compatibleDeviceType", 0, () => {
      expect(typeof compatibleDeviceType() === "string").assertTrue();
    });
    it("smoke", 0, () => {
      expect(smoke().length > 0).assertTrue();
    });
  });
}
