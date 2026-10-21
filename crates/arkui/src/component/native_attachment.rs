//! UI-thread attachment targets for externally owned ArkTS content.
//!
//! The creator retains its ArkTS node and must call `retire` synchronously
//! before disposing it. Rust stores only a borrowed native handle. No N-API
//! environment, object, value or reference is retained here.
use crate::{
    api::ARK_UI_NATIVE_NODE_API_1, common::node::ArkUINodeRaw,
    component::attribute::ArkUICommonAttribute, ArkUINode,
};
use napi_ohos::{
    bindgen_prelude::{FromNapiValue, Object},
    Env, Error, JsValue, Result,
};
use std::{
    cell::{Cell, RefCell},
    collections::BTreeMap,
    rc::Rc,
};

#[derive(Default)]
struct Registry {
    next: u32,
    // Keep a failed detachment addressable until its ArkTS owner retries
    // retirement. Forgetting the target would acknowledge a dangling borrow.
    targets: BTreeMap<u32, Rc<Target>>,
}
thread_local! { static TARGETS: RefCell<Registry> = RefCell::default(); }
struct Target {
    id: u32,
    closed: Cell<bool>,
    operation: crate::common::operation::Operation,
    parent: Rc<RefCell<ArkUINode>>,
    parent_retention: RefCell<Option<crate::common::node_retention::NodeRetention>>,
    child: RefCell<Option<Rc<RefCell<ArkUINode>>>>,
    inner: RefCell<Option<Rc<RefCell<ArkUINode>>>>,
    inner_retention: RefCell<Option<crate::common::node_retention::NodeRetention>>,
    slot: RefCell<Option<crate::ArkUIHandle>>,
}
impl Target {
    fn release_owners(&self) {
        self.inner_retention.borrow_mut().take();
        self.parent_retention.borrow_mut().take();
    }
    fn detach(&self) -> Result<()> {
        let _active = self.begin()?;
        self.detach_inner()
    }
    fn begin(&self) -> Result<crate::common::operation::Active<'_>> {
        self.operation
            .enter()
            .ok_or_else(|| Error::from_reason("native attachment operation is in progress"))
    }
    fn detach_inner(&self) -> Result<()> {
        let slot = *self.slot.borrow();
        let inner = self.inner.borrow().clone();
        if let (Some(slot), Some(inner)) = (slot, inner) {
            let slot = crate::api::node_content::NodeContent::from_handle(slot);
            slot.remove_node(&inner.borrow())
                .map_err(|error| Error::from_reason(error.to_string()))?;
            let _ = ARK_UI_NATIVE_NODE_API_1.with(|api| api.remove_event_receiver(&inner.borrow()));
            crate::common::node_registry::unregister(&inner.borrow());
        }
        self.slot.borrow_mut().take();
        let child = self.child.borrow().clone();
        if let Some(child) = child {
            let mut parent = self.parent.borrow_mut();
            let index = parent.children().iter().position(|n| Rc::ptr_eq(n, &child));
            let native_parent = ARK_UI_NATIVE_NODE_API_1
                .with(|api| api.get_parent(&child.borrow()))
                .map_err(|e| Error::from_reason(e.to_string()))?;
            if let Some(native_parent) = native_parent {
                if native_parent != parent.raw_handle() {
                    return Err(Error::from_reason(
                        "native attachment was reparented outside its target",
                    ));
                }
                if let Some(index) = index {
                    parent
                        .remove_child(index)
                        .map_err(|e| Error::from_reason(e.to_string()))?;
                } else {
                    ARK_UI_NATIVE_NODE_API_1
                        .with(|api| api.remove_child(&parent, &child.borrow()))
                        .map_err(|e| Error::from_reason(e.to_string()))?;
                }
            } else if let Some(index) = index {
                // UI-context teardown may already have severed the native edge.
                parent.children_mut().remove(index);
            }
        }
        self.child.borrow_mut().take();
        Ok(())
    }
}
/// An opaque attachment target. Like ArkUI nodes, this type is neither Send
/// nor Sync and must be created, used and dropped on the owning UI thread.
///
/// ```compile_fail
/// use ohos_arkui_binding::component::native_attachment::NativeAttachment;
/// fn require_send<T: Send>() {}
/// require_send::<NativeAttachment>();
/// ```
pub struct NativeAttachment(Rc<Target>);
impl NativeAttachment {
    /// Registers a live owned native container for asynchronous ArkTS construction.
    /// The target retains its native parent wrapper until successful detachment,
    /// including after owner destruction. Borrowed containers are rejected.
    /// The external creator must retain the ArkTS content until retirement.
    pub fn new(parent: &Rc<RefCell<ArkUINode>>) -> Result<Self> {
        if !parent.borrow().is_owned() {
            return Err(Error::from_reason(
                "native attachment requires a live owned container",
            ));
        }
        let retention = parent
            .borrow()
            .retain_native()
            .map_err(|error| Error::from_reason(error.to_string()))?;
        TARGETS.with(|targets| {
            let mut targets = targets.borrow_mut();
            targets.next = targets
                .next
                .checked_add(1)
                .ok_or_else(|| Error::from_reason("native attachment IDs exhausted"))?;
            let target = Rc::new(Target {
                id: targets.next,
                closed: Cell::new(false),
                operation: Default::default(),
                parent: parent.clone(),
                parent_retention: RefCell::new(Some(retention)),
                child: RefCell::default(),
                inner: RefCell::default(),
                inner_retention: RefCell::default(),
                slot: RefCell::default(),
            });
            targets.targets.insert(target.id, target.clone());
            Ok(Self(target))
        })
    }
    /// Non-reused ID accepted by the matching module's UI-thread registry.
    pub fn id(&self) -> u32 {
        self.0.id
    }
    /// Registers an owned subtree to be hosted inside an ArkTS wrapper's NodeContent.
    /// Both native nodes stay owned by Rust; ArkTS owns only the wrapper and slot.
    pub fn for_subtree(
        parent: &Rc<RefCell<ArkUINode>>,
        inner: &Rc<RefCell<ArkUINode>>,
    ) -> Result<Self> {
        if !inner.borrow().is_owned() || Rc::ptr_eq(parent, inner) {
            return Err(Error::from_reason(
                "A subtree attachment needs distinct owned nodes",
            ));
        }
        let target = Self::new(parent)?;
        let retention = inner
            .borrow()
            .retain_native()
            .map_err(|error| Error::from_reason(error.to_string()))?;
        *target.0.inner.borrow_mut() = Some(inner.clone());
        *target.0.inner_retention.borrow_mut() = Some(retention);
        Ok(target)
    }
    /// Attaches an externally owned visual wrapper around the registered native subtree.
    ///
    /// # Safety
    /// The ArkTS owner must retain both objects until synchronous retirement succeeds.
    pub unsafe fn attach_subtree(
        env: &Env,
        id: u32,
        content: &Object<'_>,
        slot: &Object<'_>,
    ) -> Result<bool> {
        let target = TARGETS.with(|targets| targets.borrow().targets.get(&id).cloned());
        let Some(target) = target.filter(|target| !target.closed.get()) else {
            return Ok(false);
        };
        let _active = target.begin()?;
        let inner = target
            .inner
            .borrow()
            .clone()
            .ok_or_else(|| Error::from_reason("Target is not a subtree attachment"))?;
        if target.slot.borrow().is_some() {
            return Err(Error::from_reason("Subtree slot is already attached"));
        }
        // SAFETY: the caller keeps the NodeContent and wrapper alive through retirement.
        let handle = unsafe { crate::ArkUIHandle::from_napi_value(env.raw(), slot.raw()) }?;
        // SAFETY: the same external owner contract covers the wrapper FrameNode.
        if !unsafe { Self::attach_target(&target, env, content) }? {
            return Ok(false);
        }
        let slot = crate::api::node_content::NodeContent::from_handle(handle);
        if let Err(error) = slot.add_node(&inner.borrow()) {
            target.detach_inner()?;
            return Err(Error::from_reason(error.to_string()));
        }
        *target.slot.borrow_mut() = Some(handle);
        // A C parent removes its child's receiver on detach. NodeContent must
        // restore that same identity route when accepting the moved subtree.
        if let Err(error) = crate::common::node_registry::register(&inner.borrow(), inner.clone())
            .and_then(|()| {
                ARK_UI_NATIVE_NODE_API_1.with(|api| api.add_event_receiver(&inner.borrow()))
            })
        {
            target.detach_inner()?;
            return Err(Error::from_reason(error.to_string()));
        }
        Ok(true)
    }
    /// Uses the borrowed child while it is still attached and externally owned.
    /// The node reference cannot escape the callback or survive retirement.
    pub fn with_child<R>(&self, use_child: impl FnOnce(&ArkUINode) -> R) -> Option<R> {
        let _active = self.0.begin().ok()?;
        let child = self.0.child.try_borrow().ok()?.clone()?;
        let child = child.try_borrow().ok()?;
        Some(use_child(&child))
    }
    /// Detaches content before asking the external creator to dispose it.
    pub fn detach(&self) -> Result<()> {
        self.0.detach()
    }
    /// Permanently invalidates the target and detaches its child. Late ready
    /// events are rejected even while the retiring Rust owner remains alive.
    pub fn invalidate(&self) -> Result<()> {
        self.0.closed.set(true);
        self.detach()?;
        TARGETS.with(|targets| {
            targets.borrow_mut().targets.remove(&self.0.id);
        });
        self.0.release_owners();
        Ok(())
    }
    /// Converts callback-scoped ArkTS content and mounts its native handle.
    /// Returns false when the target has already been disposed. The creator
    /// must then release the unmounted content itself.
    ///
    /// # Safety
    /// The caller must retain the ArkTS owner until `retire` succeeds. It
    /// must not dispose its BuilderNode or FrameNode after failed retirement.
    pub unsafe fn attach(env: &Env, id: u32, content: &Object<'_>) -> Result<bool> {
        let target = TARGETS.with(|targets| targets.borrow().targets.get(&id).cloned());
        let Some(target) = target else {
            return Ok(false);
        };
        let _active = target.begin()?;
        // SAFETY: the public caller retains the external ArkTS owner through retirement.
        unsafe { Self::attach_target(&target, env, content) }
    }
    unsafe fn attach_target(target: &Target, env: &Env, content: &Object<'_>) -> Result<bool> {
        if target.closed.get() {
            return Ok(false);
        }
        let parent = &target.parent;
        if !parent.borrow().is_owned() {
            return Ok(false);
        }
        if target.child.borrow().is_some() {
            return Err(Error::from_reason("native attachment is already occupied"));
        }
        // SAFETY: the creator owns the live content during this callback and
        // until retiring; this target only borrows the resulting native node.
        let raw = unsafe { ArkUINodeRaw::from_napi_value(env.raw(), content.raw()) }?;
        let child = Rc::new(RefCell::new(
            // SAFETY: attach's caller retains the external owner until retirement.
            unsafe { raw.borrowed_node() }.map_err(|e| Error::from_reason(e.to_string()))?,
        ));
        parent
            .borrow_mut()
            .add_existing_node(child.clone())
            .map_err(|e| Error::from_reason(e.to_string()))?;
        *target.child.borrow_mut() = Some(child);
        Ok(true)
    }
    /// Called synchronously before an ArkTS owner releases its node.
    /// A target that was already disposed is acknowledged idempotently.
    pub fn retire(id: u32) -> Result<()> {
        let target = TARGETS.with(|targets| targets.borrow().targets.get(&id).cloned());
        if let Some(target) = target {
            target.detach()?;
            if target.closed.get() {
                TARGETS.with(|targets| targets.borrow_mut().targets.remove(&id));
                target.release_owners();
            }
        }
        Ok(())
    }
}
impl Drop for NativeAttachment {
    fn drop(&mut self) {
        // On failure the registry retains the target for the required
        // synchronous retiring event; it never releases a borrowed child.
        let _ = self.invalidate();
    }
}
