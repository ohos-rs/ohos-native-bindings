import { describe, expect, it } from "@ohos/hypium";
import { smoke } from "libdrawing_test.so";

export default function drawingTest(): void {
  describe("drawing", () => {
    it("offscreen canvas", 0, () => {
      const out: string = smoke();
      expect(out).assertContain("bitmap=");
      expect(out).assertContain("canvas=");
    });
  });
}
