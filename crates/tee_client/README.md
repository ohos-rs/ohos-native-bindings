# ohos-tee-client-binding

This crate is a binding for the TEE client module in OpenHarmony.

The TEE Client API is the GlobalPlatform interface a normal-world client
application uses to talk to a trusted application running in the secure world:
connect to a TEE, open a session with a trusted application, invoke commands on
it, and share memory with it. This crate wraps the native `tee_client_api.h` C
API with a safe layer — RAII handles whose lifetimes encode the required
teardown order, a tagged enum for the operation parameter union, and
`Result`-based error handling that keeps the return origin.

## Install

```shell
cargo add ohos-tee-client-binding --features api-20
```

The whole API arrived in API 20, so the `api-20` feature is required; without it
the crate exposes only `sys`.

## Usage

```rust
use ohos_tee_client_binding::{Context, Direction, Login, Operation, Parameter, Uuid};

let context = Context::new()?;
let mut session = context.open_session(
    &"79b77788-9789-4a7a-a2be-b60155eef5f3".parse::<Uuid>()?,
    Login::Public,
    None,
)?;

let request = b"ping";
let mut response = [0u8; 32];
let mut operation = Operation::with_params([
    Parameter::TempMemoryInput(request),
    Parameter::TempMemoryOutput(&mut response),
    Parameter::Value { direction: Direction::Output, a: 0, b: 0 },
    Parameter::None,
]);

session.invoke_command(1, Some(&mut operation))?;

let written = operation.output_size(1).unwrap_or(0);
let (status, _) = operation.value(2).unwrap_or((0, 0));
println!("{status}: {:?}", &response[..written]);
```

Zero-copy transfers use a shared memory block instead of a temporary reference:

```rust
use ohos_tee_client_binding::{Direction, Operation, Parameter, SharedMemory};

let mut buffer = SharedMemory::allocate(&context, 4096, Direction::InOut)?;
buffer.as_mut_slice()[..4].copy_from_slice(b"ping");

let mut operation = Operation::with_params([
    Parameter::WholeMemory(buffer.as_param()),
    Parameter::None,
    Parameter::None,
    Parameter::None,
]);
session.invoke_command(2, Some(&mut operation))?;
```

The raw bindings are re-exported as `tee_client::sys` for anything not yet
covered by the safe layer.

## Coverage

Behind the `api-20` feature — the API 20 header in full:

- Context: `Context::new` / `Context::with_path`, finalised on drop.
- Sessions: `Context::open_session`, `Session::invoke_command`, closed on drop.
- Shared memory: `SharedMemory::allocate`, `SharedMemory::register`,
  `as_slice` / `as_mut_slice` / `len` / `direction` / `is_allocated`, released
  on drop.
- Operations: `Operation` with four `Parameter` slots, the packed `paramTypes`
  field computed from them, write-back through `Operation::value` and
  `Operation::output_size`, and `Operation::request_cancellation`.
- Types: `Uuid` (parse, format, RFC 4122 bytes, fields), `Login`, `Direction`.
- Errors: `TeeError` with `ErrorKind`, `ReturnOrigin`, and the raw code.

## Notes

- **Teardown order is a compile-time rule.** `Session` and `SharedMemory` borrow
  their `Context`, and a registered `SharedMemory` also borrows the buffer it
  was built from. A program that would finalise a context while a session or a
  shared memory block is still open does not compile, so the intrusive lists the
  native context keeps cannot be corrupted by an out-of-order teardown.
- **The parameter union is a tagged enum.** `Parameter` fixes the payload and
  the 4-bit `TEEC_ParamType` code together; `Operation` packs the four codes
  into `paramTypes` itself, so `TEEC_PARAM_TYPES` bit shifting never reaches the
  caller. `Operation::param_types` exposes the packed value for debugging.
- **Login data is carried by the login method.** `Login::Group` and
  `Login::GroupApplication` hold the group identifier, so the untyped
  `connectionData` pointer has no counterpart in the safe API.
- **A shared memory block occupies at most one parameter slot.**
  `SharedMemory::as_param` takes an exclusive borrow, because the trusted
  application may write into the block while the operation runs and no other
  view of its content may be alive then.
- **Temporary memory references split by direction.** `TempMemoryInput` takes a
  shared slice; `TempMemoryOutput` and `TempMemoryInOut` take a mutable one.
  After the call, the length the trusted application reported — the bytes
  written, or the size needed when the call failed with `ErrorKind::ShortBuffer`
  — is read from `Operation::output_size`.
- **Cancellation is advisory.** `Operation::request_cancellation` only sends the
  request, and the current OpenHarmony implementation ignores it. Because an
  in-flight operation is borrowed exclusively by the call running it, a request
  can only be issued before or after that call.
- Handles are neither `Send` nor `Sync`: the native structures link themselves
  into per-context intrusive lists with no documented locking.

## License

MIT OR Apache-2.0
