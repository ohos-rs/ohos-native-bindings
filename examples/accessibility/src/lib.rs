#![allow(clippy::all)]

use std::{
    cell::RefCell,
    sync::atomic::{AtomicU32, Ordering},
};

use accesskit::{
    Action, ActionHandler, ActionRequest, ActivationHandler, Node, NodeId, Rect, Role, Tree,
    TreeUpdate,
};
use accesskit_ohos::Adapter;
use napi_derive_ohos::napi;
use napi_ohos::{bindgen_prelude::Object, Env, Error, Result};
use ohos_accessibility_binding::Provider;
use ohos_arkui_binding::{
    component::{
        attribute::{ArkUIAttributeBasic, ArkUICommonAttribute},
        built_in_component::Custom,
    },
    ArkUIHandle, RootNode,
};
use ohos_xcomponent_binding::XComponent;

const ROOT_ID: NodeId = NodeId(0);
const BUTTON_ID: NodeId = NodeId(1);

static ARKUI_ACTIVATIONS: AtomicU32 = AtomicU32::new(0);
static ARKUI_ACTIONS: AtomicU32 = AtomicU32::new(0);
static ARKUI_UPDATES: AtomicU32 = AtomicU32::new(0);
static XCOMPONENT_ACTIVATIONS: AtomicU32 = AtomicU32::new(0);
static XCOMPONENT_ACTIONS: AtomicU32 = AtomicU32::new(0);
static XCOMPONENT_UPDATES: AtomicU32 = AtomicU32::new(0);

thread_local! {
    static ARKUI_ADAPTER: RefCell<Option<Adapter<'static>>> = const { RefCell::new(None) };
    static XCOMPONENT_ADAPTER: RefCell<Option<Adapter<'static>>> = const { RefCell::new(None) };
}

#[derive(Clone, Copy)]
enum SurfaceKind {
    ArkUi,
    XComponent,
}

impl SurfaceKind {
    fn label(self, actions: u32) -> String {
        let owner = match self {
            Self::ArkUi => "ArkUI",
            Self::XComponent => "XComponent",
        };
        if actions == 0 {
            format!("{owner} AccessKit Button")
        } else {
            format!("{owner} AccessKit Button clicked {actions}")
        }
    }

    fn activation_counter(self) -> &'static AtomicU32 {
        match self {
            Self::ArkUi => &ARKUI_ACTIVATIONS,
            Self::XComponent => &XCOMPONENT_ACTIVATIONS,
        }
    }

    fn action_counter(self) -> &'static AtomicU32 {
        match self {
            Self::ArkUi => &ARKUI_ACTIONS,
            Self::XComponent => &XCOMPONENT_ACTIONS,
        }
    }

    fn update_counter(self) -> &'static AtomicU32 {
        match self {
            Self::ArkUi => &ARKUI_UPDATES,
            Self::XComponent => &XCOMPONENT_UPDATES,
        }
    }
}

struct ExampleActivationHandler {
    kind: SurfaceKind,
}

impl ActivationHandler for ExampleActivationHandler {
    fn request_initial_tree(&mut self) -> Option<TreeUpdate> {
        self.kind
            .activation_counter()
            .fetch_add(1, Ordering::Relaxed);
        Some(full_tree(self.kind))
    }
}

struct ExampleActionHandler {
    kind: SurfaceKind,
}

impl ActionHandler for ExampleActionHandler {
    fn do_action(&mut self, request: ActionRequest) {
        if request.target == BUTTON_ID && request.action == Action::Click {
            self.kind.action_counter().fetch_add(1, Ordering::Relaxed);
        }
    }
}

fn button_node(kind: SurfaceKind) -> Node {
    let actions = kind.action_counter().load(Ordering::Relaxed);
    let mut button = Node::new(Role::Button);
    button.set_label(kind.label(actions));
    button.set_bounds(Rect::new(24.0, 24.0, 336.0, 104.0));
    button.add_action(Action::Click);
    button
}

fn full_tree(kind: SurfaceKind) -> TreeUpdate {
    let mut root = Node::new(Role::Window);
    root.set_children(vec![BUTTON_ID]);
    root.set_bounds(Rect::new(0.0, 0.0, 360.0, 128.0));
    TreeUpdate {
        nodes: vec![(ROOT_ID, root), (BUTTON_ID, button_node(kind))],
        tree: Some(Tree::new(ROOT_ID)),
        focus: BUTTON_ID,
    }
}

fn button_update(kind: SurfaceKind) -> TreeUpdate {
    TreeUpdate {
        nodes: vec![(BUTTON_ID, button_node(kind))],
        tree: None,
        focus: BUTTON_ID,
    }
}

fn to_napi_error(error: impl std::fmt::Display) -> Error {
    Error::from_reason(error.to_string())
}

/// Extend the provider borrow while the example separately owns the native
/// object for at least as long as the adapter.
unsafe fn provider_for_owned_host(provider: Provider<'_>) -> Result<Provider<'static>> {
    unsafe { Provider::from_raw(provider.as_raw()) }.map_err(to_napi_error)
}

fn make_adapter(provider: Provider<'static>, kind: SurfaceKind) -> Result<Adapter<'static>> {
    Adapter::new(
        provider,
        ExampleActivationHandler { kind },
        ExampleActionHandler { kind },
    )
    .map_err(to_napi_error)
}

fn refresh_adapter(adapter: &RefCell<Option<Adapter<'static>>>, kind: SurfaceKind) -> Result<()> {
    let actions = kind.action_counter().load(Ordering::Relaxed);
    let updates = kind.update_counter().load(Ordering::Relaxed);
    if actions <= updates {
        return Ok(());
    }
    if let Some(adapter) = adapter.borrow().as_ref() {
        adapter
            .update_if_active(|| button_update(kind))
            .map_err(to_napi_error)?;
        kind.update_counter().store(actions, Ordering::Relaxed);
    }
    Ok(())
}

#[napi]
pub struct ArkUiAccessibilityApp {
    // Drop clears the adapter before RootNode releases its custom node.
    root: RootNode,
    mounted: bool,
}

#[napi]
impl ArkUiAccessibilityApp {
    #[napi(constructor)]
    pub fn new(#[napi(ts_arg_type = "NodeContent")] slot: ArkUIHandle) -> Self {
        Self {
            root: RootNode::new(slot),
            mounted: false,
        }
    }

    #[napi]
    pub fn mount(&mut self) -> Result<()> {
        if self.mounted {
            return Ok(());
        }
        let custom = Custom::new().map_err(to_napi_error)?;
        custom.width(360.0).map_err(to_napi_error)?;
        custom.height(128.0).map_err(to_napi_error)?;
        custom.background_color(0xffe8f5e9).map_err(to_napi_error)?;

        let provider = custom
            .raw()
            .accessibility_provider()
            .map_err(to_napi_error)?;
        // Safety: `self.root` owns the mounted custom node, and Drop removes
        // the adapter before RootNode releases that node.
        let provider = unsafe { provider_for_owned_host(provider)? };
        let adapter = make_adapter(provider, SurfaceKind::ArkUi)?;
        self.root.mount(custom).map_err(to_napi_error)?;
        ARKUI_ADAPTER.with(|slot| *slot.borrow_mut() = Some(adapter));
        self.mounted = true;
        Ok(())
    }

    #[napi]
    pub fn unmount(&mut self) -> Result<()> {
        ARKUI_ADAPTER.with(|slot| slot.borrow_mut().take());
        self.root.unmount().map_err(to_napi_error)?;
        self.mounted = false;
        Ok(())
    }
}

impl Drop for ArkUiAccessibilityApp {
    fn drop(&mut self) {
        ARKUI_ADAPTER.with(|slot| slot.borrow_mut().take());
    }
}

#[napi(module_exports)]
pub fn init(exports: Object<'_>, env: Env) -> Result<()> {
    if XCOMPONENT_ADAPTER.with(|slot| slot.borrow().is_some()) {
        return Ok(());
    }
    let xcomponent = match XComponent::init(env, exports) {
        Ok(value) => value,
        Err(_) => return Ok(()),
    };
    let provider = xcomponent.accessibility_provider().map_err(to_napi_error)?;
    // Safety: ArkUI owns this XComponent for the native module lifetime. The
    // adapter is thread-local and is dropped when that UI thread exits.
    let provider = unsafe { provider_for_owned_host(provider)? };
    let adapter = make_adapter(provider, SurfaceKind::XComponent)?;
    XCOMPONENT_ADAPTER.with(|slot| *slot.borrow_mut() = Some(adapter));
    Ok(())
}

#[napi]
pub fn refresh_accessibility_trees() -> Result<()> {
    ARKUI_ADAPTER.with(|adapter| refresh_adapter(adapter, SurfaceKind::ArkUi))?;
    XCOMPONENT_ADAPTER.with(|adapter| refresh_adapter(adapter, SurfaceKind::XComponent))?;
    Ok(())
}

#[napi]
pub fn accessibility_status() -> String {
    format!(
        "arkuiActivated={} arkuiActions={} arkuiUpdates={} xcomponentActivated={} xcomponentActions={} xcomponentUpdates={}",
        ARKUI_ACTIVATIONS.load(Ordering::Relaxed),
        ARKUI_ACTIONS.load(Ordering::Relaxed),
        ARKUI_UPDATES.load(Ordering::Relaxed),
        XCOMPONENT_ACTIVATIONS.load(Ordering::Relaxed),
        XCOMPONENT_ACTIONS.load(Ordering::Relaxed),
        XCOMPONENT_UPDATES.load(Ordering::Relaxed),
    )
}
