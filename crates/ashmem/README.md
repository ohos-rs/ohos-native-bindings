# ohos-ashmem-binding

Safe Rust bindings for OpenHarmony anonymous shared memory (`/dev/ashmem`).

The crate implements the Ashmem operations directly and does not require a
separate `-sys` crate or a C++ shim.

## Install

```shell
cargo add ohos-ashmem-binding
```

## Usage

```rust,no_run
use ohos_ashmem_binding::{Ashmem, Protection};

let mut ashmem = Ashmem::create("example", 4096)?;
ashmem.map_read_write()?;
ashmem.write(0, b"hello")?;

assert_eq!(ashmem.read(0, 5)?, b"hello");

// Ashmem protection can only be reduced after creation.
ashmem.unmap()?;
ashmem.set_protection(Protection::READ)?;
ashmem.map_read_only()?;

# Ok::<(), ohos_ashmem_binding::AshmemError>(())
```

`Ashmem` automatically unmaps and closes its file descriptor when dropped. Use
`AsFd` to pass the descriptor to an IPC API, or `Ashmem::from_owned_fd` to adopt
an Ashmem descriptor received from another process.

Access to memory shared with another process must be externally synchronized.

## License

MIT OR Apache-2.0
