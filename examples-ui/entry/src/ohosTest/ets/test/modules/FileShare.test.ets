import { describe, expect, it } from "@ohos/hypium";
import { ranOrThrew, writeTemp } from "../helpers/Utils";
import { uriFromPath } from "libfileuri_test.so";
import {
  activate,
  checkPersistent,
  deactivate,
  persist,
  revoke,
  smoke,
} from "libfileshare_test.so";

export default function fileShareTest(): void {
  describe("fileshare", () => {
    it("policy APIs are callable", 0, () => {
      const path: string = writeTemp("fileshare_ohostest.txt", "hi");
      expect(smoke(path)).assertContain("uri=");
    });
    it("each policy API is callable", 0, () => {
      const path: string = writeTemp("fileshare_each_ohostest.txt", "hi");
      const uri: string = uriFromPath(path);
      expect(ranOrThrew((): string => persist(uri))).assertTrue();
      expect(ranOrThrew((): string => activate(uri))).assertTrue();
      expect(ranOrThrew((): string => deactivate(uri))).assertTrue();
      expect(ranOrThrew((): string => checkPersistent(uri))).assertTrue();
      expect(ranOrThrew((): string => revoke(uri))).assertTrue();
    });
  });
}
