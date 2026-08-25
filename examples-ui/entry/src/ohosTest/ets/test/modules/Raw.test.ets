import { describe, expect, it } from "@ohos/hypium";
import { appContext } from "../helpers/Utils";
import { isRawDir, openRawDir, readRawFile, seekRawFile } from "libraw_test.so";

export default function rawTest(): void {
  describe("raw", () => {
    it("rawfile dir and read", 0, () => {
      const manager = appContext().resourceManager;
      expect(isRawDir(manager, "rawdir")).assertTrue();
      openRawDir(manager, "rawdir");
      expect(readRawFile(manager, "rawdir/hello.txt")).assertContain("size:");
      expect(seekRawFile(manager, "rawdir/hello.txt")).assertContain("size:");
    });
  });
}
