import { describe, expect, it } from "@ohos/hypium";
import {
  bigintRoundtrip,
  bufferRoundtrip,
  evalInJsvm,
  exceptionMessage,
  externalRoundtrip,
  jsonRoundtrip,
  nativeMethodWithData,
  objectKeys,
  objectRoundtrip,
  promiseRoundtrip,
  typeTagRoundtrip,
  typedArrayRoundtrip,
  wrapRoundtrip,
} from "libjsvm_test.so";

export default function jsvmTest(): void {
  describe("jsvm", () => {
    it("evalInJsvm", 0, () => {
      expect(evalInJsvm("1 + 1")).assertContain("2");
    });
    it("objectRoundtrip", 0, () => {
      expect(objectRoundtrip()).assertContain("jsvm");
    });
    it("promiseRoundtrip", 0, () => {
      expect(promiseRoundtrip()).assertTrue();
    });
    it("typeTag and wrap roundtrip", 0, () => {
      expect(typeTagRoundtrip()).assertTrue();
      expect(wrapRoundtrip()).assertTrue();
    });
    it("objectKeys", 0, () => {
      expect(objectKeys().join(",")).assertEqual("alpha,beta");
    });
    it("buffer and typed array roundtrip", 0, () => {
      expect(`${bufferRoundtrip()}`).assertEqual("1,2,3,4");
      expect(`${typedArrayRoundtrip()}`).assertEqual("1,2,3");
    });
    it("bigintRoundtrip", 0, () => {
      const max: string = "340282366920938463463374607431768211455";
      expect(bigintRoundtrip(max)).assertEqual(max);
    });
    it("externalRoundtrip", 0, () => {
      expect(externalRoundtrip()).assertEqual("external-data");
    });
    it("jsonRoundtrip", 0, () => {
      expect(jsonRoundtrip()).assertContain("jsvm");
    });
    it("exceptionMessage", 0, () => {
      expect(exceptionMessage()).assertContain("boom");
    });
    it("nativeMethodWithData", 0, () => {
      expect(nativeMethodWithData()).assertEqual("native:ok");
    });
  });
}
