//! An owned API 12 NodeContent and its identity-stable native root.

use std::{cell::RefCell, rc::Rc};

#[cfg(target_env = "ohos")]
use napi_ohos::{
    bindgen_prelude::{FromNapiValue, Function},
    JsValue,
};
use napi_ohos::{
    bindgen_prelude::{Object, ObjectRef},
    Env, Result,
};

#[cfg(target_env = "ohos")]
use crate::ArkUIHandle;
use crate::{api::node_content::NodeContent, ArkUINode, ArkUIResult, RootNode};

/// Owns a NodeContent that an ArkTS host places in a standard ContentSlot.
/// All methods and destruction must run on the creating N-API/UI thread.
pub struct ContentRoot {
    owner: Option<ContentOwner>,
}

struct ContentOwner {
    root: RootNode,
    events: NodeContent,
    value: Option<ObjectRef>,
    env: Env,
}

impl ContentRoot {
    /// Constructs the platform NodeContent object and retains its native slot.
    #[cfg(target_env = "ohos")]
    pub fn new(env: Env) -> Result<Self> {
        let module = env.load("@ohos.arkui.node")?;
        let constructor: Function<'_, (), Object<'_>> = module.get("NodeContent")?;
        let value = constructor.new_instance(())?;
        // SAFETY: the system constructor returned a live NodeContent in this scope.
        let object = unsafe { Object::from_napi_value(env.raw(), value.raw()) }?;
        // SAFETY: the object remains retained until after root/callback teardown.
        let handle = unsafe { ArkUIHandle::from_napi_value(env.raw(), object.raw()) }?;
        Ok(Self {
            owner: Some(ContentOwner {
                root: RootNode::new(handle),
                events: NodeContent::from_handle(handle),
                value: Some(object.create_ref()?),
                env,
            }),
        })
    }

    /// Returns the same NodeContent object on every call; mount it in one slot.
    pub fn object<'env>(&self, env: &'env Env) -> Result<Object<'env>> {
        self.owner
            .as_ref()
            .expect("live ContentRoot")
            .value
            .as_ref()
            .expect("retained NodeContent")
            .get_value(env)
    }

    /// Mounts an owned root; descendant ownership stays in this wrapper tree.
    pub fn mount(&mut self, node: Rc<RefCell<ArkUINode>>) -> ArkUIResult<()> {
        self.owner
            .as_mut()
            .expect("live ContentRoot")
            .root
            .mount_node(node)
    }

    /// Detaches and disposes the root. The empty NodeContent remains reusable.
    pub fn unmount(&mut self) -> ArkUIResult<()> {
        self.owner
            .as_mut()
            .expect("live ContentRoot")
            .root
            .unmount()
    }

    /// Observes attachment to a window without an ArkTS lifecycle wrapper.
    pub fn on_window_change(&mut self, callback: impl Fn(bool) + 'static) -> ArkUIResult<()> {
        self.owner
            .as_mut()
            .expect("live ContentRoot")
            .events
            .register_callback(move |event| {
                callback(
                    event.event_type()
                        == crate::types::advanced::NodeContentEventType::OnAttachToWindow,
                );
            })
    }

    /// Removes the observation before releasing the renderer's state.
    pub fn clear_window_callback(&mut self) -> ArkUIResult<()> {
        self.owner
            .as_mut()
            .expect("live ContentRoot")
            .events
            .clear_callback()
    }
}

impl Drop for ContentRoot {
    fn drop(&mut self) {
        // Clear user data while NodeContent is still retained. Native callbacks
        // must never reach a released closure, including during root removal.
        let Some(mut owner) = self.owner.take() else {
            return;
        };
        if owner.events.clear_callback().is_err() || owner.root.unmount().is_err() {
            // Native code still owns an edge or callback. Retain its complete
            // Rust/N-API owner rather than releasing a still-reachable value.
            std::mem::forget(owner);
            return;
        }
        if let Some(value) = owner.value.take() {
            let _ = value.unref(&owner.env);
        }
    }
}
