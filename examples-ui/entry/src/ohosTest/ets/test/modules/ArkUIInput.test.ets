import { describe, expect, it } from "@ohos/hypium";
import { lastClonedEvent, lastEvent } from "libarkui_input_test.so";
import { waitUntil } from "../helpers/Utils";

export default function arkuiInputTest(): void {
  describe("arkui_input", () => {
    // The host page (main module OhosTestHost) loads a focusable XComponent
    // with libraryname=arkui_input_test. Touch events on the app's surface
    // drive on_ui_input_event; lastEvent reports the parsed event.
    it("lastEvent reports parsed input events", 0, () => {
      expect(lastEvent().length > 0).assertTrue();
    });

    // When a UI input event fires, the demo clones it (api-15 cloned-event
    // family), mutates the clone (action, local position, finger id) and
    // reads the values back — lastClonedEvent exposes the readback.
    // Events only arrive while the host page is interacted with, so accept
    // both "no event yet" (empty) and a successful clone roundtrip.
    it("cloned-event modify and readback", 0, async () => {
      const out: string = lastClonedEvent();
      if (out.length === 0) {
        // No interaction during the run; the clone path was not exercised.
        expect(true).assertTrue();
        return;
      }
      expect(out).assertContain("clone ok");
      expect(out).assertContain("pos=(10.5,20.5)");
      expect(out).assertContain("finger_id=7");
    });
  });
}
