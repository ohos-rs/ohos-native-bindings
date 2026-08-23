//! The predefined domain, event and parameter names of HiAppEvent.

use ohos_hiappevent_sys as sys;

/// Drop the trailing NUL from a bindgen string macro.
const fn name(bytes: &'static [u8]) -> &'static str {
    let (text, _) = bytes.split_at(bytes.len() - 1);
    match std::str::from_utf8(text) {
        Ok(text) => text,
        Err(_) => panic!("HiAppEvent name is not valid UTF-8"),
    }
}

/// Event domains.
pub mod domain {
    use super::name;
    use super::sys;

    /// The domain of the events predefined by the OS.
    pub const OS: &str = name(sys::DOMAIN_OS);
}

/// Predefined event names.
///
/// The `hiappevent.*` ones are written by the application; the rest are OS
/// events in [`domain::OS`](super::domain::OS) that a
/// [`Watcher`](crate::Watcher) can subscribe to.
pub mod event {
    use super::name;
    use super::sys;

    pub const USER_LOGIN: &str = name(sys::EVENT_USER_LOGIN);
    pub const USER_LOGOUT: &str = name(sys::EVENT_USER_LOGOUT);
    pub const DISTRIBUTED_SERVICE_START: &str = name(sys::EVENT_DISTRIBUTED_SERVICE_START);

    pub const APP_CRASH: &str = name(sys::EVENT_APP_CRASH);
    pub const APP_FREEZE: &str = name(sys::EVENT_APP_FREEZE);
    pub const APP_LAUNCH: &str = name(sys::EVENT_APP_LAUNCH);
    #[cfg(feature = "api-18")]
    pub const APP_HICOLLIE: &str = name(sys::EVENT_APP_HICOLLIE);
    #[cfg(feature = "api-20")]
    pub const APP_KILLED: &str = name(sys::EVENT_APP_KILLED);
    pub const SCROLL_JANK: &str = name(sys::EVENT_SCROLL_JANK);
    #[cfg(feature = "api-21")]
    pub const AUDIO_JANK_FRAME: &str = name(sys::EVENT_AUDIO_JANK_FRAME);
    pub const MAIN_THREAD_JANK: &str = name(sys::EVENT_MAIN_THREAD_JANK);
    #[cfg(feature = "api-22")]
    pub const MAIN_THREAD_JANK_V2: &str = name(sys::EVENT_MAIN_THREAD_JANK_V2);
    pub const CPU_USAGE_HIGH: &str = name(sys::EVENT_CPU_USAGE_HIGH);
    pub const BATTERY_USAGE: &str = name(sys::EVENT_BATTERY_USAGE);
    pub const RESOURCE_OVERLIMIT: &str = name(sys::EVENT_RESOURCE_OVERLIMIT);
    pub const ADDRESS_SANITIZER: &str = name(sys::EVENT_ADDRESS_SANITIZER);
}

/// Predefined event parameter names.
pub mod param {
    use super::name;
    use super::sys;

    pub const USER_ID: &str = name(sys::PARAM_USER_ID);
    pub const DISTRIBUTED_SERVICE_NAME: &str = name(sys::PARAM_DISTRIBUTED_SERVICE_NAME);
    pub const DISTRIBUTED_SERVICE_INSTANCE_ID: &str =
        name(sys::PARAM_DISTRIBUTED_SERVICE_INSTANCE_ID);

    /// `EventConfig` items for
    /// [`event::MAIN_THREAD_JANK`](super::event::MAIN_THREAD_JANK).
    #[cfg(feature = "api-22")]
    pub mod main_thread_jank {
        use super::name;
        use super::sys;

        pub const LOG_TYPE: &str = name(sys::MAIN_THREAD_JANK_PARAM_LOG_TYPE);
        pub const SAMPLE_INTERVAL: &str = name(sys::MAIN_THREAD_JANK_PARAM_SAMPLE_INTERVAL);
        pub const IGNORE_STARTUP_TIME: &str = name(sys::MAIN_THREAD_JANK_PARAM_IGNORE_STARTUP_TIME);
        pub const SAMPLE_COUNT: &str = name(sys::MAIN_THREAD_JANK_PARAM_SAMPLE_COUNT);
        pub const REPORT_TIMES_PER_APP: &str =
            name(sys::MAIN_THREAD_JANK_PARAM_REPORT_TIMES_PER_APP);
        pub const AUTO_STOP_SAMPLING: &str = name(sys::MAIN_THREAD_JANK_PARAM_AUTO_STOP_SAMPLING);
    }

    /// `EventConfig` items for [`event::APP_CRASH`](super::event::APP_CRASH).
    #[cfg(feature = "api-24")]
    pub mod app_crash {
        use super::name;
        use super::sys;

        pub const EXTEND_PC_LR_PRINTING: &str = name(sys::OH_APP_CRASH_PARAM_EXTEND_PC_LR_PRINTING);
        pub const LOG_FILE_CUTOFF_SZ_BYTES: &str =
            name(sys::OH_APP_CRASH_PARAM_LOG_FILE_CUTOFF_SZ_BYTES);
        pub const SIMPLIFY_VMA_PRINTING: &str = name(sys::OH_APP_CRASH_PARAM_SIMPLIFY_VMA_PRINTING);
        pub const MERGE_CPPCRASH_APP_LOG: &str =
            name(sys::OH_APP_CRASH_PARAM_MERGE_CPPCRASH_APP_LOG);
    }
}

/// Item names accepted by [`configure`](crate::configure).
pub mod config_item {
    use super::name;
    use super::sys;

    /// Whether event logging is turned off.
    pub const DISABLE: &str = name(sys::DISABLE);
    /// Quota of the event file directory, e.g. `"100M"`.
    pub const MAX_STORAGE: &str = name(sys::MAX_STORAGE);
}
