//! API-23 accessibility providers with a retained native node owner.
use std::{cell::RefCell, rc::Rc};

use ohos_accessibility_binding::{EventInfo, Provider, ProviderCallbacks, ProviderRegistration};

use crate::{ArkUIError, ArkUINode, ArkUIResult};

/// A UI-thread registration that retains its owned Custom node until callbacks retire.
/// Requires the `accessibility` and `api-23` features and an API-23 runtime.
pub struct NodeAccessibility {
    // Callback registration must retire before the final native node reference.
    _registration: ProviderRegistration<'static>,
    _retained: crate::common::node_retention::NodeRetention,
    _owner: Rc<RefCell<ArkUINode>>,
}
impl NodeAccessibility {
    /// Sends an event through the retained provider on its owning UI thread.
    pub fn send_event(&self, event: &EventInfo) {
        self._registration.provider().send_event(event, None);
    }

    /// Registers instance-scoped callbacks on an API-23 or newer runtime.
    /// Only owned Custom nodes may be supplied. No raw pointers escape this wrapper.
    pub fn new<C: ProviderCallbacks>(
        owner: &Rc<RefCell<ArkUINode>>,
        instance: &str,
        callbacks: C,
    ) -> ArkUIResult<Self> {
        if !owner.borrow().is_owned() {
            return Err(invalid("Accessibility requires an owned node"));
        }
        let mut node = owner.borrow().raw_handle();
        let retained = owner.borrow().retain_native()?;
        // SAFETY: registration retires before _owner releases the native node.
        let provider: Provider<'static> = unsafe { Provider::from_node_handle(&mut node) }
            .map_err(|error| invalid(error.to_string()))?;
        let registration = provider
            .register_callbacks_with_instance(instance, callbacks)
            .map_err(|error| invalid(error.to_string()))?;
        Ok(Self {
            _registration: registration,
            _retained: retained,
            _owner: owner.clone(),
        })
    }
}

fn invalid(message: impl AsRef<str>) -> ArkUIError {
    ArkUIError::new(
        crate::arkui_input_binding::ArkUIErrorCode::ParamInvalid,
        message.as_ref(),
    )
}
