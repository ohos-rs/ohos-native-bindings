import { describe, expect, it } from "@ohos/hypium";
import { smoke } from "libdisplay_test.so";

export default function displayTest(): void {
  describe("display", () => {
    it("default display metrics", 0, () => {
      const out: string = smoke();
      expect(out).assertContain("id=");
    });
  });
}
