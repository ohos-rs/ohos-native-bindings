// Compile the owning build-script modules without calling the generation entry.
// The config files reference the crate root's SysConfig by repository convention.
#[allow(dead_code)]
#[path = "../build/main.rs"]
mod generator;
use generator::{config, SysConfig};
