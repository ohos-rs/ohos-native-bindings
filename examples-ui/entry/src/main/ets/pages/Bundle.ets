import { router } from "@kit.ArkUI";
import {
  appId,
  appIdentifier,
  compatibleDeviceType,
  currentBundleInfo,
  mainElementName,
  smoke,
} from "libbundle_test.so";

@Entry
@Component
struct Bundle {
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
      this.result = `${name}\n${fn()}`;
    } catch (e) {
      this.result = `${name} ERROR: ${e}`;
    }
  }

  build() {
    Column({ space: 10 }) {
      Text("Bundle")
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
        Button("info")
          .onClick((): void => this.run("currentBundleInfo", currentBundleInfo))
        Button("appId")
          .onClick((): void => this.run("appId", appId))
      }
      Row({ space: 6 }) {
        Button("identifier")
          .onClick((): void => this.run("appIdentifier", appIdentifier))
        Button("mainElement")
          .onClick((): void => this.run("mainElementName", mainElementName))
      }
      Button("compatibleDeviceType")
        .width("90%")
        .onClick((): void => this.run("compatibleDeviceType", compatibleDeviceType))
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
