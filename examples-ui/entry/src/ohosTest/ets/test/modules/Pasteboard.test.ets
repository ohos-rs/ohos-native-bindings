import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import {
  getPasteboardData,
  hasPasteboardData,
  hasPasteboardPlainText,
  isPasteboardRemoteData,
  setPasteboardData,
} from "libpasteboard_test.so";

export default function pasteboardTest(): void {
  describe("pasteboard", () => {
    // Reading system pasteboard data can be denied (201) when the data
    // belongs to a privacy-protected source; assert callability.
    it("write and query pasteboard", 0, () => {
      setPasteboardData("ohos-native-bindings-ohostest");
      expect(ranOrThrew((): boolean => hasPasteboardData())).assertTrue();
      expect(ranOrThrew((): boolean => hasPasteboardPlainText())).assertTrue();
      expect(ranOrThrew((): string => getPasteboardData())).assertTrue();
      expect(ranOrThrew((): boolean => isPasteboardRemoteData())).assertTrue();
    });
  });
}
