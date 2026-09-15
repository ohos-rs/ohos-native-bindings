import { ChildProcess } from "@kit.AbilityKit";
import { fileIo } from "@kit.CoreFileKit";

// startChildProcess only supports synchronous APIs. In particular SELF_FORK
// cannot use Binder IPC, so this fixture uses a file and a bounded local loop.
export default class BindingInteropProcess extends ChildProcess {
  onStart(): void {
    // procfs files report size zero; read the PID directly rather than using a
    // text-file helper that sizes its buffer from stat().
    const stat = fileIo.openSync("/proc/self/stat", fileIo.OpenMode.READ_ONLY);
    const buffer: ArrayBuffer = new ArrayBuffer(32);
    fileIo.readSync(stat.fd, buffer);
    fileIo.closeSync(stat);
    let pid: string = "";
    for (const byte of new Uint8Array(buffer)) {
      if (byte === 32) {
        break;
      }
      pid += String.fromCharCode(byte);
    }
    const path: string = `/data/storage/el2/base/haps/entry/files/binding_interop_${pid}`;
    const file = fileIo.openSync(
      path,
      fileIo.OpenMode.CREATE | fileIo.OpenMode.WRITE_ONLY | fileIo.OpenMode.TRUNC,
    );
    fileIo.writeSync(file.fd, "ready");
    fileIo.closeSync(file);
    const until: number = Date.now() + 3000;
    while (Date.now() < until) {
      // Keep this child alive while its parent checks the native kill contract.
    }
    const done = fileIo.openSync(path, fileIo.OpenMode.WRITE_ONLY | fileIo.OpenMode.TRUNC);
    fileIo.writeSync(done.fd, "returned");
    fileIo.closeSync(done);
  }
}
