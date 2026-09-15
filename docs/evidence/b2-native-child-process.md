# B2 Native child-process safe binding — source handoff

Date: 2026-09-15. Implementation owner: GPT-5.6 Sol, xhigh.

Status: bounded source implementation and B2-SOURCE-01 owner repair/checks
completed. The original independent source/build review was CONDITIONAL for
API20/API21 strict lint; fresh independent post-repair acceptance and real-device
execution remain pending. No B1/B3/G0 PASS, product readiness, signed HAP,
commit, merge or publication is claimed.

## Inputs and scope

- Worktree `ohos-native-bindings-hotrix-child-process`, branch
  `hotrix/native-child-process`, HEAD `c08677a5fdff5fab58dcdaaff86b4f89c912f844`.
- Read Hotrix AGENTS.md, complete architecture/migration/acceptance plans,
  B2 handoff, complete B1 evidence, owning manifests/modules and mandatory Rust
  architecture standards skill before edits.
- B1 raw ABI remains frozen and pending independent acceptance. Existing B1
  changes were retained; no sys or generator was changed/regenerated.
- Original dirty bindings worktree, Hotrix product, aria2, Arkit/K0/UI, app data
  and signing configuration were not modified. No shell AppSpawn simulation.
- rustc `1.98.1 (48a229cea 2026-09-01)`, cargo
  `1.98.1 (797e8a9bc 2026-08-05)`, ohrs `1.5.0`.
- Uses B1's SDK26: DevEco Studio `26.0.0.821`, Native package `26.0.0.105`.
  SDK LLVM tools:
  `/Users/ranger/.meat/ide/DevEco-Studio-26.0.0.821.app/Contents/sdk/default/openharmony/native/llvm/bin`.
- Cargo.lock remains ignored and was not force-added. Resolved libc `0.2.189`
  and napi-ohos/napi-derive-ohos/napi-build-ohos `1.2.0` from existing workspace
  caret constraints. No unrelated dependency version policy was changed.

## B2-only paths

```text
Cargo.toml                      (one safe-package workspace dependency)
README.md                       (safe module row and source-handoff links)
crates/native_child_process/Cargo.toml
crates/native_child_process/README.md
crates/native_child_process/CHANGELOG.md
crates/native_child_process/src/lib.rs
crates/native_child_process/src/error.rs
crates/native_child_process/src/types.rs
crates/native_child_process/src/args.rs
crates/native_child_process/src/list.rs
crates/native_child_process/src/entry.rs
crates/native_child_process/src/configs.rs
crates/native_child_process/src/registry.rs
crates/native_child_process/src/manager.rs
examples/native_child_process/Cargo.toml
examples/native_child_process/README.md
examples/native_child_process/build.rs
examples/native_child_process/src/lib.rs
examples/native_child_process/src/probe.rs
docs/evidence/b2-native-child-process.md
```

Root Cargo/README also retain B1 hunks. B2 does not own the raw dependency,
generator dependencies/raw module table or any other B1 path listed in its
report. Ignored dist/target outputs are local artifacts, not staged source.

## Ownership and public contract

Unpublished safe package `ohos-native-child-process-binding` `0.1.0`.
Dependencies: example -> safe -> sys (OHOS target only), plus workspace libc.
All member dependencies use `workspace = true`; no application/engine edge or
cycle. Empty default is API12; all API13..26 features chain/forward SDK gates.

- Owned entry validates packaged ASCII lib*.so basename and C export identifier;
  traversal/non-UTF-8/NUL/malformed input fails before FFI. Typed options always
  zero reserved. Typed FD and process names replace unvalidated caller strings.
- Binding limits: entry components255 bytes, FD names64 bytes, FDs16,
  params64 KiB. These are conservative binding limits, not AppSpawn measurements.
  Typed process names match the header's 1..64 ASCII letters/digits/underscore.
- Args builder duplicates BorrowedFd with F_DUPFD_CLOEXEC/EINTR retry, retains
  exact errno on failure, rejects duplicate names and keeps originals parent-owned.
  PreparedArgs retains all strings, OwnedFd duplicates and fixed-address
  generated linked nodes through synchronous start. Host-tested LinkedNodes
  supplies pointer stability without reproducing a C declaration/layout.
  Launch duplicates close after success/failure; transfer timing/CLOEXEC still
  require device proof, not a host-verified platform promise.
- ChildLaunchArgs owns once-only delivered FD adoption/take. One atomic claim
  prevents adapter/current-args double decoding. Higher-ranked entry closures
  contain borrowed string lifetimes; untaken/adopted FDs close on normal return,
  malformed metadata, error and contained panic. SDK pointer readability remains
  an explicit unsafe entry contract, never validation of arbitrary forged pointers.
- Entry macro emits the exact generated unsafe extern C ABI and edition-safe
  export attribute. Decoder/handler/drop panics are contained at that boundary;
  panic payloads are forgotten to prevent panicking payload Drop. No fallible
  logging follows containment. API17 with_current is an alternative once-only
  bootstrap, not a second FD owner.
- Configs are non-null unique RAII, exactly one SDK destroy attempt, no Clone or
  Send/Sync promise; config strings remain owned. UID setter is API21 gated.
- API20 registers one process-lifetime trampoline before launch. The system
  callback attempts a bounded128 non-blocking send, no registry lock/user work.
  Dispatcher/cached replay invoke users after releasing all locks, with panic
  containment. Subscriptions remove once on Drop; in-flight work may finish.
- Launch generation capture remains active until PID association completes,
  closing the FFI-return/immediate-exit gap. Early events capped64, five-second
  stale window; old captured events cannot poison later generations. Records
  and per-record subscribers capped64; recent PID reuse/loss/stale dispatch
  reports ObservationLost rather than inventing an exit signal.
- Handle owns identity only and has no terminating Drop. API22 kill is explicit,
  serialized against launch and generation-checked; stale/ambiguous identity is
  refused. Success acknowledges request, not exit; repetition may yield InvalidPid.
  API26 support query is separately gated. All published numeric errors map to
  typed variants; unknown u32 codes round-trip.
- API12 Binder family remains an explicit OHOS raw-ipc feature of generated
  unsafe declarations. No fake safe IPC facade, C declaration copy, private
  shim or independent-process capability/configuration was added. Generic SDK
  isolation enums are faithful; example always uses Normal extended child.

SDK exit callback has PID/signal but no generation token. Arbitrarily delayed
OS PID reuse cannot be proven unambiguous from it alone; control EOF/Ready
handshake remain mandatory. Do not mix raw process/argument consumers or
separately unregister this binding's process-lifetime trampoline.

## Exact checks and results

Run in this worktree except ohrs builds in the example directory. A target
compile/check is not runtime or independent acceptance.

| Command/check | Owner result |
| --- | --- |
| cargo test -p ohos-native-child-process-binding --all-features (final --quiet rerun too) | exit0, 21/21 host pure tests |
| cargo test -p ohos-native-child-process-binding --no-default-features | exit0, API12 error test1/1 |
| cargo check -p ohos-native-child-process-binding --target aarch64-unknown-linux-ohos --no-default-features --all-targets | exit0 |
| same check with --features api-N for every N=13..26 | all14 exit0;15 total combinations |
| cargo check -p ohos-native-child-process-binding -p native_child_process_example --target TARGET --all-features --all-targets --quiet | all3 exit0; TARGET aarch64/armv7/x86_64-unknown-linux-ohos |
| cargo clippy -p ohos-native-child-process-binding -p native_child_process_example --target aarch64-unknown-linux-ohos --all-targets --all-features -- -D warnings | exit0, final source rerun |
| cargo clippy -p ohos-native-child-process-binding --all-targets --all-features -- -D warnings | exit0, host |
| cargo fmt --all; cargo fmt --all -- --check; narrow taplo formatting | exit0 |
| pnpm run format:check | exit0, Rust/all113 TOML files |
| pnpm run check:rust | exit0, full workspace except generator |
| pnpm run lint:rust | exit101, baseline IME lint below; not unrestricted PASS |
| cargo metadata --format-version 1 --no-deps --quiet, filtered through Node | exit0, owning graph confirmed |
| cargo tree -e features -i ohos-native-child-process-binding -p native_child_process_example --target aarch64-unknown-linux-ohos | exit0; API13..22 chain, no API26 |
| ohrs build --arch aarch | exit0; actual linked API22 shared library/types |
| ohrs build --arch aarch --dist dist/api26 -- --no-default-features --features api-26 | exit0; separate API26 artifact |
| SDK llvm-nm -D -u, llvm-nm -D --defined-only, llvm-readelf -d on example artifacts | exit0; symbols/dependencies below |
| git diff --check | exit0 |

Pure tests cover entry/name/NUL/limit/error validation; original/duplicate FD
lifetimes/CLOEXEC/failed host start cleanup; child once-only FD take and untaken/
invalid-metadata cleanup; zero/one/many stable lists; once-only config destroy;
early exit/cached replay; lock-free user invocation/panic/cancel; failed launch,
stale/generation/PID reuse; bounded/overflow and pending-launch-loss behavior.
None execute AppSpawn or a shell substitute.

## Baseline failures and author iterations

Full lint fails at examples/ime/src/lib.rs:7 with
clippy::missing_const_for_thread_local (lib and lib-test). Its initializer is
already const { RefCell::new(None) }; git show HEAD:examples/ime/src/lib.rs
confirmed identical baseline code, and that file is unmodified. No lint allowance
or unrelated fix was added. B1 generator lint limitations remain in its report.
The baseline nine ignored napi-ohos default-features manifest warnings remain;
the new example does not add that warning.

Initial author iterations exposed a parallel FD-test reuse assertion race,
NAPI Result-alias mismatch, test-only export reachability and shadowed lifetime;
all repaired before final21-test/target checks. Large metadata extraction was
truncated during orchestration; the Node filter reproduced it without writes.
No failing iteration is counted as passing evidence.

## Real linked example, API symbols and hashes

API22 output: examples/native_child_process/dist/arm64-v8a/libnative_child_process_example.so.
API26 output: examples/native_child_process/dist/api26/arm64-v8a/libnative_child_process_example.so.
Both export BindingProbeMain. Generated run entrance:

```ts
export declare function runProbe(forced: boolean): Promise<string>
```

API22 undefined child APIs are exactly:

```text
OH_Ability_KillChildProcess
OH_Ability_RegisterNativeChildProcessExitCallback
OH_Ability_StartNativeChildProcess
```

API26 adds only OH_Ability_IsNativeChildProcessSupported to this list. Neither
example imports Binder Create/WithConfigs. API22 absence is checked in the linked
ELF, not inferred just from features. DT_NEEDED: libace_napi.z.so,
libchild_process.so, libc.so. The same packaged library serves NAPI parent and
Native child; signed HAP loading/NAPI initializer behavior still need device proof.

SHA256 at original handoff (historical; repaired outputs are pinned below):

```text
17 safe/example source files, sorted aggregate:
ab98f05308d078556648064d76346279667af8307fd08947fe597147665690f1
Cargo.toml (preserved B1 plus B2 dependency):
177b1ba2ce13f8eb7eb7821caa9124184d0f540aa201409c99ff04fb3c2ba6cd
README.md (preserved B1 plus B2 registration):
1824501ea1bdb0978832f2d582d06f5a6311c754909a6649d99d67c4c07ab47f
API22 example shared library:
abd34f565eea35391bcca426dab275483fa440da0f4837e36fc9790e0ee519e9
API26 example shared library:
81be9d7b805503cf897ae6f0631d6abb7f419529a7974bc3877f2fe1cfcc7aee
unchanged B1 sys source:
5a115b472df2350b086bacc4d498929ef92ca712eb79854849c8b35c834f28dc
unchanged pre-existing tracked sys aggregate:
26896d0910584c78b499efe794c37f5412859562847d323bc5fecee5b346267f
```

Source aggregate command: rg --files crates/native_child_process
examples/native_child_process | LC_ALL=C sort | xargs shasum -a 256 | shasum -a
256. Ignored binaries and this report are excluded. Existing-sys command:
git ls-files sys | xargs shasum -a 256 | shasum -a 256, matching B1.
Debug hashes identify local outputs; clean-checkout reproduction remains
independent acceptance, not an author certification.

## B2-SOURCE-01 — bounded API20/API21 cfg repair

Date: 2026-09-15. Repair owner: fresh GPT-5.6 Sol, xhigh, task
`/root/b2_api_matrix_repair`. This is an author repair/check result, not
independent acceptance. The unchanged Hotrix independent report
`docs/evidence/acceptance/b1-b2-source.md` records the original Minor finding
and its original hashes; a fresh reviewer must reproduce this repaired snapshot.

Read complete Hotrix AGENTS.md, architecture/migration/acceptance plans, B2
handoff, independent B1+B2 report, this owner report and the mandatory Rust
architecture standards skill before implementation. Inspected owning manifests,
manager/registry modules and the metadata-filtered sys/safe/example graph.
The pre-repair manager/registry hashes and 17-file aggregate matched the
independent finding exactly. API20 strict clippy was reproduced with exit101:
`ChildProcessHandle.runtime` was unused at manager128, and `Record::terminal`
was unused at registry141. This was not a baseline IME/generator failure.

Only these three source/evidence paths were edited in the repair:

```text
crates/native_child_process/src/manager.rs
crates/native_child_process/src/registry.rs
docs/evidence/b2-native-child-process.md
```

- `ChildProcessHandle.runtime` and both start-method initializer uses now
  compile only under API22, matching their only consumer, explicit `kill`.
  API20 still initializes the same process-global runtime before launch, retains
  its record/generation and subscribes/replays exits identically. The static
  `RUNTIME` retains the process-lifetime runtime independently of handle fields.
- `Record::terminal` now compiles under API22 or tests. Cached terminal state,
  observation, subscription delivery and cancellation are unchanged. API20 and
  API21 pure registry tests still inspect terminal state through the test gate.
- No lint allowance, public API, raw ABI, dependency, feature chain, process
  policy or entry/FD ownership contract was changed. The Rust skill directed
  consumer-owned cfg boundaries and strict verification; host-only dead-code
  attributes already present in the original source were not changed.
- B1 sys/generator/evidence, root manifests/README and all other B2 source were
  preserved. No original dirty worktree, Hotrix source, aria2, UI/Arkit/K0,
  device/app data or signing configuration was edited. No subagent, commit,
  merge, publication, device write or AppSpawn substitute was used.

### Post-repair commands actually executed

Same toolchain/SDK and isolated worktree as the original handoff. Quiet only
reduces output; no command-level warning allowance was used. All listed checks
are owner reproduction, not a device or clean-checkout certification.

| Command/check | Repair owner result |
| --- | --- |
| `cargo test -p ohos-native-child-process-binding --no-default-features --features api-20` | exit0, 21/21 pure host tests; zero doctests |
| same host test with `--features api-21 --quiet` | exit0, 21/21; zero doctests |
| same package `--all-features --quiet` | exit0, 21/21; zero doctests |
| same package `--no-default-features --quiet` | exit0, API12 error test1/1; zero doctests |
| combined sys/safe strict aarch64 clippy, empty default then each api13..26, all targets | all15 exit0; exact loop below |
| combined sys/safe/example OHOS `cargo check --all-targets --all-features --quiet` | exit0 for each aarch64/armv7/x86_64 ABI |
| combined sys/safe/example OHOS `cargo clippy --all-targets --all-features --quiet -- -D warnings` | exit0 for each aarch64/armv7/x86_64 ABI |
| `cargo clippy -p ohos-native-child-process-binding --all-targets --all-features --quiet -- -D warnings` | exit0, host |
| `cargo fmt --all -- --check` | exit0, check-only; no formatting rewrite |
| `taplo format --check` | exit0, all113 TOML files; check-only |
| `cargo metadata --format-version 1 --no-deps --quiet`, Node-filtered owning graph | exit0; original graph/features preserved |
| original inverse `cargo tree -e features -i ohos-native-child-process-binding -p native_child_process_example --target aarch64-unknown-linux-ohos` | exit0; API13..22 chain, no API26 |
| example cwd: `ohrs build --arch aarch` | exit0; rebuilt linked API22 library and types |
| example cwd: `ohrs build --arch aarch --dist dist/api26 -- --no-default-features --features api-26` | exit0; rebuilt separate API26 library and types |
| SDK `llvm-nm -D -u`, `llvm-nm -D --defined-only`, `llvm-readelf -d` on both final libraries | exit0; unchanged child API/export/dependency lists below |
| source/artifact/preserved B1 hash checks and `git diff --check` | exit0; hashes below |

Exact strict feature regression (run from isolated bindings root):

```sh
for api in default 13 14 15 16 17 18 19 20 21 22 23 24 25 26; do
  if [ "$api" = default ]; then
    cargo clippy -p ohos-native-child-process-sys \
      -p ohos-native-child-process-binding \
      --target aarch64-unknown-linux-ohos --no-default-features \
      --all-targets --quiet -- -D warnings
  else
    cargo clippy -p ohos-native-child-process-sys \
      -p ohos-native-child-process-binding \
      --target aarch64-unknown-linux-ohos --no-default-features \
      --features "api-$api" --all-targets --quiet -- -D warnings
  fi
  result=$?
  echo "API=$api strict_clippy_exit=$result"
  if [ "$result" -ne 0 ]; then exit "$result"; fi
done
```

Output was `strict_clippy_exit=0` for default/API12 and every API13..26;
loop exit0. The existing test suite is the observation-behavior regression;
the API20/API21 host runs specifically verify that terminal inspection remains
available to tests without requiring API22 production support.

API22 undefined child API list remains exactly StartNativeChildProcess,
RegisterNativeChildProcessExitCallback and KillChildProcess. API26 adds only
IsNativeChildProcessSupported. Neither artifact imports Binder Create or
WithConfigs. Both defined dynamic symbol lists contain exactly BindingProbeMain
and napi_register_module_v1. Both DT_NEEDED lists remain exactly
libace_napi.z.so, libchild_process.so and libc.so. These are generic linked
example libraries, not production Hotrix ELFs or a signed HAP; no compiled
Ready/echo/EOF/graceful/kill path was executed.

### Repaired frozen SHA-256 snapshot

| Source/output | Repair owner SHA-256 |
| --- | --- |
| manager.rs | `ffe380a1d0ecf398b789a6e8a52724a6f18cd6a6451ad1e780e362d4c23902c7` |
| registry.rs | `6e73b0b3c2757fe77499356a470bd3e542ce3bf42f571f10ae598db7d47dc3f1` |
| safe/example17-file sorted aggregate | `707f04dd94852e6338bdacb18d5a2c1f4d55143d50e6d1404e61f65883df824f` |
| rebuilt API22 example shared library | `d87915e40f73ebf0ff0a0b1678c8abfe04ccfbbeeeb6429384c9e61bc3ea2906` |
| rebuilt API26 example shared library | `68a1e07600899de543c6867f7660705af3332e09343af6d950e23e8652ea992e` |
| preserved Cargo.toml | `177b1ba2ce13f8eb7eb7821caa9124184d0f540aa201409c99ff04fb3c2ba6cd` |
| preserved README.md | `1824501ea1bdb0978832f2d582d06f5a6311c754909a6649d99d67c4c07ab47f` |
| preserved generated sys src/lib.rs | `5a115b472df2350b086bacc4d498929ef92ca712eb79854849c8b35c834f28dc` |
| preserved tracked sys aggregate | `26896d0910584c78b499efe794c37f5412859562847d323bc5fecee5b346267f` |
| preserved generator build/main.rs | `2c4cd096cd9fe8ab6179f404022fe799a6e8a3c3a813350f443f03ad106e14dd` |
| preserved generator build/selection.rs | `8b6e9aff7c1658848e1e1fad79153860ce558995d19a134072ea00e22b89fcd9` |
| preserved B1 owner evidence | `ad509d48dcd1b65c7450b3be52b498476b5c0c7ce6c38846ca24f4a2170ff14b` |

The aggregate commands are unchanged from the original handoff. Original
source/output hashes above remain historical evidence and are superseded only
by this repaired source/artifact table. Build output hashes changed and must be
pinned anew by the independent reviewer. Ignored target/dist outputs remain
rebuildable local evidence; no lockfile/source pin/publication policy was widened.
Original baseline IME/generator lint limitations remain outside this repair;
unrestricted full-workspace lint is not newly certified or waived.

Repair source was frozen and reported to the coordinator before the final
artifact/evidence handoff. Rollback is only the cfg additions in manager.rs and
registry.rs, this owner evidence update, and rebuildable ignored outputs; never
remove preserved B1/B2 originals or reset any worktree. B3/G0/device/live-product,
K0/accessibility and release/publication gates remain pending and are not
unlocked by this source repair.

## Next gate and rollback

Source example has real compiled NAPI worker/Native entry, Normal start, two
named FDs/params/Ready/distinct PID/nonce echo, graceful return, EOF/exit event
and explicit API22 kill. Phone is the next selected target. No HAP/device was
installed/invoked during this task; no shell substitute was used.

A fresh independent Sol xhigh must review/pin this source snapshot. A separate
explicitly selected signed feasibility HAP must package exact library/types
with protected external signing and named Phone/model/API/ABI. Device proof
must cover AppSpawn/NAPI loading, FD transfer timing/CLOEXEC/unrelated-FD
exclusion, callback thread/order, lifecycle and parent termination. Full B3
also requires invalid entry, zero/one/many FDs, unexpected crash, Busy/maximum,
100 cycles, FD counts and orphan checks. API26 Phone cannot imply API22 or other
form factors. B3/G0/storage/aria2 gates remain independently incomplete.

Rollback is limited to listed B2 paths/root hunks and rebuildable ignored
outputs. Never reset this worktree, remove preserved B1 changes, overwrite the
original dirty worktree or alter app data/signing/aria2. Do not commit, merge or
publish before independent/device acceptance.
