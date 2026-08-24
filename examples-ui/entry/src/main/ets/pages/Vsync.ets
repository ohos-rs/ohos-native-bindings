import { router } from "@kit.ArkUI";
import {
  handleVsync,
  handleVsyncWithSelf,
  requestFrameOnce,
  vsyncPeriod,
  VsyncController,
} from "libvsync_test.so";
import { hilog } from "@kit.PerformanceAnalysisKit";

@Entry
@Component
struct Vsync {
  @State count: number = 0;
  @State controller: VsyncController | undefined = undefined;
  @State period: string = "(unknown)";
  @State status: string = "controller idle";

  private ensureController(): VsyncController {
    if (this.controller === undefined) {
      this.controller = new VsyncController();
    }
    return this.controller;
  }

  build() {
    Column({ space: 10 }) {
      Text("Vsync")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(`fired: ${this.count}  period: ${this.period}`)
      Text(this.status)
        .fontSize(12)
        .fontColor("#666666")
      Row({ space: 6 }) {
        Button("once")
          .onClick((): void => {
            this.count++;
            handleVsync();
          })
        Button("with_self")
          .onClick((): void => {
            this.count++;
            handleVsyncWithSelf();
          })
      }
      Row({ space: 6 }) {
        Button("requestFrameOnce")
          .onClick((): void => {
            const code: number = requestFrameOnce();
            this.status = `requestFrameOnce -> ${code}`;
            hilog.info(0x0, "VsyncPage", "requestFrameOnce ret=%{public}d", code);
          })
        Button("period")
          .onClick((): void => {
            this.period = `${vsyncPeriod()} ns`;
          })
      }
      Row({ space: 6 }) {
        Button("ctl.start")
          .onClick((): void => {
            const ret: number = this.ensureController().start();
            this.status = `controller started (${ret}) — frames in hilog`;
          })
        Button("ctl.once")
          .onClick((): void => {
            const ret: number = this.ensureController().once();
            this.status = `controller once (${ret})`;
          })
      }
      Row({ space: 6 }) {
        Button("ctl.stop")
          .onClick((): void => {
            this.ensureController().stop();
            this.status = "controller stopped";
          })
        Button("ctl.period")
          .onClick((): void => {
            this.period = `${this.ensureController().period()} ns`;
          })
      }
      Button("burst x50 (stability)")
        .width("90%")
        .fontSize(14)
        .onClick((): void => {
          for (let i = 0; i < 50; i++) {
            handleVsync();
          }
          this.count += 50;
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
