#[cfg(any(feature = "api-14", feature = "api-21"))]
use crate::error::{check, Result};
use ohos_hidebug_sys as sys;

/// System-wide memory sizes, in kibibytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct SystemMemInfo {
    pub total_mem: u32,
    pub free_mem: u32,
    pub available_mem: u32,
}

/// Native memory footprint of the current application process, in kibibytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct NativeMemInfo {
    pub pss: u32,
    pub vss: u32,
    pub rss: u32,
    pub shared_dirty: u32,
    pub private_dirty: u32,
    pub shared_clean: u32,
    pub private_clean: u32,
}

/// Memory limits of the current application process, in kibibytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct MemoryLimit {
    pub rss_limit: u64,
    pub vss_limit: u64,
}

/// Graphics memory of the current application process, in kibibytes.
#[cfg(feature = "api-21")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct GraphicsMemorySummary {
    pub gl: u32,
    pub graph: u32,
}

/// System memory sizes. An all-zero result means the query failed.
pub fn system_mem_info() -> SystemMemInfo {
    // SAFETY: every field is an integer, so the zeroed struct is a valid input
    // that hidebug overwrites.
    let mut raw: sys::HiDebug_SystemMemInfo = unsafe { std::mem::zeroed() };
    // SAFETY: raw points at an owned, initialized struct.
    unsafe { sys::OH_HiDebug_GetSystemMemInfo(&mut raw) };
    SystemMemInfo {
        total_mem: raw.totalMem,
        free_mem: raw.freeMem,
        available_mem: raw.availableMem,
    }
}

fn native_mem_info_from(raw: &sys::HiDebug_NativeMemInfo) -> NativeMemInfo {
    NativeMemInfo {
        pss: raw.pss,
        vss: raw.vss,
        rss: raw.rss,
        shared_dirty: raw.sharedDirty,
        private_dirty: raw.privateDirty,
        shared_clean: raw.sharedClean,
        private_clean: raw.privateClean,
    }
}

/// Native memory footprint of this process. An all-zero result means the query
/// failed.
pub fn app_native_mem_info() -> NativeMemInfo {
    // SAFETY: every field is an integer, so the zeroed struct is a valid input.
    let mut raw: sys::HiDebug_NativeMemInfo = unsafe { std::mem::zeroed() };
    // SAFETY: raw points at an owned, initialized struct.
    unsafe { sys::OH_HiDebug_GetAppNativeMemInfo(&mut raw) };
    native_mem_info_from(&raw)
}

/// Native memory footprint of this process, served from a cache that hidebug
/// keeps for five minutes. `force_refresh` bypasses and refreshes the cache.
#[cfg(feature = "api-20")]
pub fn app_native_mem_info_cached(force_refresh: bool) -> NativeMemInfo {
    // SAFETY: every field is an integer, so the zeroed struct is a valid input.
    let mut raw: sys::HiDebug_NativeMemInfo = unsafe { std::mem::zeroed() };
    // SAFETY: raw points at an owned, initialized struct.
    unsafe { sys::OH_HiDebug_GetAppNativeMemInfoWithCache(&mut raw, force_refresh) };
    native_mem_info_from(&raw)
}

/// Memory limits of this process. An all-zero result means the query failed.
pub fn app_memory_limit() -> MemoryLimit {
    // SAFETY: every field is an integer, so the zeroed struct is a valid input.
    let mut raw: sys::HiDebug_MemoryLimit = unsafe { std::mem::zeroed() };
    // SAFETY: raw points at an owned, initialized struct.
    unsafe { sys::OH_HiDebug_GetAppMemoryLimit(&mut raw) };
    MemoryLimit {
        rss_limit: raw.rssLimit,
        vss_limit: raw.vssLimit,
    }
}

/// Graphics memory of this process, in kibibytes.
#[cfg(feature = "api-14")]
pub fn graphics_memory() -> Result<u32> {
    let mut value = 0u32;
    // SAFETY: value points at an owned u32.
    unsafe { check(sys::OH_HiDebug_GetGraphicsMemory(&mut value))? };
    Ok(value)
}

/// Graphics memory of this process, split by kind.
///
/// `interval` is the maximum cache age in seconds; values outside 2..=3600 fall
/// back to the hidebug default of 300.
#[cfg(feature = "api-21")]
pub fn graphics_memory_summary(interval: u32) -> Result<GraphicsMemorySummary> {
    // SAFETY: every field is an integer, so the zeroed struct is a valid input.
    let mut raw: sys::HiDebug_GraphicsMemorySummary = unsafe { std::mem::zeroed() };
    // SAFETY: raw points at an owned, initialized struct.
    unsafe { check(sys::OH_HiDebug_GetGraphicsMemorySummary(interval, &mut raw))? };
    Ok(GraphicsMemorySummary {
        gl: raw.gl,
        graph: raw.graph,
    })
}
