import { router } from "@kit.ArkUI";
import { image } from "@kit.ImageKit";
import { createPacker, createPixelMap, inspectPixelMap, smoke } from "libimage_test.so";

@Entry
@Component
struct ImageDemo {
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
      Text("Image (NAPI PixelMap)")
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
      Button("createPixelMap")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = createPixelMap();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("createPacker")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = createPacker();
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("inspect ArkTS PixelMap")
        .width("90%")
        .onClick((): void => {
          const color: ArrayBuffer = new ArrayBuffer(8 * 8 * 4);
          const pixelMap: image.PixelMap = image.createPixelMapSync(color, {
            size: { width: 8, height: 8 },
            srcPixelFormat: image.PixelMapFormat.RGBA_8888,
            pixelFormat: image.PixelMapFormat.RGBA_8888,
            editable: true,
          });
          try {
            this.result = inspectPixelMap(pixelMap);
          } catch (e) {
            this.result = `ERROR: ${e}`;
          } finally {
            pixelMap.release();
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
