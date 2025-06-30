//! Utilities for testing.
use std::fs;
use std::path::{Path, PathBuf};

/// Automatically removes the specified path when dropped.
pub struct PathRemover {
    path_to_remove: PathBuf,
}
impl PathRemover {
    pub fn new(path_to_remove: impl AsRef<Path>) -> Self {
        Self {
            path_to_remove: path_to_remove.as_ref().to_path_buf(),
        }
    }
}
impl Drop for PathRemover {
    fn drop(&mut self) {
        if self.path_to_remove.is_file() {
            fs::remove_file(&self.path_to_remove).unwrap();
        } else {
            fs::remove_dir_all(&self.path_to_remove).unwrap();
        }
    }
}
