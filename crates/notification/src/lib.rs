//! Safe Rust bindings for the OpenHarmony **notification** module.
//!
//! The native `notification.h` C API of NotificationKit exposes a single entry
//! point: a query telling the application whether the user currently allows it
//! to publish notifications. This crate wraps that entry point with a safe
//! layer.
//!
//! The whole surface was introduced in API 13 and therefore sits behind the
//! `api-13` feature; with the default feature set this crate only re-exports
//! the raw bindings as [`sys`].
//!
//! Publishing notifications, notification slots and subscriptions are not part
//! of the C API — they are only reachable from ArkTS through
//! `@ohos.notificationManager`. There is consequently no handle to own, no
//! buffer to hand over and no error code to map, so this crate exposes a plain
//! predicate rather than a `Result`.

pub use ohos_notification_sys as sys;

/// Whether this application is currently allowed to publish notifications.
///
/// Wraps `OH_Notification_IsNotificationEnabled`. The native call takes no
/// argument, allocates nothing and returns a plain `bool`, so there is nothing
/// to release and no failure to report: a denied permission and a failed query
/// are alike reported as `false`.
///
/// The answer reflects the notification switch the user controls in system
/// settings and may change at any time, so it is best queried right before
/// publishing rather than cached.
///
/// # Example
///
/// ```no_run
/// use ohos_notification_binding as notification;
///
/// if notification::is_notification_enabled() {
///     // publish through the ArkTS notification manager
/// } else {
///     // fall back to an in-app hint, or guide the user to system settings
/// }
/// ```
#[cfg(feature = "api-13")]
pub fn is_notification_enabled() -> bool {
    unsafe { sys::OH_Notification_IsNotificationEnabled() }
}
