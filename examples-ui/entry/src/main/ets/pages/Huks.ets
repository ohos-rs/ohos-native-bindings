import { router } from "@kit.ArkUI";
import { abortSession, hmacMac, rsaGenerateExport, smoke } from "libhuks_test.so";

@Entry
@Component
struct Huks {
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
      Text("HUKS")
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
      Button("hmacMac")
        .width("90%")
        .onClick((): void => this.run("hmacMac", (): string => hmacMac("hello-huks")))
      Button("rsaGenerateExport")
        .width("90%")
        .onClick((): void => this.run("rsaGenerateExport", rsaGenerateExport))
      Button("abortSession")
        .width("90%")
        .onClick((): void => this.run("abortSession", abortSession))
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
