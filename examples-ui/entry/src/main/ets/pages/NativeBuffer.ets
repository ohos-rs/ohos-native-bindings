import { router } from "@kit.ArkUI";
import { allocAndWrite, smoke } from "libnative_buffer_test.so";

@Entry
@Component
struct NativeBuffer {
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
      Text("NativeBuffer")
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
      Button("allocAndWrite")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = allocAndWrite();
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
