use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufReader, LineWriter};
use std::path::{Path, PathBuf};

use super::package_manager::*;

#[cfg(test)]
mod tests;

pub const PACKAGE_MANAGER_FILENAME: &str = "package_manager.toml";
pub const PACKAGES_FILENAME: &str = "installed_packages.txt";
pub const EXCLUDED_PACKAGES_FILENAME: &str = "excluded_packages.txt";

// Contains a `PackageManager` and the folders that store package information.
pub struct PackageSystem {
    pub package_manager: PackageManager,
    pub packages_file: PathBuf,
    pub excluded_packages_file: PathBuf,
}
impl PackageSystem {
    /// Build a new `PackageSystem`.
    /// # Parameters
    /// - `package_manager` - The `PackageManager` to be used by the package system.
    /// - `folder` - The full path to the package system's directory.
    pub fn build(package_manager: PackageManager, folder: impl AsRef<Path>) -> Self {
        let folder = folder.as_ref();
        PackageSystem {
            packages_file: folder.join(PACKAGES_FILENAME),
            excluded_packages_file: folder.join(EXCLUDED_PACKAGES_FILENAME),
            package_manager: package_manager,
        }
    }

    /// Build from a folder alone.
    ///
    /// # Errors
    /// - Failed to read from `folder`.
    /// - An invalid file was detected.
    /// - Any errors from `PackageManager::from_toml()`.
    /// - No package manager file was found.
    ///
    /// # Folder Contents
    /// - `package_manager.toml` - Info about the package manager.
    /// - `installed_packages.txt` - A list of installed packages.
    /// - `excluded_packages.txt` - A list of excluded packages.
    pub fn from_folder(folder: impl AsRef<Path>) -> Result<Self, String> {
        let folder = folder.as_ref();
        // get an iterable of paths in `folder`
        let Ok(paths) = folder.read_dir() else {
            return Err(format!(
                "Could not read package system folder: {}",
                &folder.to_string_lossy()
            ));
        };
        for path in paths {
            // get the actual path
            let path = if let Ok(temp) = path {
                temp.path()
            } else {
                return Err(format!(
                    "Invalid file detected in package system folder: {}",
                    &folder.to_string_lossy()
                ));
            };
            // if the path is the package manager file, create a package manager from it
            if let Some(file_name) = path.file_name() {
                if file_name == PACKAGE_MANAGER_FILENAME {
                    // try to build the package system by trying to load the package manager
                    match PackageManager::from_toml(path) {
                        Ok(package_manager) => return Ok(Self::build(package_manager, folder)),
                        Err(message) => return Err(message),
                    }
                }
            }
        }
        // we get here if no package manager was found; error
        Err(format!("No package manager file found in {}", folder.to_string_lossy()))
    }

    /// Attempt to install packages from the package file.
    /// # Errors
    /// - The packages file could not be read.
    /// - The install command failed.
    pub fn install(&mut self) -> Result<&mut Self, String> {
        let Ok(packages) = read_file_to_vector(&self.packages_file) else {
            return Err(format!(
                "Failed to read packages file: {}",
                self.packages_file.to_string_lossy()
            ));
        };
        if let Err(error_message) = self.package_manager.install(&packages) {
            return Err(error_message);
        } else {
            println!("Successfully installed packages!");
        }

        Ok(self)
    }

    /// Upload a list of installed packages to the packages file, excluding packages in the excluded
    /// packages file.
    /// # Errors
    /// - The list command failed.
    /// - The excluded packages file could not be read.
    /// - The packages file could not be created or truncated.
    /// - The packages file could not be written to.
    pub fn upload(&mut self) -> Result<&mut Self, String> {
        // get the list of installed packages
        let installed_packages = match self.package_manager.list() {
            Ok(package_list) => package_list,
            Err(error_message) => return Err(error_message),
        };
        // get the list of excluded packages
        let Ok(excluded_packages) = read_file_to_vector(&self.excluded_packages_file) else {
            return Err(format!(
                "Failed to read excluded packages file: {}",
                &self.excluded_packages_file.to_string_lossy()
            ));
        };
        // create/truncate the packages file
        let mut packages_file = match File::create(&self.packages_file) {
            Ok(file) => LineWriter::new(file),
            _ => {
                return Err(format!(
                    "Failed to create or truncate packages file: {}",
                    &self.packages_file.to_string_lossy()
                ));
            }
        };
        // write the updated package list to the file, excluding any
        for package in installed_packages {
            // if the package should not be excluded
            if !excluded_packages.contains(&package) {
                if let Err(_) = writeln!(packages_file, "{}", package) {
                    return Err(format!(
                        "Failed to write to packages file: {}",
                        &self.packages_file.to_string_lossy()
                    ));
                }
            }
        }

        Ok(self)
    }

    /// Calls `install()` followed by `upload()`.
    pub fn sync(&mut self) -> Result<&mut Self, String> {
        if let Err(error_message) = self.install() {
            return Err(error_message);
        }
        if let Err(error_message) = self.upload() {
            return Err(error_message);
        }
        return Ok(self);
    }

    /// Exclude packages from `upload()`. Warns the user if any packages that are about to be
    /// excluded are not installed.
    ///
    /// # Errors
    /// - Any errors from `PackageManager.list()`.
    pub fn exclude(&mut self) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// Reinclude packages that were previously excluded. Warns the user if any packages were not
    /// excluded or are not installed.
    ///
    /// # Errors
    /// - Any errors from `PackageManager.list()`.
    pub fn reinclude(&mut self) -> Result<&mut Self, String> {
        Ok(self)
    }
}

/// Read a file into a newline-separated `Vec` of `String`s.
///
/// Parameters
/// - `file` - The file to read from.
///
/// Errors
/// The file cannot be read.
fn read_file_to_vector(file: &impl AsRef<Path>) -> io::Result<Vec<String>> {
    BufReader::new(File::open(file)?).lines().collect()
}
