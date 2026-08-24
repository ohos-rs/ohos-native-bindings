import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import {
  init,
  initAndKeep,
  refreshPage,
  registerCustomProtocol,
  watchLifecycle,
} from "libark_web_test.so";

const TAG: string = "ohosTest_web";

export default function arkWebTest(): void {
  describe("ark_web", () => {
    // The host page (main module OhosTestHost) hosts a Web component with
    // id=ohosTest_web. The binding's tag-scoped APIs target that instance.
    it("init and initAndKeep register JS proxy", 0, () => {
      expect(ranOrThrew((): void => init(TAG))).assertTrue();
      expect(ranOrThrew((): void => initAndKeep(TAG))).assertTrue();
    });
    it("watchLifecycle registers callbacks", 0, () => {
      expect(ranOrThrew((): void => watchLifecycle(TAG))).assertTrue();
    });
    it("registerCustomProtocol handles custom:// scheme", 0, () => {
      expect(ranOrThrew((): boolean => registerCustomProtocol(TAG))).assertTrue();
    });
    it("refreshPage re-injects proxy", 0, () => {
      expect(ranOrThrew((): void => refreshPage(TAG))).assertTrue();
    });
  });
}
