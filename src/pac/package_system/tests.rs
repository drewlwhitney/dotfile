use std::process::Command;

use super::*;

/// Tests the constructors.
#[cfg(test)]
mod construction {
    use super::*;

    /// Tests `build()`.
    #[test]
    fn build() {
        let folder = Path::new("package_systems");
        let package_system = PackageSystem::build(String::from("pacman"), &folder, PackageManager::build(Command::new(""), Command::new("")));
        // make sure the packages file and excluded packages file have the correct paths
        assert_eq!(package_system.packages_file, folder.join(super::PACKAGES_FILENAME));
        assert_eq!(package_system.excluded_packages_file, folder.join(super::EXCLUDED_PACKAGES_FILENAME));
    }

    /// Tests `from_folder()`.
    #[test]
    fn from_folder() {
        let name = "test_package_system";
        let test_package_system_folder = Path::new(file!()).parent().unwrap().join(name);
        let package_system = PackageSystem::from_folder(test_package_system_folder).unwrap();
        // just need to check that the package system's name is based on the folder,
        // everything else is already tested
        assert_eq!(package_system.get_name(), name);
    }
}
