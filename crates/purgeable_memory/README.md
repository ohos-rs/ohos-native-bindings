# ohos-purgeable-memory-binding

This crate is a binding for the purgeable memory module in OpenHarmony.

A purgeable memory object owns a block of content that the system is free to
reclaim under memory pressure whenever the application is not actively using it.
To use the content, the application first asks for an access permit (begin read /
begin write); while the permit is held the system cannot reclaim the block, and
once the permit is released it may. If the content was already reclaimed, the
object rebuilds it from the rebuild function supplied at construction time, plus
every modification appended afterwards. This crate wraps the native
`purgeable_memory.h` C API with a safe layer.

## Install

```shell
cargo add ohos-purgeable-memory-binding
```

## Usage

```rust
use ohos_purgeable_memory_binding::PurgeableMemory;

// The rebuild function fills the block whenever the system has reclaimed it.
let mut mem = PurgeableMemory::new(4096, |content: &mut [u8]| {
    content.fill(0xAB);
    true
})?;

// Exclusive access; the permit ends when the guard is dropped.
{
    let mut content = mem.write()?;
    content[0] = 1;
}

// Keep the write above across rebuilds.
mem.append_modify(|content: &mut [u8]| {
    content[0] = 1;
    true
})?;

// Shared access.
let content = mem.read()?;
assert_eq!(content[0], 1);
```

## Coverage

The whole native API is available since API 10, so no `api-*` feature is
required.

| Item | Wraps |
|---|---|
| `PurgeableMemory::new` | `OH_PurgeableMemory_Create` |
| `PurgeableMemory::read` / `ReadGuard` drop | `OH_PurgeableMemory_BeginRead` / `_EndRead` |
| `PurgeableMemory::write` / `WriteGuard` drop | `OH_PurgeableMemory_BeginWrite` / `_EndWrite` |
| guard `Deref` / `DerefMut`, `PurgeableMemory::len` | `OH_PurgeableMemory_GetContent`, `_ContentSize` |
| `PurgeableMemory::append_modify` | `OH_PurgeableMemory_AppendModify` |
| `PurgeableMemory` drop | `OH_PurgeableMemory_Destroy` |

## Notes

- The content is reachable only through a guard, which ends the access on drop.
  The guard borrows the object (shared for read, exclusive for write), so the
  content slice cannot outlive the permit.
- A begin-access call reports whether the content is present, either because it
  was never reclaimed or because the rebuild succeeded; it does not report
  whether a rebuild happened. If that matters, record it from inside the rebuild
  function. A failure surfaces as `PurgeableMemoryError::ContentPurged`, and no
  guard, hence no end-access call, is produced.
- A plain write through `WriteGuard` is lost if the content is later reclaimed.
  Use `append_modify` for changes that must be replayed on the rebuilt content.
- The rebuild and modify functions are taken as `Fn(&mut [u8]) -> bool + 'static`
  closures. They are owned by the object and released after it is destroyed. The
  system runs them while the object is locked, so they must not call back into
  the same object; a panic inside one is caught and reported as a failed build
  rather than unwinding into C.
- `PurgeableMemory` is neither `Send` nor `Sync`, since the native header
  documents nothing about using one object from several threads.

## License

MIT OR Apache-2.0
