import { router } from "@kit.ArkUI";
import {
  getPasteboardData,
  hasPasteboardData,
  hasPasteboardPlainText,
  isPasteboardRemoteData,
  setPasteboardData,
} from "libpasteboard_test.so";

@Entry
@Component
struct Pasteboard {
  @State result: string = "(not run)";
  @State input: string = "hello from ohos-rs pasteboard demo";

  build() {
    Column({ space: 10 }) {
      Text("Pasteboard")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      TextInput({ text: this.input, placeholder: "text to write" })
        .width("90%")
        .onChange((value: string): void => {
          this.input = value;
        })
      Row({ space: 6 }) {
        Button("write")
          .onClick((): void => {
            try {
              setPasteboardData(this.input);
              this.result = "written — paste elsewhere to verify";
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("read")
          .onClick((): void => {
            try {
              this.result = getPasteboardData();
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
      }
      Row({ space: 6 }) {
        Button("hasData")
          .onClick((): void => {
            this.result = `has data: ${hasPasteboardData()}`;
          })
        Button("hasPlainText")
          .onClick((): void => {
            try {
              this.result = `has plain text: ${hasPasteboardPlainText()}`;
            } catch (e) {
              this.result = `ERROR: ${e}`;
            }
          })
        Button("isRemote")
          .onClick((): void => {
            this.result = `is remote data: ${isPasteboardRemoteData()}`;
          })
      }
      Scroll() {
        Text(this.result)
          .fontSize(14)
          .width("100%")
      }
      .width("92%")
      .layoutWeight(1)
      .align(Alignment.Top)
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
