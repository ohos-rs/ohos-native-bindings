import { router } from "@kit.ArkUI";
import { checkPermission, smoke } from "libability_access_control_test.so";

@Entry
@Component
struct AbilityAccess {
  @State result: string = "(not run)";

  aboutToAppear(): void {
    this.runSmoke();
  }

  private runSmoke(): void {
    try {
      this.result = smoke();
    } catch (e) {
      this.result = `ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("AbilityAccessControl")
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
      Button("smoke()")
        .width("90%")
        .onClick((): void => this.runSmoke())
      Button("check INTERNET")
        .width("90%")
        .onClick((): void => {
          this.result = `INTERNET=${checkPermission("ohos.permission.INTERNET")}`;
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
