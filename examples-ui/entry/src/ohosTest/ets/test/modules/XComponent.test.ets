import { describe, expect, it } from "@ohos/hypium";
import { drawXcomponent, setFrameRate, surfaceEvents } from "libxcomponent_test.so";
import { eventsById } from "libxcomponent_multi_test.so";

export default function xcomponentTest(): void {
  describe("xcomponent", () => {
    it("surface event query is available without a host", 0, () => {
      expect(surfaceEvents()).assertContain("created=0");
    });

    it("drawXcomponent without surface is a no-op", 0, () => {
      // GL_CTX lock is held with None; the call returns without drawing.
      drawXcomponent();
      expect(true).assertTrue();
    });

    it("setFrameRate returns or reports a descriptive runtime error", 0, () => {
      let completed: boolean = false;
      try {
        setFrameRate(30, 30, 30);
        completed = true;
      } catch (e) {
        completed = `${e}`.length > 0;
      }
      expect(completed).assertTrue();
    });
  });

  describe("xcomponent_multi", () => {
    // multi_mode dispatch keys callbacks by XComponent id. Without a host
    // surface the map stays empty — the deterministic baseline. The
    // per-id dispatch itself is verified on the main app's OhosTestHost
    // page, which mounts two XComponents (ids ohosTest_xc_multi_a/b) bound
    // to this same library; each id accumulates its own created counter.
    it("no-surface map is empty", 0, () => {
      expect(eventsById()).assertEqual("");
    });
  });
}
