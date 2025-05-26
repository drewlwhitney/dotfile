pub mod package_manager;
pub mod parser;
use package_manager::*;

#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{self, BufReader};
use std::io::{LineWriter, prelude::*};
use std::path::{Path, PathBuf};

const PACKAGES_FILENAME: &str = "installed_packages.txt";
const EXCLUDED_PACKAGES_FILENAME: &str = "excluded_packages.txt";

// Contains a `PackageManager` and the folders that store package information.
pub struct PackageSystem {
    pub package_manager: PackageManager,
    pub packages_file: PathBuf,
    pub excluded_packages_file: PathBuf,
}
impl PackageSystem {
    /// Build a new `PackageSystem`.
    ///
    /// # Parameters
    /// - `package_manager` - The `PackageManager` to be used by the package system.
    /// - `folder` - The full path to the package system's directory.
    pub fn build(package_manager: PackageManager, folder: &str) -> Self {
        PackageSystem {
            packages_file: PathBuf::from(folder).join(PACKAGES_FILENAME),
            excluded_packages_file: PathBuf::from(folder).join(EXCLUDED_PACKAGES_FILENAME),
            package_manager: package_manager,
        }
    }

    /// Attempt to install packages from the package file.
    ///
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
    ///
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
