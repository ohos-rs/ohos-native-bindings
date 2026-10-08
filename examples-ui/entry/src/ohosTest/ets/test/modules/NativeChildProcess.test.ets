import { describe, expect, it } from "@ohos/hypium";
import { childProcessManager } from "@kit.AbilityKit";
import {
  armParentExitProbe,
  runBindingProbe,
  runConfiguredProbe,
  runInteropProbe,
  runProbe,
} from "libnative_child_process_example.so";

function field(report: string, key: string): string {
  const prefix: string = `${key}=`;
  for (const part of report.split("; ")) {
    if (part.startsWith(prefix)) {
      return part.substring(prefix.length);
    }
  }
  throw new Error(`Missing ${key} in native probe result: ${report}`);
}

function assertExtendedChild(report: string, forced: boolean, configured: boolean): void {
  console.info(`[native_child_process_extended] ${report}`);
  const parentPid: number = Number(field(report, "parent_pid"));
  const childPid: number = Number(field(report, "child_pid"));
  expect(Number.isInteger(parentPid) && parentPid > 0).assertTrue();
  expect(Number.isInteger(childPid) && childPid > 0 && childPid !== parentPid).assertTrue();
  expect(field(report, "named_fds")).assertEqual("2");
  expect(field(report, "params")).assertEqual("binding-probe-v1");
  expect(field(report, "echo")).assertEqual("matched");
  expect(field(report, "eof")).assertEqual("true");
  expect(field(report, "mode")).assertEqual("normal");
  expect(field(report, "forced")).assertEqual(forced ? "true" : "false");
  expect(field(report, "configured")).assertEqual(configured ? "true" : "false");
  const signal: number = Number(field(report, "exit_signal"));
  expect(Number.isInteger(signal)).assertTrue();
  if (forced) {
    expect(signal > 0).assertTrue();
  } else {
    expect(signal).assertEqual(0);
  }
  if (configured) {
    expect(field(report, "process_name").endsWith(":native_child_process_probe")).assertTrue();
  }
}

async function bindingScenario(scenario: string): Promise<string> {
  const report: string = await runBindingProbe(scenario);
  console.info(`[native_child_process_binding] ${report}`);
  expect(field(report, "scenario")).assertEqual(scenario);
  return report;
}

function assertFdScenario(report: string, sandbox: string, signal: number): void {
  expect(field(report, "sandbox")).assertEqual(sandbox);
  expect(Number(field(report, "exit_signal"))).assertEqual(signal);
  expect(field(report, "eof")).assertEqual("true");
  expect(field(report, "borrowed_fds")).assertEqual("open");
}

function assertIpcScenario(report: string, sandbox: string): void {
  expect(field(report, "startup_code")).assertEqual("0");
  expect(field(report, "sandbox")).assertEqual(sandbox);
  expect(field(report, "callback_thread")).assertEqual("independent");
  expect(field(report, "ipc")).assertEqual("matched");
  expect(field(report, "remote_dead")).assertEqual("true");
  expect(field(report, "proxy")).assertEqual("released");
  expect(field(report, "fd_exit_callbacks")).assertEqual("0");
}

export default function nativeChildProcessTest(): void {
  // These probes call the Native child launch APIs in Normal sandbox mode.
  // Independent UIAbility/HAP process placement requires a component fixture.
  describe("native_child_process_extended", () => {
    it("graceful exit transfers arguments and two named FDs", 0, async () => {
      assertExtendedChild(await runProbe(false), false, false);
    });
    it("current argument query matches the native entry arguments and FDs", 0, async () => {
      const report: string = await runProbe(false);
      assertExtendedChild(report, false, false);
      expect(field(report, "current_args")).assertEqual("matched");
    });
    it("forced termination reports child exit and closes transport", 0, async () => {
      assertExtendedChild(await runProbe(true), true, false);
    });
    it("native configs launch a child with the assigned process name", 0, async () => {
      assertExtendedChild(await runConfiguredProbe(false), false, true);
    });
    it("callbacks and descriptors remain usable across repeated launches", 0, async () => {
      assertExtendedChild(await runProbe(false), false, false);
      assertExtendedChild(await runProbe(true), true, false);
      assertExtendedChild(await runProbe(false), false, false);
    });
    it("API26 support query permits this application and parent has no child args", 0, async () => {
      const report: string = await bindingScenario("support");
      expect(field(report, "supported")).assertEqual("true");
      expect(field(report, "parent_args")).assertEqual("none");
    });
    it(
      "all string inputs and native parameter boundaries preserve errors and borrowed FDs",
      0,
      async () => {
        const report: string = await bindingScenario("boundaries");
        expect(field(report, "nul_sites")).assertEqual("7");
        expect(field(report, "invalid_param_code")).assertEqual("401");
        expect(field(report, "overflow")).assertEqual("TooManyFileDescriptors");
        expect(field(report, "invalid_pid_code")).assertEqual("16010010");
        expect(field(report, "borrowed_fds")).assertEqual("open");
      },
    );
    it("default arguments transfer an empty string and zero FDs", 0, async () => {
      const report: string = await bindingScenario("empty");
      expect(field(report, "empty")).assertEqual("true");
      expect(field(report, "named_fds")).assertEqual("0");
      expect(field(report, "exit_signal")).assertEqual("0");
    });
    it("empty entry parameters transfer alongside descriptors", 0, async () => {
      const report: string = await bindingScenario("params_empty");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "params_bytes")).assertEqual("0");
    });
    it("Unicode parameters and descriptor names survive native transfer", 0, async () => {
      assertFdScenario(await bindingScenario("params_unicode"), "shared", 0);
    });
    it("documented 150 KiB entry parameter boundary transfers without truncation", 0, async () => {
      const report: string = await bindingScenario("params_limit");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "params_bytes")).assertEqual("153600");
    });
    it("all sixteen named descriptors route correctly and remain borrowed", 0, async () => {
      const report: string = await bindingScenario("fds_16");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "named_fds")).assertEqual("16");
    });
    it("options select an isolated Native sandbox with the default shared UID", 0, async () => {
      const report: string = await bindingScenario("isolated_options");
      assertFdScenario(report, "isolated", 0);
      expect(field(report, "child_uid")).assertEqual(field(report, "parent_uid"));
    });
    it("native configs defaults retain the normal sandbox and shared UID", 0, async () => {
      assertFdScenario(await bindingScenario("configs_defaults"), "shared", 0);
    });
    it("Normal child can exchange TCP data with the parent network environment", 0, async () => {
      const report: string = await bindingScenario("network_normal");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "network")).assertEqual("shared");
    });
    it(
      "Isolated child cannot reach a TCP listener in the parent network environment",
      0,
      async () => {
        const report: string = await bindingScenario("network_isolated");
        assertFdScenario(report, "isolated", 0);
        expect(field(report, "network")).assertEqual("isolated");
      },
    );
    it("UID isolation setting has no effect in Normal mode", 0, async () => {
      const report: string = await bindingScenario("configs_normal_uid_true");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "child_uid")).assertEqual(field(report, "parent_uid"));
    });
    it("isolated configs can retain the parent UID", 0, async () => {
      const report: string = await bindingScenario("configs_isolated_uid_false");
      assertFdScenario(report, "isolated", 0);
      expect(field(report, "child_uid")).assertEqual(field(report, "parent_uid"));
    });
    it("isolated configs can allocate an independent UID", 0, async () => {
      const report: string = await bindingScenario("configs_isolated_uid_true");
      assertFdScenario(report, "isolated", 0);
      expect(field(report, "child_uid") !== field(report, "parent_uid")).assertTrue();
    });
    it("configs launched child can be killed with SIGKILL", 0, async () => {
      assertFdScenario(await bindingScenario("configs_kill"), "shared", 9);
    });
    it(
      "configs survive moves reuse name replacement failed setters and Drop after launch",
      0,
      async () => {
        const report: string = await bindingScenario("configs_reuse");
        expect(field(report, "configs_reused")).assertEqual("2");
        expect(field(report, "failed_setter")).assertEqual("preserved");
        expect(field(report, "process_name").endsWith(":reused_name")).assertTrue();
      },
    );
    it(
      "native callback registration deduplicates and unregisters only the requested function",
      0,
      async () => {
        const report: string = await bindingScenario("callbacks");
        expect(field(report, "deduplicated")).assertEqual("true");
        expect(field(report, "multiple_callbacks")).assertEqual("2");
        expect(field(report, "unregistered_delivery")).assertEqual("0");
        expect(field(report, "missing_callback_code")).assertEqual("16010009");
        expect(field(report, "reregistered")).assertEqual("true");
      },
    );
    it(
      "three simultaneously live children have separate PIDs transports and exit events",
      0,
      async () => {
        const report: string = await bindingScenario("concurrent");
        expect(field(report, "simultaneous_children")).assertEqual("3");
        expect(field(report, "callbacks")).assertEqual("3");
        expect(field(report, "eof")).assertEqual("true");
      },
    );
    it("kill rejects a child PID after its exit has been observed", 0, async () => {
      const report: string = await bindingScenario("kill_stale");
      expect(field(report, "stale_pid_code")).assertEqual("16010010");
      expect(field(report, "exited")).assertEqual("true");
    });
    it("entry handler error returns safely across the official ABI", 0, async () => {
      assertFdScenario(await bindingScenario("entry_error"), "shared", 0);
    });
    it("entry handler panic is contained before returning to native", 0, async () => {
      assertFdScenario(await bindingScenario("entry_panic"), "shared", 0);
    });
    it("panic payload destructor cannot unwind across the native entry", 0, async () => {
      assertFdScenario(await bindingScenario("entry_payload_panic"), "shared", 0);
    });
    it("child crash reports SIGABRT without terminating its parent", 0, async () => {
      assertFdScenario(await bindingScenario("entry_abort"), "shared", 6);
    });
    it("all four creation APIs reject nested Native child creation", 0, async () => {
      const report: string = await bindingScenario("nested");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "nested_code")).assertEqual("16010005");
    });
    it("FD start missing library closes transport after child loading fails", 0, async () => {
      const report: string = await bindingScenario("fd_missing_library");
      expect(field(report, "launch")).assertEqual("acknowledged");
      expect(field(report, "entry")).assertEqual("not_called");
      expect(field(report, "eof")).assertEqual("true");
    });
    it("FD start missing entry export exits without calling the entry", 0, async () => {
      const report: string = await bindingScenario("fd_missing_export");
      expect(field(report, "entry")).assertEqual("not_called");
      expect(field(report, "exit_signal")).assertEqual("0");
    });
    it("FD configs start missing library closes transport after loading fails", 0, async () => {
      const report: string = await bindingScenario("fd_configs_missing_library");
      expect(field(report, "launch")).assertEqual("acknowledged");
      expect(field(report, "entry")).assertEqual("not_called");
      expect(field(report, "eof")).assertEqual("true");
    });
    it("FD configs start missing export exits without calling the entry", 0, async () => {
      const report: string = await bindingScenario("fd_configs_missing_export");
      expect(field(report, "entry")).assertEqual("not_called");
      expect(field(report, "exit_signal")).assertEqual("0");
    });
    it("current argument query is available from another child thread", 0, async () => {
      const report: string = await bindingScenario("query_thread");
      assertFdScenario(report, "shared", 0);
      expect(field(report, "thread_args")).assertEqual("matched");
    });
    it("IPC create receives a proxy performs requests and ends on MainProc return", 0, async () => {
      assertIpcScenario(await bindingScenario("ipc_normal"), "shared");
    });
    it("IPC create with configs retains its name after the configs owner drops", 0, async () => {
      assertIpcScenario(await bindingScenario("ipc_configs_normal"), "shared");
    });
    it("IPC Normal configs ignore UID isolation", 0, async () => {
      const report: string = await bindingScenario("ipc_configs_normal_uid_true");
      assertIpcScenario(report, "shared");
      expect(field(report, "child_uid")).assertEqual(field(report, "parent_uid"));
    });
    it("IPC isolated configs retain UID when requested", 0, async () => {
      const report: string = await bindingScenario("ipc_configs_isolated_uid_false");
      assertIpcScenario(report, "isolated");
      expect(field(report, "child_uid")).assertEqual(field(report, "parent_uid"));
    });
    it("IPC isolated configs allocate an independent UID when requested", 0, async () => {
      const report: string = await bindingScenario("ipc_configs_isolated_uid_true");
      assertIpcScenario(report, "isolated");
      expect(field(report, "child_uid") !== field(report, "parent_uid")).assertTrue();
    });
    it("kill accepts an IPC child and its remote object becomes dead", 0, async () => {
      assertIpcScenario(await bindingScenario("ipc_kill"), "shared");
    });
    it(
      "IPC nonexistent library reports loading failure through the startup callback",
      0,
      async () => {
        const report: string = await bindingScenario("ipc_missing");
        expect(field(report, "startup_code")).assertEqual("16010007");
        expect(field(report, "proxy")).assertEqual("null");
      },
    );
    it(
      "IPC configs nonexistent library reports loading failure through the callback",
      0,
      async () => {
        const report: string = await bindingScenario("ipc_configs_missing");
        expect(field(report, "startup_code")).assertEqual("16010007");
        expect(field(report, "proxy")).assertEqual("null");
      },
    );
    it("IPC library missing official exports reports loading failure", 0, async () => {
      const report: string = await bindingScenario("ipc_missing_exports");
      expect(field(report, "startup_code")).assertEqual("16010007");
      expect(field(report, "proxy")).assertEqual("null");
    });
    it("IPC OnConnect returning null reports connection failure", 0, async () => {
      const report: string = await bindingScenario("ipc_null_connect");
      expect(field(report, "startup_code")).assertEqual("16010008");
      expect(field(report, "proxy")).assertEqual("null");
    });
    it("Rust kill and exit callback work for an ArkTS-started Native child", 0, async () => {
      const pid: number = await childProcessManager.startNativeChildProcess(
        "libnative_child_process_example.so:BindingHoldMain",
        { entryParams: "", fds: {} },
      );
      const report: string = await runInteropProbe(pid, "native");
      console.info(`[native_child_process_binding] ${report}`);
      expect(field(report, "kind")).assertEqual("native");
      expect(field(report, "kill_code")).assertEqual("0");
      expect(field(report, "native_exit_callbacks")).assertEqual("1");
    });
    it(
      "Rust kill accepts an APP_SPAWN_FORK ArkTS child without Native exit notification",
      0,
      async () => {
        const pid: number = await childProcessManager.startChildProcess(
          "./ets/process/BindingInteropProcess.ets",
          childProcessManager.StartMode.APP_SPAWN_FORK,
        );
        const report: string = await runInteropProbe(pid, "ark_spawn");
        console.info(`[native_child_process_binding] ${report}`);
        expect(field(report, "kill_code")).assertEqual("0");
        expect(field(report, "native_exit_callbacks")).assertEqual("0");
      },
    );
    it(
      "Rust kill rejects SELF_FORK while allowing the child entry to return normally",
      0,
      async () => {
        const pid: number = await childProcessManager.startChildProcess(
          "./ets/process/BindingInteropProcess.ets",
          childProcessManager.StartMode.SELF_FORK,
        );
        const report: string = await runInteropProbe(pid, "self_fork");
        console.info(`[native_child_process_binding] ${report}`);
        expect(field(report, "kill_code")).assertEqual("16010010");
        expect(field(report, "exited")).assertEqual("true");
        expect(field(report, "native_exit_callbacks")).assertEqual("0");
      },
    );
  });
  describe("native_child_process_parent_exit", () => {
    it("host observes cleanup of all four creation APIs after parent termination", 0, async () => {
      const report: string = await armParentExitProbe();
      console.info(`[native_child_process_parent_exit] ${report}`);
      // The host must kill this process and verify the four observed PIDs.
      await new Promise<void>((resolve) => setTimeout(resolve, 15000));
      expect(false).assertTrue();
    });
  });
}
