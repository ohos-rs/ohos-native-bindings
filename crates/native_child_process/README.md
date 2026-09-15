# Native child-process binding

Unpublished `0.1.0` source handoff. Independent source and device acceptance are
pending. This crate owns AbilityKit extended Native child-process access; it
does not spawn executables or configure independent processes.

## Features and dependency boundary

Default features are empty (API12). Enable `api-13` for typed start/entry/args,
`api-17` for current-child args, `api-20` for configs/exit subscriptions,
`api-21` for the configs UID flag, `api-22` for explicit kill, and `api-26` for
support query. Every intermediate feature forwards the chained sys feature.
The build SDK does not raise the runtime API minimum automatically.

```toml
[dependencies]
ohos-native-child-process-binding = { version = "0.1.0", features = ["api-22"] }
```

The OHOS target alone depends on the generated sys crate; a Unix host can run
pure tests without linking `child_process`. Host platform operations return
`HostUnsupported`, never a simulated process result. Production dependencies
are only `libc` and, on OHOS, the sys crate. Versions live in the root workspace.

## Typed launch

```rust,ignore
use std::{os::fd::AsFd, path::Path};
use ohos_native_child_process_binding::{
    ChildFdName, ChildProcessArgsBuilder, ChildProcessEntry,
    ChildProcessOptions, IsolationMode, NativeChildProcessManager,
};

let entry = ChildProcessEntry::new(Path::new("libchild.so"), "ChildMain")?;
let args = ChildProcessArgsBuilder::new()
    .entry_params("protocol-v1")?
    .named_fd(ChildFdName::new("probe.control")?, child_socket.as_fd())?;
let child = NativeChildProcessManager::new().start(
    &entry, args, ChildProcessOptions::new(IsolationMode::Normal),
)?;
// Original parent sockets remain parent-owned. Drop every original child-side
// socket after start so control EOF is observable. Handle Drop never kills.
```

Entry libraries are application-packaged ASCII `lib*.so` basenames, not absolute
or relative paths; symbols are C identifiers. Binding validation limits are
255 bytes per entry component, 64 ASCII bytes per FD name, 16 named FDs and
64 KiB parameter bytes. These conservative binding limits are not represented
as measured SDK/AppSpawn limits. Duplicate names and interior NULs fail before
FFI. Invalid borrowed FDs cannot be constructed by safe Rust; duplication
failures still retain exact errno before FFI.

Each named FD is duplicated with `F_DUPFD_CLOEXEC` (EINTR retried). The builder
owns those `OwnedFd`s, C strings and fixed-address linked nodes until start
returns. Both successful and failed calls drop all launch duplicates. Device
acceptance must prove that AbilityKit finishes the transfer before that return
and accepts CLOEXEC; host tests prove only Rust-side ownership/close behavior.

## Child entry

```rust,ignore
use ohos_native_child_process_binding::{
    ChildFdName, ChildLaunchArgs, NativeChildProcessError, native_child_entry,
};
fn child_main(mut args: ChildLaunchArgs<'_>) -> Result<(), NativeChildProcessError> {
    let control = args.take_fd(&ChildFdName::new("probe.control")?)?;
    // Run child work synchronously. Return ends the child process.
    Ok(())
}
native_child_entry!(ChildMain, child_main);
```

The macro emits the exact generated ABI and contains all decode/handler/drop
panics. It never performs fallible logging outside containment. Handler errors
return from the entry; report any application protocol failure inside the
handler before return. User panic payloads are intentionally forgotten so a
panicking payload destructor cannot cause a second unwind over C.

One process-global atomic claim prevents the adapter and API17 `with_current`
alternative from decoding the same transferred FDs twice. Borrowed strings
stay invocation-scoped; `named_fd` borrows its owner's FD and `take_fd` returns
an `OwnedFd` once. Untaken/adopted descriptors close on return/error/panic.
Readable SDK pointers cannot be established from arbitrary forged addresses;
the hidden unsafe adapter documents this platform-call contract. Mixing raw
argument/FD consumers with the safe adapter violates that contract.

## Exit observation and identity

The API20 trampoline is registered before the first binding-managed start and
remains registered for process lifetime. Its dispatcher state is process-global
and never deallocated while the callback is installed; no raw caller may
unregister it. Per-handle subscriptions are one-shot RAII owners and remove
their registry slot exactly once. Drop does not join a callback already in
flight. No handle Drop terminates a child.

The system callback attempts a bounded 128-event non-blocking send, performs no
user work and takes no registry lock. The worker routes by PID and monotonically
allocated launch generation, releases all locks, then invokes subscribers with
panic containment. Cached terminal events replay outside locks on the caller's
thread. Callbacks should enqueue short work, not perform long synchronous IO.

Early events are bounded to 64 with a five-second stale window. Old captured
events cannot attach to a later generation. Recent PID reuse, queue loss or
late dispatcher observation produces `ObservationLost`, not a fabricated exit
signal. The SDK callback has no generation token, so arbitrarily delayed OS
notifications cannot be proven unambiguous from PID alone: control EOF and
the child handshake remain mandatory device/consumer evidence. The registry
holds at most 64 live Rust observations and 64 subscribers per observation.

API22 `kill` is explicit, validates the current generation and rejects stale or
ambiguous identity. Success acknowledges the platform request, not observed
exit. Repeated calls may return `InvalidPid`; no automatic retry/idempotency
success is invented. Graceful control shutdown precedes forced termination.

## Configs and raw IPC

API20 configs use a non-null, unique RAII owner, no Clone or Send/Sync promise,
and exactly one platform destroy attempt. Process suffixes are a typed
`ChildProcessName` matching the header's 1..64 letters/digits/underscores rule.
The UID flag is gated at API21. Destruction errors cannot be returned by Drop
and need device investigation. Generic isolation options faithfully mirror the
SDK; the example always selects Normal.

`raw-ipc` exposes only visibly unsafe generated API12 Binder declarations (and
its API20 configs variant) on OHOS. It is not a safe IPC facade and does not own
remote proxies or provide IPCKit destruction. Product consumers must not use
this family. No independent-process capability or manifest is supplied.

## Verification

```sh
cargo test -p ohos-native-child-process-binding --all-features
cargo check -p ohos-native-child-process-binding \
  --target aarch64-unknown-linux-ohos --all-features --all-targets
cargo clippy -p ohos-native-child-process-binding \
  --target aarch64-unknown-linux-ohos --all-features --all-targets -- -D warnings
```

See [the two-FD example](../../examples/native_child_process) and
[B2 source evidence](../../docs/evidence/b2-native-child-process.md). No device
or API22 compatibility result is inferred from host tests or target compilation.
