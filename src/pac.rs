#[cfg(test)]
mod tests;

use std::fs;
use std::fs::File;
use std::io::{LineWriter, prelude::*};
use std::path::PathBuf;
use std::process::Command;

pub struct PackageSystem {
    pub packages_file: PathBuf,
    pub excluded_packages_file: PathBuf,
    pub list_command: Command,
    pub install_command: Command,
}
impl PackageSystem {
    fn new(
        package_file: &str,
        excluded_packages_file: &str,
        list_command: Command,
        install_command: Command,
    ) -> Self {
        PackageSystem {
            packages_file: PathBuf::from(package_file),
            excluded_packages_file: PathBuf::from(excluded_packages_file),
            list_command,
            install_command,
        }
    }
}

/// Downloads packages from a file.
/// # Parameters
/// `package_system` The **PackageSystem** for the current package manager.
pub fn download(package_system: &mut PackageSystem) {
    // get the list of packages to install
    let packages =
        fs::read_to_string(&package_system.packages_file).expect("Failed to read packages file.");
    let packages: Vec<&str> = packages.lines().collect();

    // run the install command
    package_system
        .install_command
        .args(packages)
        .status()
        .expect("Install command failed.");
}

/// Gets a list of installed packages and saves the names to a file.
/// # Parameters
/// `package_system` The **PackageSystem** for the current package manager.
pub fn upload(package_system: &mut PackageSystem) {
    // get a list of installed packages from the system
    let packages = String::from_utf8(
        package_system
            .list_command
            .output()
            .expect("List command failed.")
            .stdout,
    )
    .expect("Failed to convert to string.");
    let packages: Vec<&str> = packages.lines().collect();

    // get a list of excluded packages
    let excluded_packages = fs::read_to_string(&package_system.excluded_packages_file)
        .expect("Failed to open excluded packages file.");

    // write to the packages file
    let packages_file =
        File::create(&package_system.packages_file).expect("Failed to create packages file.");
    let mut packages_file = LineWriter::new(packages_file);
    for package in packages {
        if !excluded_packages.contains(&package) {
            writeln!(packages_file, "{}", package).expect("Failed to write to packages file.");
        }
    }
}

/// Runs `download()` followed by `upload()`.
/// # Parameters
/// `package_system` The **PackageSystem** for the current package manager.
pub fn sync(package_system: &mut PackageSystem) {
    download(package_system);
    upload(package_system);
} // does not need a test because it calls tested functions

// exclude(excluded_packages_file)
pub fn exclude(excluded_packages_file: &PathBuf, packages: &Vec<&str>) {}

// reinclude(excluded_packages_file)
pub fn reinclude(excluded_packages_file: &PathBuf, packages: &Vec<&str>) {}

// TODO: integrate the other buffer

// winget_download

// winget_upload
