//! Parser for the package manager configuration file.
use std::collections::HashMap;
use std::fs;

use super::package_system::*;

/// Read a list of `PackageSystem`s from subfolders containing **TOML** files in `folder`.
///
/// # Errors
/// - Failed to list subfolders in `folder`.
/// - Failed to create the package manager because of an invalid file format.
pub fn package_systems_from_folder(folder: &str) -> Result<HashMap<String, PackageSystem>, String> {
    let mut package_systems: HashMap<String, PackageSystem> = HashMap::new();
    // get an iterator of paths in the folder
    let Ok(package_system_paths) = fs::read_dir(folder) else {
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
        // try to create the package system from the current folder
        match PackageSystem::from_folder(&package_system_path) {
            // success; insert into the mapping
            Ok(package_system) => {
                package_systems.insert(package_system.package_manager.name.clone(), package_system);
            }
            // failure; return the error message
            Err(message) => return Err(message),
        }
    }
    Ok(package_systems)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_package_managers() {
        // load from a test string
        let package_managers =
            package_systems_from_folder("test_files/pac/parser/package_managers").unwrap();

        // verify pacman is correct
        let pacman = &package_managers.get("pacman").unwrap().package_manager;
        assert_eq!(pacman.name, "pacman");
        assert_eq!(pacman.install_command.get_program().to_str().unwrap(), "sudo");
        assert_eq!(
            pacman.install_command.get_args().collect::<Vec<_>>(),
            vec!["pacman", "-S", "--needed", "--noconfirm"]
        );
        assert_eq!(pacman.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(pacman.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqen"]);

        // verify yay is correct
        let yay = &package_managers.get("yay").unwrap().package_manager;
        assert_eq!(yay.name, "yay");
        assert_eq!(yay.install_command.get_program().to_str().unwrap(), "yay");
        assert_eq!(yay.install_command.get_args().collect::<Vec<_>>(), vec!["-S", "--needed"]);
        assert_eq!(yay.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(yay.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqm"]);
    }
}
