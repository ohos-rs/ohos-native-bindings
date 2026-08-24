import { router } from "@kit.ArkUI";
import { currentQos, resetQos, setQos, smoke } from "libqos_test.so";

@Entry
@Component
struct Qos {
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
      Text("QoS")
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
        Button("UserInteractive")
          .onClick((): void => {
            try {
              setQos("UserInteractive");
              this.result = `set UserInteractive, now ${currentQos()}`;
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("Background")
          .onClick((): void => {
            try {
              setQos("Background");
              this.result = `set Background, now ${currentQos()}`;
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
      Row({ space: 6 }) {
        Button("current")
          .onClick((): void => {
            try {
              this.result = currentQos();
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("reset")
          .onClick((): void => {
            try {
              resetQos();
              this.result = "reset ok";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
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
