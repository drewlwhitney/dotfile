pub mod pac;

use std::path::PathBuf;
use std::sync::LazyLock;

use dirs;

pub const APP_NAME: &str = "dotfile";

pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| dirs::config_dir().expect("Unable to determine user's config directory").join(APP_NAME));
