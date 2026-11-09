import { describe, expect, it } from "@ohos/hypium";
import { measureTextLineHeight, smoke } from "libdrawing_test.so";
import { TextLineHeightMetrics } from "../../types/drawing";

export default function drawingTest(): void {
  describe("drawing", () => {
    it("offscreen canvas", 0, () => {
      const out: string = smoke();
      expect(out).assertContain("bitmap=");
      expect(out).assertContain("canvas=");
    });
    it("half leading distributes extra line height equally", 0, () => {
      const normal: TextLineHeightMetrics = measureTextLineHeight(20, true);
      const taller: TextLineHeightMetrics = measureTextLineHeight(30, true);
      console.info(`half leading: ${JSON.stringify(normal)} -> ${JSON.stringify(taller)}`);
      expect(Math.abs(normal.height - 40) < 0.05).assertTrue();
      expect(Math.abs(taller.height - normal.height - 20) < 0.05).assertTrue();
      expect(Math.abs(taller.top - normal.top - 5) < 0.05).assertTrue();
      expect(Math.abs(taller.bottom - normal.bottom - 5) < 0.05).assertTrue();
      expect(Math.abs(normal.secondTop - normal.top - 20) < 0.05).assertTrue();
      expect(Math.abs(taller.secondTop - normal.secondTop - 15) < 0.05).assertTrue();
      expect(Math.abs(taller.width - normal.width) < 0.05).assertTrue();
    });
    it("half leading is opt-in", 0, () => {
      const original: TextLineHeightMetrics = measureTextLineHeight(20);
      const disabled: TextLineHeightMetrics = measureTextLineHeight(20, false);
      const enabled: TextLineHeightMetrics = measureTextLineHeight(20, true);
      console.info(
        `leading default: ${JSON.stringify(original)}, enabled: ${JSON.stringify(enabled)}`,
      );
      expect(Math.abs(disabled.top - original.top) < 0.05).assertTrue();
      expect(Math.abs(disabled.bottom - original.bottom) < 0.05).assertTrue();
      expect(Math.abs(disabled.height - enabled.height) < 0.05).assertTrue();
      expect(Math.abs(disabled.width - enabled.width) < 0.05).assertTrue();
      expect(disabled.top - enabled.top > 1).assertTrue();
    });
  });
}
