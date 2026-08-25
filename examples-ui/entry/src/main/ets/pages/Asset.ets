import { router } from "@kit.ArkUI";
import { addSecret, querySecret, removeSecret, smoke, updateSecret } from "libasset_test.so";

@Entry
@Component
struct Asset {
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
      Text("Asset")
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
      Row({ space: 6 }) {
        Button("add")
          .onClick((): void => {
            this.result = addSecret("first-secret");
          })
        Button("query")
          .onClick((): void => {
            this.result = querySecret();
          })
      }
      Row({ space: 6 }) {
        Button("update")
          .onClick((): void => {
            this.result = updateSecret("second-secret");
          })
        Button("remove")
          .onClick((): void => {
            this.result = removeSecret();
          })
      }
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
