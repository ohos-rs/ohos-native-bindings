import { router } from "@kit.ArkUI";
import {
  htmlRoundtrip,
  plainTextRoundtrip,
  recordAndData,
  smoke,
  utdEquals,
} from "libudmf_test.so";

@Entry
@Component
struct Udmf {
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
      Text("UDMF")
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
      Button("plainTextRoundtrip")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = plainTextRoundtrip("hello udmf");
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("htmlRoundtrip")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = htmlRoundtrip("<b>x</b>", "x");
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("recordAndData")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = recordAndData("record");
          } catch (e) {
            this.result = `ERROR: ${e}`;
          }
        })
      Button("utdEquals")
        .width("90%")
        .onClick((): void => {
          try {
            this.result = `utdEquals=${utdEquals()}`;
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
