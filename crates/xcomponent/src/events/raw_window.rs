use std::{
    os::raw::c_void,
    sync::{LazyLock, RwLock},
};

pub(crate) struct RawWindow(pub *mut c_void);

unsafe impl Send for RawWindow {}
unsafe impl Sync for RawWindow {}

pub(crate) static RAW_WINDOW: LazyLock<RwLock<Option<RawWindow>>> =
    LazyLock::new(|| RwLock::new(None));
