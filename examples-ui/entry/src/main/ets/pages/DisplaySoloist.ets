import { router } from "@kit.ArkUI";
import {
  setFrameRate,
  startDisplaySoloist,
  startExclusiveSoloist,
  stopDisplaySoloist,
  stopExclusiveSoloist,
} from "libdisplay_soloist_test.so";

@Entry
@Component
struct DisplaySoloist {
  @State running: boolean = false;
  @State exclusiveRunning: boolean = false;
  @State note: string = "";

  build() {
    Column({ space: 10 }) {
      Text("DisplaySoloist")
        .fontSize(24)
        .fontWeight(FontWeight.Bold)
        .margin({ top: 24 })
      Text(this.running ? "shared: running (watch hilog: display_soloist)" : "shared: stopped")
      Row({ space: 6 }) {
        Button("start")
          .onClick((): void => {
            startDisplaySoloist();
            this.running = true;
          })
        Button("stop")
          .onClick((): void => {
            stopDisplaySoloist();
            this.running = false;
          })
      }
      Text(this.exclusiveRunning ? "exclusive: running" : "exclusive: stopped")
      Row({ space: 6 }) {
        Button("excl.start")
          .onClick((): void => {
            startExclusiveSoloist();
            this.exclusiveRunning = true;
          })
        Button("excl.stop")
          .onClick((): void => {
            stopExclusiveSoloist();
            this.exclusiveRunning = false;
          })
      }
      Row({ space: 6 }) {
        Button("rate 30fps")
          .onClick((): void => {
            setFrameRate(30, 30, 30);
            this.note = "expected rate set to 30";
          })
        Button("rate 60fps")
          .onClick((): void => {
            setFrameRate(60, 120, 90);
            this.note = "expected rate set to 90 (60-120)";
          })
      }
      Text(this.note)
        .fontSize(12)
        .fontColor("#666666")
      Button("start/stop x10 (stability)")
        .width("90%")
        .fontSize(14)
        .onClick((): void => {
          for (let i = 0; i < 10; i++) {
            startDisplaySoloist();
            stopDisplaySoloist();
          }
          this.running = false;
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
