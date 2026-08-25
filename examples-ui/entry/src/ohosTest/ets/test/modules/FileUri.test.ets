import { describe, expect, it } from "@ohos/hypium";
import { writeTemp } from "../helpers/Utils";
import {
  directoryUri,
  fileName,
  pathFromUri,
  smoke,
  uriFromPath,
  validUri,
} from "libfileuri_test.so";

export default function fileUriTest(): void {
  describe("fileuri", () => {
    it("path uri roundtrip", 0, () => {
      const path: string = writeTemp("fileuri_ohostest.txt", "hi");
      const uri: string = uriFromPath(path);
      expect(validUri(uri)).assertTrue();
      expect(validUri("not a uri")).assertFalse();
      expect(pathFromUri(uri).length > 0).assertTrue();
      expect(directoryUri(uri).length > 0).assertTrue();
      expect(fileName(uri)).assertContain("fileuri_ohostest.txt");
    });
    it("smoke", 0, () => {
      const path: string = writeTemp("fileuri_smoke_ohostest.txt", "hi");
      const out: string = smoke(path);
      expect(out).assertContain("uri=file://");
      expect(out).assertContain("valid_uri=true");
      expect(out).assertContain("valid_garbage=false");
    });
  });
}
