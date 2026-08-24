import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import {
  htmlRoundtrip,
  plainTextRoundtrip,
  recordAndData,
  smoke,
  utdEquals,
} from "libudmf_test.so";

export default function udmfTest(): void {
  describe("udmf", () => {
    it("plainTextRoundtrip", 0, () => {
      expect(plainTextRoundtrip("hello udmf")).assertContain("hello udmf");
    });
    it("htmlRoundtrip", 0, () => {
      expect(htmlRoundtrip("<b>x</b>", "x")).assertContain("x");
    });
    it("recordAndData", 0, () => {
      expect(recordAndData("record")).assertContain("count=");
    });
    it("utdEquals", 0, () => {
      expect(ranOrThrew((): boolean => utdEquals())).assertTrue();
    });
    it("smoke", 0, () => {
      expect(ranOrThrew((): string => smoke())).assertTrue();
    });
  });
}
