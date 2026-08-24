import { router } from "@kit.ArkUI";
import {
  cleartextPolicy,
  createRequest,
  headersRoundtrip,
  smoke,
  websocketConstruct,
} from "libnet_stack_test.so";

@Entry
@Component
struct NetStack {
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
      Text("NetStack")
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
      Button("headersRoundtrip")
        .width("90%")
        .onClick((): void => this.run("headers", headersRoundtrip))
      Button("createRequest")
        .width("90%")
        .onClick((): void =>
          this.run("request", (): string => createRequest("https://www.huawei.com")),
        )
      Button("cleartextPolicy")
        .width("90%")
        .onClick((): void => this.run("cleartext", cleartextPolicy))
      Button("websocketConstruct")
        .width("90%")
        .onClick((): void => this.run("websocket", websocketConstruct))
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
