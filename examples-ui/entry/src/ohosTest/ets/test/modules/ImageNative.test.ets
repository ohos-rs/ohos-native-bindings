import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import { createAndInfo, readWritePixels, smoke, transform } from "libimage_native_test.so";

export default function imageNativeTest(): void {
  describe("image_native", () => {
    it("createAndInfo", 0, () => {
      expect(createAndInfo()).assertContain("w=");
    });
    it("transform", 0, () => {
      expect(transform()).assertContain("scaled=");
    });
    it("readWritePixels", 0, () => {
      expect(readWritePixels()).assertContain("read=");
    });
    it("smoke", 0, () => {
      expect(ranOrThrew((): string => smoke())).assertTrue();
    });
  });
}
