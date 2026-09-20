# Extended Native child-process example

This example tests extended Native children through separate FD and IPC fixtures.
Native sandbox and UID isolation do not select independent UIAbility/HAP
processes; those require component startup and lifecycle tests.

All blocking parent work runs on NAPI workers. `runProbe(forced)` and
`runConfiguredProbe(forced)` exchange named FDs and observe normal exit or kill.
`runBindingProbe(scenario)` exercises the binding API and real IPCKit requests.
`runInteropProbe(pid, kind)` checks children started from ArkTS.
`armParentExitProbe()` keeps four children alive for the parent-exit host test.

## Build and run

The default example enables API26, including the support query. Package the
shared library in an application allowed to create Native children.

```sh
ohrs build --arch aarch
```

From the repository root, with SDK 7.0 and hdc configured:

```sh
pnpm run ui:sync -- native_child_process
pnpm run test:ui -- native_child_process
```

## E2E scenarios

The suite exercises every public binding operation using actual process data,
FD communication, IPC requests and exit notifications.

| Binding surface | Coverage |
| --- | --- |
| `ChildProcessArgs`, `ChildProcessArgsRef`, `ChildProcessFd` | Empty/Unicode/150 KiB parameters, owned strings and moved args, zero/two/16 named FDs, 20-byte names, borrowed FD lifetime, capacity and NUL errors |
| `ChildProcessOptions`, `IsolationMode` | Normal/Isolated sandbox access and TCP communication |
| `ChildProcessConfigs` | Native defaults, all setters, UID isolation, process-name lifetime and limits, failed setter preservation, reuse and Drop |
| `start`, `start_with_configs` | PID, argument/FD transfer, concurrent and repeated launches, normal exit, kill, missing library/export and nested creation rejection |
| `create`, `create_with_configs` | Official OnConnect/MainProc exports, startup callback thread, Unicode IPC reply, proxy release, death notification, sandbox/UID configs, kill and startup errors |
| `current_args` | None in the parent and IPC-only child; entry and thread queries must match FD-child arguments |
| Exit callback registration/unregistration | Duplicate registration, multiple functions, selective removal, no delivery after unregister, missing callback and re-registration |
| `kill` | Native FD/IPC and ArkTS APP_SPAWN_FORK children, wrong/nonchild/stale PID rejection, SELF_FORK rejection |
| `is_supported` | API26 support query in the actual 2in1 application |
| `native_child_entry!` | Normal return, handler error, contained panic and panicking payload destructor, child SIGABRT without losing the parent |
| `NativeChildProcessError` | Preserved native codes, string conversion source, FD capacity error and borrowed FDs remaining open after failure |
| Parent lifetime | Four live children from all creation APIs; host sends SIGKILL only to the parent and requires all five PIDs to disappear within five seconds |

The runner executes 47 ordinary cases in `native_child_process_extended`, then
runs `native_child_process_parent_exit` separately. The parent-exit result is
recorded in `parent-exit-result.json` and included in the host summary.
Native allocation failure, service outage, startup timeout and resource
exhaustion need separate fault injection; they are not forced by this suite.

FD startup success returns a PID before loading the entry: a missing library or
export subsequently closes the transport and delivers exit. IPC loading errors
arrive through the startup callback with code 16010007 and a null proxy;
OnConnect returning null produces code 16010008 and a null proxy.

## QEMU CI

[Native child-process 2in1 E2E](../../.github/workflows/native-child-process-e2e.yml)
runs independently of phone E2E. Both use the shared
[QEMU runner](../../.github/workflows/qemu-e2e.yml), SDK 7.0 and release tag
`v20260919`, with separate concurrency groups and device/version cache keys.
CI builds API26 x64 examples and requires KVM.

The Native workflow prepares a private image with
`persist.sys.abilityms.multi_process_model=true` and
`const.max_native_child_process=50` in `appfwk.para` before boot, then checks
these values and `deviceType=2in1`. The cached release image remains unchanged.
HAPs are signed for the actual target UDID. Hypium results, parent-exit results
and Native process logs are uploaded as diagnostics; failures fail the workflow.

## Known image issues

The `v20260913` full 2in1 image, OpenHarmony-7.0.0.39/API26, was tested locally
with SDK 7.0 on arm64/HVF: **45 passed, 3 failed, 48 total**. All four creation
APIs passed parent-exit cleanup. This local run does not establish x64/KVM success.

| Failing case | Observed result |
| --- | --- |
| Current arguments in the FD child entry | None, although entry arguments and FD communication work |
| Current arguments from another FD child thread | None |
| Full 150 KiB parameter boundary | Native start returns 16010003; AppMS logs `Write param request failed` before reaching the entry |

None in the parent or IPC-only child is expected. None in the documented
FD-child query path remains a failing assertion, without an argument fallback.
The full parameter-boundary assertion also remains active.

[Official Native child guide](https://github.com/openharmony/docs/blob/master/zh-cn/application-dev/application-models/capi-nativechildprocess-development-guideline.md)
