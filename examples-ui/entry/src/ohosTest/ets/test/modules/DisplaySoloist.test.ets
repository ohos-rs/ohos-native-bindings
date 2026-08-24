import { describe, expect, it } from "@ohos/hypium";
import {
  setFrameRate,
  startDisplaySoloist,
  startExclusiveSoloist,
  stopDisplaySoloist,
  stopExclusiveSoloist,
} from "libdisplay_soloist_test.so";

export default function displaySoloistTest(): void {
  describe("display_soloist", () => {
    it("start and stop shared soloist", 0, () => {
      startDisplaySoloist();
      setFrameRate(30, 60, 30);
      stopDisplaySoloist();
      expect(true).assertTrue();
    });
    it("start and stop exclusive soloist", 0, () => {
      startExclusiveSoloist();
      stopExclusiveSoloist();
      expect(true).assertTrue();
    });
  });
}
