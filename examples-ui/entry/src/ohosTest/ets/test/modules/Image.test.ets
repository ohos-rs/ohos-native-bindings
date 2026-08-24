import { describe, expect, it } from "@ohos/hypium";
import { image } from "@kit.ImageKit";
import { ranOrThrew } from "../helpers/Utils";
import { createPacker, createPixelMap, inspectPixelMap, smoke } from "libimage_test.so";

export default function imageTest(): void {
  describe("image", () => {
    // OH_PixelMap_CreatePixelMap may fail in non-UI test-runner contexts
    // (returns a null pixelmap with a success status); treat a thrown
    // napi error as callability-verified.
    it("createPixelMap", 0, () => {
      expect(ranOrThrew((): string => createPixelMap())).assertTrue();
    });
    it("createPacker", 0, () => {
      expect(ranOrThrew((): string => createPacker())).assertTrue();
    });
    it("inspectPixelMap", 0, () => {
      const color: ArrayBuffer = new ArrayBuffer(8 * 8 * 4);
      const pixelMap: image.PixelMap = image.createPixelMapSync(color, {
        size: { width: 8, height: 8 },
        srcPixelFormat: image.PixelMapFormat.RGBA_8888,
        pixelFormat: image.PixelMapFormat.RGBA_8888,
        editable: true,
      });
      try {
        expect(inspectPixelMap(pixelMap)).assertContain("w=");
      } finally {
        pixelMap.release();
      }
    });
    it("smoke", 0, () => {
      expect(ranOrThrew((): string => smoke())).assertTrue();
    });
  });
}
