import { describe, expect, it } from "@ohos/hypium";
import { fileIo } from "@kit.CoreFileKit";
import { appContext, ranOrThrew } from "../helpers/Utils";
import { cancel, customStart, start } from "libvibrator_test.so";

export default function vibratorTest(): void {
  describe("vibrator", () => {
    it("start and cancel", 0, () => {
      expect(ranOrThrew((): void => start())).assertTrue();
      cancel();
    });
    it("customStart from rawfile", 0, async () => {
      const content: Uint8Array =
        await appContext().resourceManager.getRawFileContent("coin_drop.json");
      const path: string = `${appContext().filesDir}/coin_drop.json`;
      const file: fileIo.File = fileIo.openSync(
        path,
        fileIo.OpenMode.READ_WRITE | fileIo.OpenMode.CREATE | fileIo.OpenMode.TRUNC,
      );
      fileIo.writeSync(file.fd, content.buffer);
      fileIo.closeSync(file);
      expect(ranOrThrew((): void => customStart(path))).assertTrue();
      cancel();
    });
  });
}
