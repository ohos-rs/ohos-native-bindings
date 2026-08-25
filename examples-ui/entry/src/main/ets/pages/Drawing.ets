import { router } from "@kit.ArkUI";
import { smoke } from "libdrawing_test.so";

@Entry
@Component
struct Drawing {
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
      Text("Drawing")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text("Offscreen bitmap/canvas/path/pen/brush/matrix")
        .fontSize(12)
        .fontColor("#666666")
      Scroll() {
        Text(this.result)
          .fontSize(13)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
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
