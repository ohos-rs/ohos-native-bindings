import { router } from "@kit.ArkUI";
import { NodeContent } from "@kit.ArkUI";
import { MyApp } from "libarkui_test.so";

@Entry
@Component
struct ArkUI {
  private content: NodeContent = new NodeContent();
  private app: MyApp | undefined = undefined;

  private ensureApp(): MyApp {
    if (this.app === undefined) {
      this.app = new MyApp(this.content);
    }
    return this.app;
  }

  build() {
    Column({ space: 10 }) {
      Column() {
        ContentSlot(this.content)
      }
      .width("100%")
      .layoutWeight(1)
      Row({ space: 8 }) {
        Button("create")
          .onClick((): void => {
            this.ensureApp().createNativeNode();
          })
        Button("input")
          .onClick((): void => {
            this.ensureApp().createTextInput();
          })
        Button("gallery")
          .onClick((): void => {
            this.ensureApp().createGallery();
          })
      }
      Row({ space: 8 }) {
        Button("animate")
          .onClick((): void => {
            this.app?.animation(this.getUIContext());
          })
        Button("dialog")
          .onClick((): void => {
            this.app?.showDialog();
          })
        Button("destroy")
          .onClick((): void => {
            this.app?.destroyNativeNode();
          })
      }
      Text(
        "gallery: buttons/slider/toggle/progress/checkbox/radio/swiper/stack + pan/swipe/pinch/rotation gestures",
      )
        .fontSize(10)
        .fontColor("#999999")
      Button("create + destroy x20 (stability)")
        .width("90%")
        .fontSize(14)
        .onClick((): void => {
          for (let i = 0; i < 20; i++) {
            const app: MyApp = new MyApp(this.content);
            app.createNativeNode();
            app.destroyNativeNode();
          }
        })
      Button("back")
        .width("90%")
        .onClick((): void => {
          router.back();
        })
    }
    .width("100%")
    .height("100%")
    .padding(12);
  }
}
