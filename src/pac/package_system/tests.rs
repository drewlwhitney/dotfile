use std::fs;
use std::process::Command;

use super::*;

const TEST_PACKAGE: &str = "trash-cli";
const TEST_PACKAGE2: &str = "nano";
const TEST_PACKAGES: [&str; 2] = [TEST_PACKAGE, TEST_PACKAGE2];

const PACKAGE_SYSTEM_FOLDER: &str = "./test_files/pac/package_system/install_upload";

#[cfg(test)]
mod install {
    use super::*;

    #[test]
    #[ignore = "Requires a password"]
    fn works() {
        let mut package_system = PackageSystem::from_folder(PACKAGE_SYSTEM_FOLDER).unwrap();
        // test that installation works
        package_system.install().unwrap();
        // check that pacman now lists trash-cli as installed
        if !String::from_utf8(Command::new("pacman").arg("-Qqen").output().unwrap().stdout)
            .unwrap()
            .lines()
            .collect::<Vec<&str>>()
            .contains(&TEST_PACKAGE)
        {
            panic!("installed packages for pacman did not contain {}", TEST_PACKAGE);
        }
    }
}

#[cfg(test)]
mod upload {
    use super::*;

    #[test]
    fn works() {
        // set up the package system
        let mut package_system = PackageSystem::from_folder(PACKAGE_SYSTEM_FOLDER).unwrap();
        // check that the test packages are installed (can't test otherwise)
        if !package_system
            .package_manager
            .check_for_packages(TEST_PACKAGES)
            .unwrap()
        {
            panic!("Cannot test, packages {:?} were not installed", &TEST_PACKAGES)
        }
        // upload
        package_system.upload().unwrap();
        // read the package file
        let contents = fs::read_to_string(package_system.packages_file).unwrap();
        let contents: Vec<&str> = contents.lines().collect();
        // check that the file contains the test package
        assert!(contents.contains(&TEST_PACKAGE), "{} was not uploaded", TEST_PACKAGE);
        // check that the exclusion functionality works
        assert!(
            !contents.contains(&TEST_PACKAGE2),
            "{} was uploaded but should have been excluded",
            TEST_PACKAGE2
        );
    }
}
