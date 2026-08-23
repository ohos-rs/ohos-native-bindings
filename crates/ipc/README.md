# ohos-ipc-binding

This crate is a binding for the IPC module in OpenHarmony.

IPC is the HarmonyOS binder-style transport between processes: a service
publishes a remote stub, a client holds a proxy to it and sends it a command
word together with a serialized parcel, and the stub answers in a reply parcel.
This crate wraps the native `IPCKit` C API with a safe layer.

## Install

```shell
cargo add ohos-ipc-binding
```

## Usage

```rust
use ohos_ipc_binding as ipc;
use ipc::{Parcel, RemoteProxy, RemoteStub};

const DESCRIPTOR: &str = "org.example.IEcho";
const CODE_ECHO: u32 = 1;

// Service side: answer requests until the last peer lets go of the object.
let stub = RemoteStub::new(DESCRIPTOR, |code, data, reply| {
    if code != CODE_ECHO {
        return Err(ipc::IpcError::user(1909001));
    }
    if data.read_interface_token()? != DESCRIPTOR {
        return Err(ipc::IpcError::user(1909002));
    }
    let message = data.read_string()?;
    reply.write_string(&message)
})?;

// Client side, holding a proxy read out of a parcel.
fn echo(proxy: &RemoteProxy, message: &str) -> ipc::Result<String> {
    let mut data = Parcel::new()?;
    data.write_interface_token(DESCRIPTOR)?;
    data.write_string(message)?;
    let mut reply = Parcel::new()?;
    proxy.send_request(CODE_ECHO, &data, &mut reply)?;
    reply.read_string()
}
```

## Coverage

Everything wrapped here is available since API 12, so no `api-*` feature is
needed.

- `Parcel`: create, `data_size`, `writable_bytes`, `readable_bytes`,
  `read_position`, `write_position`, `rewind_read_position`,
  `rewind_write_position`, `append`, `write_*`/`read_*` for `i8`, `i16`, `i32`,
  `i64`, `f32`, `f64`, strings, interface tokens, byte buffers, file
  descriptors, remote stubs and remote proxies.
- `RemoteProxy`: `send_request`, `send_request_with` (sync or async, with
  `MessageOption`), `interface_descriptor`, `add_death_recipient`,
  `remove_death_recipient`, `is_remote_dead`.
- `RemoteStub`: create from a descriptor and a Rust closure.
- `DeathRecipient`: create from a Rust closure.
- Skeleton: `join_work_thread`, `stop_work_thread`, `calling_token_id`,
  `first_token_id`, `self_token_id`, `calling_pid`, `calling_uid`,
  `is_local_calling`, `is_handling_transaction`, `set_max_work_thread_num`,
  `reset_calling_identity` / `set_calling_identity`, and their scoped form
  `reset_calling_identity_scoped`, whose `CallingIdentityGuard` restores the
  caller credentials on drop.

## Notes

- Every handle is destroyed on drop: `Parcel`, `RemoteProxy`, `RemoteStub` and
  `DeathRecipient`. The parcels a request handler receives are borrowed from the
  runtime and are not destroyed.
- The closure passed to `RemoteStub::new` or `DeathRecipient::new` is reclaimed
  in the native destroy callback, not in `Drop`. Dropping a handle only releases
  this process's reference, while a request or a death notification may still be
  running on a runtime thread; reclaiming the closure there instead would be a
  use-after-free. In exchange, a runtime that never invokes the destroy callback
  leaks the closure, which is safe.
- Handler closures run on IPC worker threads, possibly several at a time, hence
  the `Send + Sync` bound. A panic inside one is caught and reported as an
  internal error rather than unwinding into C.
- A stub request handler returns `Err(IpcError::user(code))` to send a custom
  error code back to the caller; the code must be in \[1909001, 1909999\].
- `Parcel::read_string` and `Parcel::read_buffer` return a pointer into the
  parcel's own storage in the native API — no allocation the caller must
  release — so both copy the data out before returning.
- `Parcel::read_interface_token`, `RemoteProxy::interface_descriptor` and
  `reset_calling_identity` take a caller-supplied allocator and hand back memory
  the caller must release, including when they report an error. The native
  header names no deallocator, so this crate both allocates and frees that
  memory, and never lets the buffer escape the safe layer.
- `Parcel::read_remote_stub` and `Parcel::read_remote_proxy` return an owned
  handle. The header does not say so explicitly, but the kit hands out those
  objects alongside a `_Destroy` function, as it does for every object whose
  ownership passes to the caller.
- The native header does not state who closes a descriptor obtained from
  `Parcel::read_file_descriptor`, so it is returned as a plain number and this
  crate does not close it.
- `join_work_thread` blocks until `stop_work_thread` is called, so a service
  normally calls it from a dedicated thread.

## License

MIT OR Apache-2.0
