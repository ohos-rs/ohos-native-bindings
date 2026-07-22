//! Trace tag constants for [`start_app_trace_capture`](crate::start_app_trace_capture).
//!
//! Each constant is a single-bit [`TraceTags`]; combine several with `|`.

use crate::trace::TraceTags;
use ohos_hidebug_sys as sys;

pub const FFRT: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_FFRT as u64);
pub const COMMON_LIBRARY: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_COMMON_LIBRARY as u64);
pub const HDF: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_HDF as u64);
pub const NET: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_NET as u64);
pub const NWEB: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_NWEB as u64);
pub const DISTRIBUTED_AUDIO: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_AUDIO as u64);
pub const FILE_MANAGEMENT: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_FILE_MANAGEMENT as u64);
pub const OHOS: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_OHOS as u64);
pub const ABILITY_MANAGER: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_ABILITY_MANAGER as u64);
pub const CAMERA: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_CAMERA);
pub const MEDIA: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_MEDIA);
pub const IMAGE: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_IMAGE);
pub const AUDIO: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_AUDIO);
pub const DISTRIBUTED_DATA: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_DATA);
pub const GRAPHICS: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_GRAPHICS);
pub const ARKUI: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_ARKUI);
pub const NOTIFICATION: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_NOTIFICATION);
pub const MISC: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_MISC);
pub const MULTIMODAL_INPUT: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_MULTIMODAL_INPUT);
pub const RPC: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_RPC);
pub const ARK: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_ARK);
pub const WINDOW_MANAGER: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_WINDOW_MANAGER);
pub const DISTRIBUTED_SCREEN: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_SCREEN);
pub const DISTRIBUTED_CAMERA: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_CAMERA);
pub const DISTRIBUTED_HARDWARE_FRAMEWORK: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_HARDWARE_FRAMEWORK);
pub const GLOBAL_RESOURCE_MANAGER: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_GLOBAL_RESOURCE_MANAGER);
pub const DISTRIBUTED_HARDWARE_DEVICE_MANAGER: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_HARDWARE_DEVICE_MANAGER);
pub const SAMGR: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_SAMGR);
pub const POWER_MANAGER: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_POWER_MANAGER);
pub const DISTRIBUTED_SCHEDULER: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_SCHEDULER);
pub const DISTRIBUTED_INPUT: TraceTags =
    TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_DISTRIBUTED_INPUT);
pub const BLUETOOTH: TraceTags = TraceTags::from_bits(sys::HIDEBUG_TRACE_TAG_BLUETOOTH);
