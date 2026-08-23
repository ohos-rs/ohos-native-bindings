use ohos_enum_derive::EnumFrom;
use ohos_hidebug_sys::*;

/// Which threads an application trace capture covers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(HiDebug_TraceFlag, "HiDebug_TraceFlag_")]
pub enum TraceFlag {
    #[suffix("HIDEBUG_TRACE_FLAG_MAIN_THREAD")]
    MainThread,
    #[suffix("HIDEBUG_TRACE_FLAG_ALL_THREADS")]
    AllThreads,
}

/// Kinds of diagnostic data attachable to the crash context.
///
/// The memory variants name the buffer size hidebug dumps on crash.
#[cfg(feature = "api-23")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(HiDebug_CrashObjType, "HiDebug_CrashObjType_")]
pub enum CrashObjType {
    #[suffix("HIDEBUG_CRASHOBJ_STRING")]
    String,
    #[suffix("HIDEBUG_CRASHOBJ_MEMORY_64B")]
    Memory64B,
    #[suffix("HIDEBUG_CRASHOBJ_MEMORY_256B")]
    Memory256B,
    #[suffix("HIDEBUG_CRASHOBJ_MEMORY_1024B")]
    Memory1024B,
    #[suffix("HIDEBUG_CRASHOBJ_MEMORY_2048B")]
    Memory2048B,
    #[suffix("HIDEBUG_CRASHOBJ_MEMORY_4096B")]
    Memory4096B,
}

/// Resource kinds the resource profiler can sample.
#[cfg(feature = "api-24")]
#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumFrom)]
#[config(OH_HiDebug_ResourceType, "OH_HiDebug_ResourceType_")]
pub enum ResourceType {
    #[suffix("OH_RES_TYPE_FD")]
    Fd,
    #[suffix("OH_RES_TYPE_THREAD")]
    Thread,
    #[suffix("OH_RES_TYPE_NATIVE")]
    Native,
    #[suffix("OH_RES_TYPE_GPU")]
    Gpu,
    #[suffix("OH_RES_TYPE_GLOBAL_HANDLE")]
    GlobalHandle,
}
