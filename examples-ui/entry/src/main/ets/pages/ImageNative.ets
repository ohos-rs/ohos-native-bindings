import { router } from "@kit.ArkUI";
import { createAndInfo, readWritePixels, smoke, transform } from "libimage_native_test.so";

@Entry
@Component
struct ImageNative {
  @State result: string = "(not run)";

  aboutToAppear(): void {
    try {
      this.result = smoke();
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  private run(name: string, fn: () => string): void {
    try {
      this.result = `${name}: ${fn()}`;
    } catch (e) {
      this.result = `${name} ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("ImageNative")
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
      Button("createAndInfo")
        .width("90%")
        .onClick((): void => this.run("createAndInfo", createAndInfo))
      Button("transform")
        .width("90%")
        .onClick((): void => this.run("transform", transform))
      Button("readWritePixels")
        .width("90%")
        .onClick((): void => this.run("readWritePixels", readWritePixels))
      Button("smoke()")
        .width("90%")
        .onClick((): void => this.run("smoke", smoke))
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
