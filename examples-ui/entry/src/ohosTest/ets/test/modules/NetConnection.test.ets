import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import {
  allNetIds,
  currentProxyInfo,
  defaultNetInfo,
  defaultNetMetered,
  hasDefaultNet,
  resolveHost,
} from "libnet_connection_test.so";

export default function netConnectionTest(): void {
  describe("net_connection", () => {
    it("hasDefaultNet", 0, () => {
      expect(ranOrThrew((): boolean => hasDefaultNet())).assertTrue();
    });
    it("defaultNetInfo", 0, () => {
      expect(ranOrThrew((): string => defaultNetInfo())).assertTrue();
    });
    it("resolveHost", 0, () => {
      expect(ranOrThrew((): string => resolveHost("localhost"))).assertTrue();
    });
    it("allNetIds and currentProxyInfo", 0, () => {
      expect(ranOrThrew((): string => allNetIds())).assertTrue();
      expect(ranOrThrew((): string => currentProxyInfo())).assertTrue();
    });
    it("defaultNetMetered", 0, () => {
      expect(ranOrThrew((): boolean => defaultNetMetered())).assertTrue();
    });
    // googleVpnProbe is intentionally not called here: it performs a blocking
    // HTTPS request from the napi thread, which freezes the test runner main
    // thread long enough for the watchdog to kill the app. Cover it manually
    // from the demo page instead.
  });
}
