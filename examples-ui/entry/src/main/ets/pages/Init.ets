import { router } from "@kit.ArkUI";
import { canIUse, smoke } from "libinit_test.so";

@Entry
@Component
struct Init {
  @State result: string = "(not run)";
  @State cap: string = "SystemCapability.Sensors.Sensor";

  aboutToAppear(): void {
    try {
      this.result = smoke();
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("Init / canIUse")
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
      TextInput({ text: this.cap, placeholder: "SystemCapability.*" })
        .width("90%")
        .onChange((value: string): void => {
          this.cap = value;
        })
      Button("canIUse(input)")
        .width("90%")
        .onClick((): void => {
          this.result = `${this.cap}=${canIUse(this.cap)}`;
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
