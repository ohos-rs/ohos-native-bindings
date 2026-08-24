import { describe, expect, it } from "@ohos/hypium";
import { restrictThenRead, roundtrip, smoke } from "libashmem_test.so";

export default function ashmemTest(): void {
  describe("ashmem", () => {
    it("roundtrip", 0, () => {
      expect(roundtrip("ohosTest")).assertContain("ohosTest");
    });
    it("restrictThenRead", 0, () => {
      expect(restrictThenRead()).assertContain("hello");
    });
    it("smoke", 0, () => {
      expect(smoke()).assertContain("ashmem-ok");
    });
  });
}
