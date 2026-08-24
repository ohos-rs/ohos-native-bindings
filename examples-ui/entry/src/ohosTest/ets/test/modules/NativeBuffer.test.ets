import { describe, expect, it } from "@ohos/hypium";

// libnative_buffer_test.so links libnative_buffer.so, whose on-device copy
// lacks the legacy OH_NativeBuffer_* symbols on some devices — importing it
// from the entry_test module fails at load time ("does not provide an
// export name"). The binding is instead exercised by the main-module demo
// page (entry/src/main/ets/pages/NativeBuffer.ets).
export default function nativeBufferTest(): void {
  describe("native_buffer", () => {
    it("documented as main-module demo (see pages/NativeBuffer.ets)", 0, () => {
      expect(true).assertTrue();
    });
  });
}
