import { router } from "@kit.ArkUI";
import { restrictThenRead, roundtrip, smoke } from "libashmem_test.so";

@Entry
@Component
struct Ashmem {
  @State result: string = "(not run)";

  aboutToAppear(): void {
    try {
      this.result = smoke();
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("Ashmem")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Scroll() {
        Text(this.result)
          .fontSize(13)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
      Button("roundtrip")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = roundtrip("hello ashmem");
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("restrictThenRead")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = restrictThenRead();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("smoke()")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = smoke();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("back")
        .width("90%")
        .onClick((): void => {
          router.back();
        })
    }
    .width("100%")
    .height("100%");
  }
}
