# Normal Native child two-FD probe

Unpublished, bindings-owned source example. No application/engine dependency,
shell executable, signing material, independent-process configuration or
isolated child mode is used. Device results are pending.

## Build and run entry

With the repository's pinned OHOS toolchain and SDK configured:

```sh
cd examples/native_child_process
ohrs build --arch aarch
```

This builds an API22-feature parent/child library:
`dist/arm64-v8a/libnative_child_process_example.so`, exporting
`BindingProbeMain` with the binding entry macro. The same packaged library is
loaded in the parent as a NAPI module and in a distinct child by AbilityKit.
Generated `dist/index.d.ts` exposes `runProbe(forced: boolean): Promise<string>`.
All synchronous start/socket/exit work executes on a NAPI libuv worker, not UI.

For a separate API26 capability-query probe (never substitute this artifact for
API22 compatibility evidence):

```sh
ohrs build --arch aarch --dist dist/api26 -- --no-default-features --features api-26
```

After independent source review, a device-probe task must package the exact
library/type declaration in an explicitly selected, signed feasibility HAP.
The HAP must declare the supported device type/API and include the library in
its `arm64-v8a` native directory; protected local signing is injected outside
tracked files. Do not add Ability/module `process`, `isolationProcess` or
independent `isolationMode` configuration. Call from the signed parent:

```ts
import probe from 'libnative_child_process_example.so';
const graceful = await probe.runProbe(false);
const forced = await probe.runProbe(true);
```

Phone is the next selected target; an API26 Phone result alone does not prove
API22 compatibility, Tablet or 2-in-1 support. No HAP/device execution is claimed
by this source change. Do not launch the .so as a shell executable.

## What each call exercises

The parent creates two Unix socket pairs and passes child-side launch duplicates
as `probe.control` and `probe.echo`, with `binding-probe-v1` entry parameters.
Start uses only `OH_Ability_StartNativeChildProcess` and Normal options through
the safe binding. The builder duplicates/keeps strings/nodes/FDs through start;
the parent closes its original child endpoints immediately after return.

The child verifies params and exactly two FDs, takes each once, sends Ready with
its PID, echoes a bounded nonce on the second FD, then either returns normally
after Shutdown/Goodbye or waits after Hold/Holding for explicit API22 kill.
The parent checks the returned/announced PID against a distinct parent PID,
matched echo, control EOF and the generation-aware exit callback. IO and exit
waits have ten-second bounds. Probe failure attempts explicit kill cleanup;
identity Drop itself never kills.

Successful output records PIDs, generation, two-FD/params/echo/EOF, force branch,
raw SDK exit signal and Normal mode. It is runtime output, not prefilled PASS
evidence. Save the signed HAP hash, exact library hash, device/model/API/ABI,
source digest, device logs and raw call errors in the separate device report.

Full bindings acceptance still needs invalid entry, zero/one/many FDs, unexpected
crash, Busy/maximum-process, 100 cycles, FD count and parent termination cases.
This source example implements the bounded two-FD happy/kill branches only; it
does not waive those cases or the separate storage/engine gates.
