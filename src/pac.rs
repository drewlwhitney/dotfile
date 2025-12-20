pub mod cli;
pub mod package_manager;
pub mod package_system;
pub mod parser;

use std::path::PathBuf;
use std::sync::LazyLock;

pub use package_manager::PackageManager;
pub use package_system::{PackageSystem, new_package_system};
pub use parser::*;

use super::CONFIG_DIR;

pub static PAC_DIR: LazyLock<PathBuf> = LazyLock::new(|| CONFIG_DIR.join("package_managers"));
