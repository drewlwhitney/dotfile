pub mod pac;
use pac::*;
use std::fs;

fn main() {
    // let package_managers = parser::package_managers_from_toml("pac.toml").unwrap();
    // let package_manager = package_managers.into_iter().nth(0).unwrap();
    // let name = package_manager.name.to_owned();
    // let mut package_system = PackageSystem::build(package_manager, format!("./{}",
    // name).as_str()); package_system.upload().unwrap();
    package_manager::PackageManager::add_package_manager(
        "./pac.toml",
        "shitty ass package manager",
    )
    .unwrap();

    ()
}
