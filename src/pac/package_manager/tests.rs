use super::*;
use crate::*;
use std::fs::File;
use std::io::prelude::*;

const TEST_PACKAGE: &str = "trash-cli";
const TEST_PACKAGE2: &str = "nano";

#[cfg(test)]
mod install {
    use super::*;

    #[test]
    fn works() {
        // set up install command
        let mut install_command = Command::new("sudo");
        install_command.args(["pacman", "-S", "--noconfirm"]);
        // set up package system
        let mut package_system = PackageSystem::build(
            PackageManager::build("pacman", install_command, Command::new("")),
            "./test_files/pac/package_manager",
        );
        // test that installation works
        package_system.install().unwrap();
        // check that pacman now lists trash-cli as installed
        if !String::from_utf8(Command::new("pacman").arg("-Qqen").output().unwrap().stdout)
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .contains(&TEST_PACKAGE)
        {
            panic!("pacman installed packages did not contain {}", TEST_PACKAGE);
        }
    }
}

#[cfg(test)]
mod upload {
    use super::*;

    #[test]
    fn works() {
        // set up list command
        let mut list_command = Command::new("pacman");
        list_command.arg("-Qqen");
        // set up the package manager
        let mut package_manager = PackageManager::build("", Command::new(""), list_command);
        // check that the test packages are installed (can't test otherwise)
        let non_installed_packages = package_manager
            .check_for_packages(&vec![TEST_PACKAGE, TEST_PACKAGE2])
            .unwrap();
        if non_installed_packages.len() > 0 {
            panic!(
                "Cannot test, packages {:?} were not installed",
                non_installed_packages
            )
        }
        // set up the package system
        let mut package_system =
            PackageSystem::build(package_manager, "./test_files/pac/package_manager/upload");
        // upload
        package_system.upload().unwrap();
        // read the package file
        let contents = fs::read_to_string(package_system.packages_file).unwrap();
        let contents: Vec<&str> = contents.lines().collect();
        // check that the file contains the test package
        if !contents.contains(&TEST_PACKAGE) {
            panic!("{} was not uploaded", TEST_PACKAGE);
        }
        // check that the exclusion functionality works
        if contents.contains(&TEST_PACKAGE2) {
            panic!(
                "{} was uploaded but should have been excluded",
                TEST_PACKAGE2
            );
        }
    }
}

#[cfg(test)]
mod add_package_manager {
    use super::*;

    #[test]
    fn works() {
        let first_entry_text = include_str!(
            "../../../test_files/pac/package_manager/add_package_manager/first_entry.toml"
        );
        let correct_text = include_str!(
            "../../../test_files/pac/package_manager/add_package_manager/correct_file.toml"
        );
        let package_manager_file =
            "./test_files/pac/package_manager/add_package_manager/package_managers.toml";
        let mut file = File::create(package_manager_file).unwrap();
        write!(file, "{}", first_entry_text).unwrap();
        PackageManager::add_package_manager(package_manager_file, "test2").unwrap();
        assert_eq!(
            correct_text,
            fs::read_to_string(package_manager_file).unwrap()
        )
    }
}
