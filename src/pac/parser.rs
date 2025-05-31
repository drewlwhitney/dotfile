//! Parser for the package manager configuration file.
use std::fs;

use itertools::Itertools;
use serde_derive::Deserialize;
use toml;

use super::package_manager::*;

pub mod toml_structs {
    //! Structs used to parse **.toml** files with the `toml` crate.
    use super::*;
    /// A proxy for a `Command`.
    #[derive(Deserialize)]
    pub struct CommandProxy {
        pub command: String,
        pub args: Vec<String>,
    }

    /// A proxy for a `PackageManager`.
    #[derive(Deserialize)]
    pub struct PackageManagerProxy {
        pub name: String,
        pub install_command: CommandProxy,
        pub list_command: CommandProxy,
    }

    /// A list of `PackageManagerProxy`s.
    #[derive(Deserialize)]
    pub struct PackageManagerProxyList {
        pub package_managers: Vec<PackageManagerProxy>,
    }
}
use toml_structs::*;

/// Read a list of package managers from a properly formatted **.toml** file.
///
/// # Errors
/// - The file cannot be read.
/// - The file format is invalid (see below).
/// - There are any duplicate package manager names.
///
/// # File Format
/// The file should contain an array of tables.
/// - The table header is **package_managers**. Each package manager needs:
///     - `name` - The name of the package manager.
///     - A subtable called **install_command** with parameters:
///         - `command` - The command to run.
///         - `args` - An array of arguments to pass to the command.
///     - A subbtable called **list_command** with the same parameters as the `install_command`
///       header.
/// ## Format Example
/// <pre>
/// # pacman
/// [[package_managers]]
/// name = "pacman"
///
/// [package_managers.install_command]
/// command = "sudo"
/// args = ["pacman", "-S", "--needed"]
///
/// [package_managers.list_command]
/// command = "pacman"
/// args = ["-Qqen"]
///
/// # yay
/// [[package_managers]]
/// name = "yay"
///
/// [package_managers.install_command]
/// command = "yay"
/// args = ["-S", "--needed"]
///
/// [package_managers.list_command]
/// command = "pacman"
/// args = ["-Qqm"]
/// </pre>
pub fn package_managers_from_toml(file: &str) -> Result<Vec<PackageManager>, String> {
    // read the file
    let Ok(file_contents) = fs::read_to_string(file) else {
        return Err(format!("Failed to read from {file}"));
    };
    // convert the file contents to a PackageManagerProxyList
    let Ok(package_managers) = toml::from_str::<PackageManagerProxyList>(&file_contents) else {
        return Err(format!("Invalid package manager configuration file: {file}"));
    };
    let package_managers = package_managers.package_managers;
    // check for duplicates
    // this isn't as fast as it could be, but it's shorter and I'm lazy >:)
    let duplicate_count = package_managers
        .iter()
        .duplicates_by(|package_manager| &package_manager.name)
        .count();
    if duplicate_count > 0 {
        return Err(format!("Duplicate package manager name detected in {file}"));
    }

    Ok(package_managers
        .into_iter()
        .map(PackageManager::from) // convert each proxy to an actual PackageManager
        .collect()) // collect as vector and return
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_package_manager_file() {
        // load from a test string
        let package_managers =
            package_managers_from_toml("test_files/pac/parser/basic.toml").unwrap();

        // verify pacman is correct
        let pacman = &package_managers[0];
        assert_eq!(pacman.name, "pacman");
        assert_eq!(pacman.install_command.get_program().to_str().unwrap(), "sudo");
        assert_eq!(
            pacman.install_command.get_args().collect::<Vec<_>>(),
            vec!["pacman", "-S", "--needed", "--noconfirm"]
        );
        assert_eq!(pacman.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(pacman.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqen"]);

        // verify yay is correct
        let yay = &package_managers[1];
        assert_eq!(yay.name, "yay");
        assert_eq!(yay.install_command.get_program().to_str().unwrap(), "yay");
        assert_eq!(
            yay.install_command.get_args().collect::<Vec<_>>(),
            vec!["-S", "--needed"]
        );
        assert_eq!(yay.list_command.get_program().to_str().unwrap(), "pacman");
        assert_eq!(yay.list_command.get_args().collect::<Vec<_>>(), vec!["-Qqm"]);
    }

    #[test]
    #[should_panic]
    fn check_for_duplicates() {
        package_managers_from_toml("test_files/pac/parser/duplicate.toml").unwrap();
    }
}
