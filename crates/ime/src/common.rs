use std::{
    collections::HashMap,
    sync::{LazyLock, RwLock},
};

use crate::IME;

pub static IME_INSTANCE: LazyLock<RwLock<HashMap<usize, Box<IME>>>> =
    LazyLock::new(|| RwLock::new(HashMap::new()));
