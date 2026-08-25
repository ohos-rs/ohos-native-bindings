import { router } from "@kit.ArkUI";
import { customTagLogs, debug, error, info, log, setGlobalTag, warn } from "libhilog_test.so";

@Entry
@Component
struct Hilog {
  @State fired: string = "none";

  private run(name: string, fn: () => void): void {
    this.fired = name;
    fn();
  }

  build() {
    Column({ space: 10 }) {
      Text("Hilog")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(`last: ${this.fired}`)
      Text("check device log: hdc shell hilog | grep hilog_rs / demo_hilog_tag")
        .fontSize(12)
        .fontColor("#666666")
      Row({ space: 6 }) {
        Button("debug")
          .onClick((): void => this.run("debug", (): void => debug()))
        Button("info")
          .onClick((): void => this.run("info", (): void => info()))
      }
      Row({ space: 6 }) {
        Button("warn")
          .onClick((): void => this.run("warn", (): void => warn()))
        Button("error")
          .onClick((): void => this.run("error", (): void => error()))
      }
      Button("log() — log crate redirect")
        .width("90%")
        .fontSize(13)
        .onClick((): void => this.run("log", (): void => log()))
      Button("customTagLogs() — instance logger custom tag/domain")
        .width("90%")
        .fontSize(12)
        .onClick((): void => this.run("customTagLogs", (): void => customTagLogs()))
      Button('setGlobalTag("my_demo_tag")')
        .width("90%")
        .fontSize(13)
        .onClick((): void => {
          this.run("setGlobalTag", (): void => setGlobalTag("my_demo_tag"));
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
