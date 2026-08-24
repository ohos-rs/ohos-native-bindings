import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew } from "../helpers/Utils";
import {
  cleartextPolicy,
  createRequest,
  headersRoundtrip,
  smoke,
  websocketConstruct,
} from "libnet_stack_test.so";

export default function netStackTest(): void {
  describe("net_stack", () => {
    it("headersRoundtrip", 0, () => {
      expect(headersRoundtrip()).assertContain("accept=");
    });
    it("createRequest", 0, () => {
      expect(createRequest("https://www.huawei.com")).assertContain("request_id=");
    });
    it("websocketConstruct", 0, () => {
      expect(ranOrThrew((): string => websocketConstruct())).assertTrue();
    });
    it("cleartextPolicy", 0, () => {
      expect(ranOrThrew((): string => cleartextPolicy())).assertTrue();
    });
    it("smoke", 0, () => {
      expect(ranOrThrew((): string => smoke())).assertTrue();
    });
  });
}
