import { router } from "@kit.ArkUI";
import { addIme, addImeShowKeyboard, attach, detach, hide, show } from "libime_test.so";

@Entry
@Component
struct Ime {
  @State text: string = "";
  @State status: string = "tap addIme first, then focus the input field";

  build() {
    Column({ space: 10 }) {
      Text("IME")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(this.status)
        .fontSize(12)
        .fontColor("#666666")
      TextInput({ text: this.text, placeholder: "type here after attaching" })
        .width("90%")
        .onChange((value: string): void => {
          this.text = value;
        })
      Row({ space: 6 }) {
        Button("addIme")
          .onClick((): void => {
            addIme();
            this.status = "IME added (all callbacks) — events in hilog (tag hilog_rs)";
          })
        Button("add+showKb")
          .onClick((): void => {
            addImeShowKeyboard();
            this.status = "IME added with show-on-attach";
          })
      }
      Row({ space: 6 }) {
        Button("attach")
          .onClick((): void => {
            attach();
          })
        Button("detach")
          .onClick((): void => {
            detach();
          })
      }
      Row({ space: 6 }) {
        Button("show")
          .onClick((): void => {
            show();
          })
        Button("hide")
          .onClick((): void => {
            hide();
          })
      }
      Text(
        "callbacks: insert_text / pre_edit / status_change / delete / backspace / enter / preview",
      )
        .fontSize(10)
        .fontColor("#999999")
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
