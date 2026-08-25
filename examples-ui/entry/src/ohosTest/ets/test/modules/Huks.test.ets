import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import { abortSession, hmacMac, rsaGenerateExport, smoke } from "libhuks_test.so";

export default function huksTest(): void {
  describe("huks", () => {
    it("hmacMac", 0, () => {
      expect(ranOrThrew((): string => hmacMac("hello-huks"))).assertTrue();
    });
    it("rsaGenerateExport", 0, () => {
      expect(ranOrThrew((): string => rsaGenerateExport())).assertTrue();
    });
    it("abortSession", 0, () => {
      expect(ranOrThrew((): string => abortSession())).assertTrue();
    });
    it("smoke", 0, () => {
      expect(ranOrThrew((): string => smoke())).assertTrue();
    });
  });
}
