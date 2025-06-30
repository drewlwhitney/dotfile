//! Parser for the package manager configuration file.
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use super::package_system::*;

/// Read a list of `PackageSystem`s from subfolders containing **TOML** files in
/// `folder`.
///
/// # Errors
/// - Failed to list subfolders in `folder`.
/// - Failed to create the package manager because of an invalid file format.
//# INTEGRATION TESTED
pub fn package_systems_from_folder(folder: impl AsRef<Path>) -> Result<HashMap<String, PackageSystem>, String> {
    let mut package_systems: HashMap<String, PackageSystem> = HashMap::new();
    // get an iterator of paths in the folder
    let Ok(package_system_paths) = fs::read_dir(folder.as_ref()) else {
        return Err("Invalid package systems folder".to_string());
    };
    for package_system_path in package_system_paths {
        // get the actual path
        let package_system_path = if let Ok(temp) = package_system_path {
            temp.path()
        } else {
            // this error probably never runs
            return Err("Invalid package system folder detected".to_string());
        };
        // only operate on directories
        if package_system_path.is_dir() {
            // try to create a package system from the current folder
            let package_system = PackageSystem::from_folder(&package_system_path)?;
            package_systems.insert(package_system.get_name().to_owned(), package_system);
        }
    }
    Ok(package_systems)
}
