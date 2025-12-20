use std::fs::{self, File};
use std::io::LineWriter;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

use utils;

use super::package_manager::*;

#[cfg(test)]
mod tests;

pub const PACKAGE_MANAGER_FILENAME: &str = "package-manager.toml";
pub const PACKAGES_FILENAME: &str = "installed-packages.txt";
pub const EXCLUDED_PACKAGES_FILENAME: &str = "excluded-packages.txt";

// Contains a `PackageManager` and the files that store package information.
pub struct PackageSystem {
    name: String,
    package_manager: PackageManager,
    packages_file: PathBuf,
    excluded_packages_file: PathBuf,
}
impl PackageSystem {
    /// Build a new `PackageSystem`.
    /// # Parameters
    /// - `name` - The package system's name.
    /// - `package_manager` - The `PackageManager` to be used by the package system.
    /// - `folder` - The full path to the package system's directory.
    //# UNIT TESTED
    pub fn build(name: String, folder: impl AsRef<Path>, package_manager: PackageManager) -> Self {
        let folder = folder.as_ref();
        PackageSystem {
            name,
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
    /// The folder must contain:
    /// - `package_manager.toml` - Info about the package manager.
    ///
    /// The folder *should* contain:
    /// - `installed_packages.txt` - A list of installed packages.
    /// - `excluded_packages.txt` - A list of excluded packages.
    ///
    /// If the above two aren't present, this function could still succeed, but
    /// the package system will fail later.
    //# UNIT TESTED
    pub fn from_folder(folder: impl AsRef<Path>) -> Result<Self, String> {
        let folder = folder.as_ref();
        // get the package manager from a file
        let package_manager_file = folder.join(PACKAGE_MANAGER_FILENAME);
        let package_manager = PackageManager::from_toml(&package_manager_file)?;
        // get the package system's name from the folder name
        let name = match folder.file_name() {
            Some(name_os) => name_os.to_string_lossy().to_string(),
            _ => {
                return Err(format!("Invalid package system folder: {}", folder.to_string_lossy()));
            }
        };

        Ok(Self::build(name, &folder, package_manager))
    }

    /// Attempt to install packages from the package file.
    /// # Errors
    /// - The packages file could not be read.
    /// - The install command failed.
    //# HELPERS TESTED
    pub fn install(&mut self) -> Result<&mut Self, String> {
        let Ok(packages) = utils::read_file_to_hashset(&self.packages_file) else {
            return Err(format!("Failed to read packages file: {}", self.packages_file.to_string_lossy()));
        };
        self.package_manager.install(&packages)?;
        println!("Successfully installed packages!");

        Ok(self)
    }

    /// Upload a list of installed packages to the packages file, excluding
    /// packages in the excluded packages file.
    /// # Errors
    /// - The list command failed.
    /// - The excluded packages file could not be read.
    /// - The packages file could not be created or truncated.
    /// - The packages file could not be written to.
    //# INTEGRATION TESTED
    pub fn upload(&mut self) -> Result<&mut Self, String> {
        let installed_packages = self.package_manager.list()?;
        // get the list of excluded packages
        let Ok(excluded_packages) = utils::read_file_to_hashset(&self.excluded_packages_file) else {
            return Err(format!("Failed to read excluded packages file: {}", &self.excluded_packages_file.to_string_lossy()));
        };
        // create/truncate the packages file
        let mut packages_file = if let Ok(file) = File::create(&self.packages_file) {
            LineWriter::new(file)
        } else {
            return Err(format!("Failed to create or truncate packages file: {}", &self.packages_file.to_string_lossy()));
        };
        // exclude packages
        // write the updated package list to the file
        for package in installed_packages.difference(&excluded_packages) {
            if writeln!(packages_file, "{}", package).is_err() {
                return Err(format!("Failed to write to packages file: {}", &self.packages_file.to_string_lossy()));
            }
        }

        Ok(self)
    }

    /// Calls `install()` followed by `upload()`.
    //# HELPERS TESTED
    pub fn sync(&mut self) -> Result<&mut Self, String> {
        self.install()?;
        self.upload()?;
        return Ok(self);
    }

    /// Exclude packages from `upload()`. Warns the user if any packages that
    /// are about to be excluded are not installed.
    ///
    /// # Errors
    /// - Any errors from `PackageManager.list()`.
    pub fn exclude(&mut self) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// Reinclude packages that were previously excluded. Warns the user if any
    /// packages were not excluded or are not installed.
    ///
    /// # Errors
    /// - Any errors from `PackageManager.list()`.
    pub fn reinclude(&mut self) -> Result<&mut Self, String> {
        Ok(self)
    }

    /// Get the package system's name.
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Get the package manager.
    pub fn get_package_manager(&self) -> &PackageManager {
        return &self.package_manager;
    }
}

/// Create a new package system in `folder`.
///
/// # Parameters
/// - `package_systems_folder` - The full path to the folder to create the package system folder in.
/// - `name` - The name of the new package manager.
///
/// # Errors
/// - Cannot create the package system folder.
/// - Cannot create any package system files.
//# INTEGRATION TESTED
pub fn new_package_system(folder: impl AsRef<Path>, name: impl AsRef<str>) -> Result<(), String> {
    let name = name.as_ref();
    let folder = Path::new(folder.as_ref()).join(name);
    // check for an already-existing package system folder
    if folder.exists() {
        return Err(format!("Package system `{}` already exists", name));
    }
    // create the package system folder
    if fs::create_dir_all(&folder).is_err() {
        return Err(format!("Could not create package system folder `{}`", folder.to_string_lossy()));
    };
    // create the package files
    for file in [PACKAGES_FILENAME, EXCLUDED_PACKAGES_FILENAME] {
        let file = folder.join(file);
        if File::create_new(&file).is_err() {
            return Err(format!("Could not create package file `{}`. Maybe it already exists?", &file.to_string_lossy()));
        }
    }
    // create the package manager file
    let package_manager_path = folder.join(PACKAGE_MANAGER_FILENAME);
    let Ok(package_manager_file) = File::create_new(&package_manager_path) else {
        return Err(format!("Could not create package manager file `{}`. Maybe it already exists?", &package_manager_path.to_string_lossy()));
    };
    // write to it
    if write!(&package_manager_file, include_str!("../../templates/package-manager.toml")).is_err() {
        println!("Failed to write template for package manager file `{}`.", &package_manager_path.to_string_lossy())
    }

    println!("Created package system `{}`", name);
    Ok(())
}
