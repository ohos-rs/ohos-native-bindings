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
      expect(appIdentifier().length > 0).assertTrue();
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
