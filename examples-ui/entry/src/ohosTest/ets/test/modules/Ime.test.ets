import { describe, expect, it } from "@ohos/hypium";
import { addIme, addImeShowKeyboard, attach, detach, hide, show } from "libime_test.so";

export default function imeTest(): void {
  describe("ime", () => {
    it("hide detach without attached ime", 0, () => {
      hide();
      detach();
      expect(true).assertTrue();
    });
    it("add ime then show attach hide detach", 0, () => {
      addIme();
      show();
      attach();
      hide();
      detach();
      expect(true).assertTrue();
    });
    it("add ime with keyboard shown then detach", 0, () => {
      addImeShowKeyboard();
      hide();
      detach();
      expect(true).assertTrue();
    });
  });
}
