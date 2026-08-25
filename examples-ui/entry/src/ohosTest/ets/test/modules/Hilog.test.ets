import { describe, expect, it } from "@ohos/hypium";
import { customTagLogs, debug, error, info, log, setGlobalTag, warn } from "libhilog_test.so";

export default function hilogTest(): void {
  describe("hilog", () => {
    it("all log levels are callable", 0, () => {
      debug();
      info();
      warn();
      error();
      log();
      customTagLogs();
      setGlobalTag("ohosTest_hilog");
      expect(true).assertTrue();
    });
  });
}
