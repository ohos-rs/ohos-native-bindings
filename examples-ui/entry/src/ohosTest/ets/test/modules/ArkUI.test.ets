import { describe, expect, it } from "@ohos/hypium";
import { NodeContent } from "@kit.ArkUI";
import { MyApp, xcomponentGestureEvents } from "libarkui_test.so";
import { readCounter } from "../helpers/Utils";

export default function arkuiTest(): void {
  describe("arkui", () => {
    // aa test runs in a fresh process. Live gesture delivery is asserted by
    // inject-xcomponent-gestures.sh before Hypium starts; this assertion pins
    // the NAPI query's deterministic pre-mount state in the test process.
    it("gesture event query starts with empty counters", 0, () => {
      const status: string = xcomponentGestureEvents();
      expect(readCounter(status, "rawDown")).assertEqual(0);
      expect(readCounter(status, "rawMove")).assertEqual(0);
      expect(readCounter(status, "rawUp")).assertEqual(0);
      expect(readCounter(status, "tap")).assertEqual(0);
      expect(readCounter(status, "panAccept")).assertEqual(0);
      expect(readCounter(status, "panEnd")).assertEqual(0);
      expect(readCounter(status, "swipe")).assertEqual(0);
    });

    // The host page (main module OhosTestHost) hosts a NodeContent slot with
    // a live MyApp instance driving native ArkUI nodes. Constructing an
    // instance here verifies the binding surface; individual node methods
    // may throw when no ArkUI UIContext is attached to this runner.
    it("MyApp is constructible", 0, () => {
      expect(typeof MyApp === "function").assertTrue();
      let constructed: boolean = false;
      try {
        const app: MyApp = new MyApp(new NodeContent());
        constructed = true;
        app.destroyNativeNode();
      } catch (e) {
        console.info(`arkuiTest: ${e}`);
      }
      expect(constructed).assertTrue();
    });
  });
}
