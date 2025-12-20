//! Utilities for interacting with the system's package manager during testing.
//! This assumes you are testing on ArchLinux using pacman.

use std::collections::HashSet;
use std::process::Command;

use super::contains;

/// Get a list of installed packages.
pub fn list_installed() -> HashSet<String> {
    String::from_utf8(Command::new("pacman").arg("-Qq").output().unwrap().stdout)
        .unwrap()
        .lines()
        .map(String::from)
        .collect()
}

/// Checks that all packages in `packages` are installed.
pub fn check_installed(packages: &[&str]) -> bool {
    let installed_packages = list_installed();
    contains::contains_all(&installed_packages.iter().map(String::as_str).collect::<Vec<_>>(), packages)
}

/// Check that all packages in `packages` are *not* installed.
pub fn check_not_installed(packages: &[&str]) -> bool {
    let installed_packages = list_installed();
    contains::contains_none(&installed_packages.iter().map(String::as_str).collect::<Vec<_>>(), packages)
}

/// Install `packages`.
pub fn install_packages(packages: &[&str]) {
    Command::new("sudo")
        .args(["pacman", "-S", "--noconfirm"])
        .args(packages)
        .status()
        .unwrap();
    assert!(check_installed(packages));
}

/// Uninstalled `packages`.
pub fn remove_packages(packages: &[&str]) {
    Command::new("sudo")
        .args(["pacman", "-Rs", "--noconfirm"])
        .args(packages)
        .status()
        .unwrap();
    assert!(check_not_installed(packages));
}
