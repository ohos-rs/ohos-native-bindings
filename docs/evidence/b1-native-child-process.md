# B1 Native child-process raw binding — source handoff

Date: 2026-09-15. Implementation owner: GPT-5.6 Sol, xhigh.

Status: bounded source implementation and checks completed; independent review
pending. No commit, merge, publish, device execution, G0 PASS or B3 PASS is
claimed. B2 safe bindings and the device example are not implemented here.

## Worktree and inputs

- Owning worktree: `ohos-native-bindings-hotrix-child-process`.
- Branch: `hotrix/native-child-process`.
- Baseline/HEAD: `c08677a5fdff5fab58dcdaaff86b4f89c912f844`.
- The original `ohos-native-bindings` dirty worktree was not modified, stashed,
  reset or merged. The isolated worktree was clean at task start.
- Read Hotrix `AGENTS.md`, complete `docs/architecture.md` and
  `docs/migration-plan.md`, bindings How-To/generator docs, owning modules and
  the Rust architecture standards skill before implementation.
- SDK: DevEco Studio `26.0.0.821`, OpenHarmony Native package `26.0.0.105`,
  API26 (`native/oh-uni-package.json`).
- Actual SDK native directory:
  `/Users/ranger/.meat/ide/DevEco-Studio-26.0.0.821.app/Contents/sdk/default/openharmony/native`.
- rustc `1.98.1 (48a229cea 2026-09-01)`; cargo
  `1.98.1 (797e8a9bc 2026-08-05)`; ohrs `1.5.0`; bindgen `0.65.1`.
- Cargo.lock remains ignored by existing repository policy; no lockfile is
  staged or force-added. No credentials, RPC secrets or signing paths added.

## Fixed raw API contract

Generated crate: `sys/ohos-native-child-process-sys`, version `0.1.0`
(unpublished), no dependencies, links `child_process`.

| API | Exact SDK declarations |
| --- | --- |
| 12 | `OH_Ability_CreateNativeChildProcess`, `OH_Ability_OnNativeChildProcessStarted`, opaque `OHIPCRemoteProxy`, error enum and baseline constants |
| 13 | `OH_Ability_StartNativeChildProcess`, `NativeChildProcess_Fd`, `FdList`, `Args`, `Options`, `IsolationMode` and both isolation constants |
| 17 | `OH_Ability_GetCurrentChildProcessArgs` |
| 20 | opaque `Ability_ChildProcessConfigs`, `OH_Ability_CreateChildProcessConfigs`, `DestroyChildProcessConfigs`, `ChildProcessConfigs_SetIsolationMode`, `ChildProcessConfigs_SetProcessName`, `CreateNativeChildProcessWithConfigs`, `StartNativeChildProcessWithConfigs`, `OnNativeChildProcessExit`, `RegisterNativeChildProcessExitCallback`, `UnregisterNativeChildProcessExitCallback`, callback-not-exist error |
| 21 | `OH_Ability_ChildProcessConfigs_SetIsolationUid` |
| 22 | `OH_Ability_KillChildProcess`, invalid-PID error |
| 26 | `OH_Ability_IsNativeChildProcessSupported` returning C `bool` |

Rows with abbreviated function names retain the `OH_Ability_` prefix. All
14 SDK functions are present, and their names exactly match the 14
`OH_Ability_` exports in the SDK's aarch64 `libchild_process.so` stub. No
undeclared start/create alternatives or IPCKit facade are invented.

The entry function is `unsafe extern "C" fn(NativeChildProcess_Args)`.
`OH_Ability_StartNativeChildProcess` receives entry `*const c_char`, args and
options **by value**, and PID `*mut i32`; the return type is the SDK error enum
represented as `u32`. Startup callback: nullable
`unsafe extern "C" fn(c_int, *mut OHIPCRemoteProxy)`; exit callback: nullable
`unsafe extern "C" fn(i32, i32)`. Configs are opaque mutable pointers.

Default features are empty (API12). All intermediate features `api-13` through
`api-26` are chained. API22 cannot name the API26 support query, independently
of the SDK used to compile. All error numeric values and all signatures are
asserted in `tests/abi.rs`; SDK C++ assertions independently verify the same
function types, enum widths/signedness, struct size/alignment and offsets.

| Layout | 64-bit | 32-bit |
| --- | --- | --- |
| `NativeChildProcess_Fd` | size24/align8; offsets0,8,16 | size12/align4; offsets0,4,8 |
| `NativeChildProcess_FdList` | size8/align8; head0 | size4/align4; head0 |
| `NativeChildProcess_Args` | size16/align8; offsets0,8 | size8/align4; offsets0,4 |
| `NativeChildProcess_Options` | size16/align8; offsets0,8 | size16/align8; offsets0,8 |

The generic raw module faithfully exposes SDK normal/isolated enum values; it
does not add HarmonyOS independent-process configuration or an application-local
C/C++ process shim. No product launch policy is implemented. Hotrix's future
engine launch remains extended Native start in NORMAL mode only.

## Generator behavior and bounded changes

`OHOS_BINDINGS_GENERATE_CONFIG` accepts one exact registry crate name. An
explicit invalid/empty/non-UTF-8/ambiguous name fails before generation writes;
patterns, aliases, padded names and lists are rejected. Both Rust generation and
manifest feature synchronization consume only the selected registry entries.
Selected generation errors fail the build. Unset selection retains legacy
best-effort full generation, with the registry's declared order.

Bindgen 0.65 omits comments on opaque forward declarations. The touched gate
processor now excludes such undocumented opaque records from the API12 fallback
and inherits the earliest documented use, correctly gating configs at API20.
The API12 IPC proxy still retains its SDK API12 documentation. This generic
inference change has a regression test; no existing sys crate was regenerated.

Actual changed/new paths (the complete allowlist for this slice):

```text
Cargo.toml
How-To-Add-A-New-Crate.md
README.md
docs/evidence/b1-native-child-process.md
tools/generate/Cargo.toml
tools/generate/README.md
tools/generate/build/config/mod.rs
tools/generate/build/config/native_child_process.rs
tools/generate/build/main.rs
tools/generate/build/selection.rs
tools/generate/tests/generator_contract.rs
sys/ohos-native-child-process-sys/Cargo.toml
sys/ohos-native-child-process-sys/README.md
sys/ohos-native-child-process-sys/CHANGELOG.md
sys/ohos-native-child-process-sys/src/lib.rs
sys/ohos-native-child-process-sys/tests/abi.rs
sys/ohos-native-child-process-sys/tests/availability/api26.rs
sys/ohos-native-child-process-sys/tests/sdk_abi.cpp
```

Dependency versions for the touched generator moved unchanged to the root
workspace; member build/dev dependencies use `workspace = true`. The new sys
crate has no Cargo dependencies; its edge to the generator is tooling-only, not
a production dependency. `cargo metadata --no-deps` registers it successfully.

## Commands and results

Commands ran in the isolated workspace unless stated otherwise. `SDK_NATIVE`
below stands for the exact SDK native directory recorded above; set a task-local
variable to that path when reproducing.

| Check | Result |
| --- | --- |
| `rustc --edition 2021 --test tools/generate/build/selection.rs` and resulting test binary | PASS, 5 tests |
| selected `ohrs build --arch aarch`, cwd `tools/generate` | PASS, exit0; no unrelated sys output |
| `cargo test -p generate --test generator_contract`, env below | PASS, 8/8 |
| aarch64 baseline + each `api-13`…`api-26`, `--no-default-features --tests` | PASS, 15 combinations |
| `cargo check -p ohos-native-child-process-sys --target TARGET --all-features --tests` for aarch64, armv7, x86_64 OHOS | PASS, all3 |
| SDK clang++ assertions with targets aarch64-linux-ohos, arm-linux-ohos, x86_64-linux-ohos | PASS, all3, `-Werror` |
| raw function names vs SDK aarch64 stub exports (`diff` of sorted names) | PASS, exact14/14 |
| same `tests/availability/api26.rs` compiled against API22 then API26 raw rlibs | PASS: API22 expected E0425 not-found; API26 compiles metadata |
| `cargo clippy -p ohos-native-child-process-sys --target aarch64-unknown-linux-ohos --all-targets --all-features -- -D warnings` | PASS |
| `cargo clippy -p generate --all-targets -- -D warnings`, env below | FAIL, 6 baseline lint errors; see next section |
| same generator clippy with command-level allowances for the 3 baseline lint categories | PASS; not represented as unrestricted clippy PASS |
| `pnpm run format:check` | PASS, Rust and all111 TOML files |
| `git diff --check` and `cargo metadata --format-version 1 --no-deps` | PASS |
| unknown explicit selector with `cargo build -p generate` | expected build-script exit101 before bindgen, sys digest unchanged |
| `cargo clean -p generate` then repeated selected ohrs generation | PASS, generated bytes identical and existing sys digest unchanged |

Narrow test/clippy environment (the host test still runs the selected build
script before its test binary):

```sh
OHOS_BINDINGS_GENERATE_CONFIG=ohos-native-child-process-sys \
LIBCLANG_PATH="$SDK_NATIVE/llvm/lib" \
BINDGEN_EXTRA_CLANG_ARGS="--target=aarch64-linux-ohos --sysroot=$SDK_NATIVE/sysroot" \
cargo test -p generate --test generator_contract
```

API matrix:

```sh
cargo check -p ohos-native-child-process-sys \
  --target aarch64-unknown-linux-ohos --no-default-features --tests
for api in 13 14 15 16 17 18 19 20 21 22 23 24 25 26; do
  cargo check -p ohos-native-child-process-sys \
    --target aarch64-unknown-linux-ohos --no-default-features \
    --features "api-$api" --tests --quiet || exit
done
```

SDK ABI check:

```sh
for abi in aarch64-linux-ohos arm-linux-ohos x86_64-linux-ohos; do
  "$SDK_NATIVE/llvm/bin/clang++" --target="$abi" \
    --sysroot="$SDK_NATIVE/sysroot" -std=c++17 -Werror -fsyntax-only \
    sys/ohos-native-child-process-sys/tests/sdk_abi.cpp || exit
done
```

Repeated generation used only `cargo clean -p generate` (576 temporary build
files, rebuildable), not a workspace/source deletion or Git reset. The generated
crate already existed and its manually added assertions/docs were not replaced.

SHA-256 evidence:

```text
SDK header:
f69fef388fa0177b4d6a75779f7905706c73afa67e9e79e8f13a297dbc4d79ec
SDK aarch64 child_process stub:
a63f401c9985e1d965993ca16ea35439012da436b9ea0051da641493f4def031
generated src/lib.rs (after gate fix, direct test and repeated ohrs generation):
5a115b472df2350b086bacc4d498929ef92ca712eb79854849c8b35c834f28dc
aggregate existing tracked sys files before/after generation and formatting:
26896d0910584c78b499efe794c37f5412859562847d323bc5fecee5b346267f
```

Existing-sys digest command: `git ls-files sys | xargs shasum -a 256 | shasum -a
256`. At this handoff the new crate is untracked, so this measures only the
unchanged baseline sys files. Invalid-selector checking included the new sys
crate via `git ls-files --cached --others --exclude-standard sys` and compared
digests before/after; it made no sys writes.

## Baseline/tooling limitations and open acceptance

Unrestricted generator clippy is not PASS. All6 errors existed in the pinned
baseline `tools/generate/build/main.rs`: `drain_collect` at baseline334,363,375;
`extend_with_drain` at389,843; `too_many_arguments` at declaration_key987 in the
changed file (baseline972). `git show` verified the same expressions/signature.
No lint allowances or unrelated refactor were added to production code. New
generator code was additionally checked using only command-level
`-A clippy::drain_collect -A clippy::extend_with_drain -A
clippy::too_many_arguments`, alongside `-D warnings`.

Cargo also emits the baseline9 example-manifest warnings for ignored
`default-features` on workspace `napi-ohos`; no example or napi dependency policy
was changed. These are manifest diagnostics, separate from Rust/clippy warnings.

Two attempted `ohrs cargo --disable-target` host-test invocations did not execute
Cargo (`You don't provide any target`). The direct Cargo environment above was
the successful alternative. No failed invocation is counted as a passing test.

Device acceptance was not executed: AppSpawn start support, signed HAP loading,
separate PID, FD transfer timing/ownership, callback thread/order, exit/kill and
parent lifetime need the fresh B2/B3/G0 sessions and admitted device matrix.
There is no executable aria2 proof here. Stop at this raw source handoff until
independent review is available; do not publish or unlock product implementation.

Rollback boundary: keep this slice separate; an authorized later task may remove
only the listed new files and revert only this slice's root/docs/generator edits.
Do not reset or copy files from the original dirty worktree.
