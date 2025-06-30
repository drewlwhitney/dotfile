//! Integration tests for the `pac` module.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use dotfile::pac::*;
use lazy_static::lazy_static;
use rstest::*;
use test_utils::{self, PathRemover};
use utils;

lazy_static! {
    static ref PAC_TEST_FILES_FOLDER: PathBuf = PathBuf::from("test_files/pac");
}

/// Tests the `package_manager` module.
#[cfg(test)]
mod package_manager_tests {
    use super::*;

    const TEST_PACKAGE1: &str = "nano";
    const TEST_PACKAGE2: &str = "trash-cli";

    /// Tests `install()`.
    #[cfg(test)]
    mod install {
        use super::*;

        /// Test the installation of a single package.
        #[rstest]
        #[ignore = "must be run single-threaded"]
        fn single(package_manager: PackageManager) {
            test_install(package_manager, &[TEST_PACKAGE1]);
        }

        /// Test the installation of multiple packages.
        #[rstest]
        #[ignore = "must be run single-threaded"]
        fn multiple(package_manager: PackageManager) {
            test_install(package_manager, &[TEST_PACKAGE1, TEST_PACKAGE2]);
        }

        /// Helper function for testing `install()`.
        fn test_install(mut package_manager: PackageManager, packages: &[&str]) {
            let _package_handler = PackageAutoHandler::new(packages); // packages uninstalled here
            package_manager.install(packages).unwrap();
            assert!(test_utils::check_installed(packages));
        } // packages reinstalled here

        /// Automatically removes the provided `packages` on construction and
        /// reinstalls them when dropped.
        struct PackageAutoHandler<'a> {
            packages: &'a [&'a str],
        }
        impl<'a> PackageAutoHandler<'a> {
            /// Create a new instance. Uninstalls all `packages`.
            pub fn new(packages: &'a [&'a str]) -> Self {
                test_utils::remove_packages(packages);
                Self { packages }
            }
        }
        impl<'a> Drop for PackageAutoHandler<'a> {
            /// Reinstall the `packages`.
            fn drop(&mut self) {
                test_utils::install_packages(self.packages);
            }
        }
    }

    /// Tests `list()`.
    #[cfg(test)]
    mod list {
        use super::*;

        /// Make sure the list command does what it should.
        #[rstest]
        fn it_works(mut package_manager: PackageManager) {
            assert_eq!(package_manager.list().unwrap(), test_utils::list_installed());
        }
    }

    /// Creates a valid package manager.
    #[fixture]
    fn package_manager() -> PackageManager {
        let mut install_command = Command::new("sudo");
        install_command.args(["pacman", "-S", "--noconfirm"]);
        let mut list_command = Command::new("pacman");
        list_command.arg("-Qq");
        PackageManager::build(install_command, list_command)
    }
}

/// Tests the package_system module.
#[cfg(test)]
mod package_system_tests {
    use super::*;

    lazy_static! {
        static ref TEST_FILES_FOLDER: PathBuf = PAC_TEST_FILES_FOLDER.join("package_system");
    }

    /// Tests `PackageSystem.upload()`.
    #[test]
    #[ignore = "must be run single-threaded"]
    fn upload() {
        let excluded_packages = ["nano", "trash-cli"];
        // make sure the test excluded packages are installed; can't test otherwise
        assert!(test_utils::check_installed(&excluded_packages), "Test excluded packages not installed: {:?}", &excluded_packages);
        // setup
        let folder = TEST_FILES_FOLDER.join("upload");
        let installed_packages_file = folder.join("installed_packages.txt");
        let _file_remover = PathRemover::new(&installed_packages_file);
        let mut package_system = PackageSystem::from_folder(folder).unwrap();
        // upload
        package_system.upload().unwrap();
        // get the packages the package system uploaded
        let uploaded_packages = utils::read_file_to_vector(&installed_packages_file).unwrap();
        // check that all uploaded packages are actually installed
        assert!(utils::contains_all(&test_utils::list_installed(), &uploaded_packages));
        // check that the excluded packages were not uploaded
        assert!(utils::contains_none(&uploaded_packages.iter().map(String::as_str).collect::<Vec<_>>(), &excluded_packages));
    }

    /// Tests `new_package_system()`.
    #[test]
    #[ignore = "must be run single-threaded"]
    fn test_new_package_system() {
        // keep track of the folder where the package system gets created
        let temp_folder = TEST_FILES_FOLDER.join("TEMP");
        let _path_remover = PathRemover::new(&temp_folder);
        // create a new package manager
        let name = "test_package_system";
        let package_system_folder = temp_folder.join(name);
        new_package_system(&temp_folder, name).unwrap(); // run the command
        // make sure the package manager contents are correct
        let correct_package_manager_contents = include_str!("../templates/package_manager.toml");
        assert_eq!(
            fs::read_to_string(package_system_folder.join(package_system::PACKAGE_MANAGER_FILENAME)).unwrap(),
            correct_package_manager_contents
        );
        // ensure the installed packages file and excluded packages file were created
        assert!(Path::exists(&package_system_folder.join(package_system::PACKAGES_FILENAME)));
        assert!(Path::exists(&package_system_folder.join(package_system::EXCLUDED_PACKAGES_FILENAME)));
    }
}

/// Tests the `parser` module.
#[cfg(test)]
mod parser_tests {
    use super::*;

    /// Tests `package_system_from_folder()`.
    #[test]
    fn test_package_systems_from_folder() {
        let package_systems_folder = PAC_TEST_FILES_FOLDER.join("parser/test_package_systems");
        // load the package systems
        let package_managers = package_systems_from_folder(package_systems_folder).unwrap();

        // verify pacman is correct
        let pacman = &package_managers.get("pacman").unwrap();
        assert_eq!(pacman.get_name(), "pacman");
        let pacman_manager = pacman.get_package_manager();
        assert_eq!(pacman_manager.install_command.get_program().to_str().unwrap(), "sudo");
        assert_eq!(pacman_manager.install_command.get_args().collect::<Vec<_>>(), vec!["pacman", "-S", "--needed", "--noconfirm"]);
        assert_eq!(pacman_manager.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(pacman_manager.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqen"]);

        // verify yay is correct
        let yay = &package_managers.get("yay").unwrap();
        assert_eq!(yay.get_name(), "yay");
        let yay_manager = yay.get_package_manager();
        assert_eq!(yay_manager.install_command.get_program().to_str().unwrap(), "yay");
        assert_eq!(yay_manager.install_command.get_args().collect::<Vec<_>>(), vec!["-S", "--needed", "--noconfirm"]);
        assert_eq!(yay_manager.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(yay_manager.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqem"]);
    }
}
