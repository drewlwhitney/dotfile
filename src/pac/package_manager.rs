#[cfg(test)]
mod tests;

use std::collections::HashSet;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

use toml;

mod toml_structs {
    //! Structs used to parse **.toml** files with the `toml` crate.
    use serde_derive::Deserialize;
    /// A proxy for a `Command`.
    #[derive(Deserialize)]
    pub struct CommandProxy {
        pub command: String,
        pub args: Vec<String>,
    }

    /// A proxy for a `PackageManager`.
    #[derive(Deserialize)]
    pub struct PackageManagerProxy {
        pub install_command: CommandProxy,
        pub list_command: CommandProxy,
    }
}
use toml_structs::*;

/// Represents a system's package manager with methods to list the installed
/// packages and install new ones.
pub struct PackageManager {
    pub install_command: Command,
    pub list_command: Command,
}
impl PackageManager {
    /// Build a `PackageManager`.
    ///
    /// # Parameters
    /// - `install_command` - The command used to install packages.
    /// - `list_command` - The command used to list installed packages.
    pub fn build(install_command: Command, list_command: Command) -> Self {
        PackageManager {
            install_command,
            list_command,
        }
    }

    /// Build from a TOML file.
    ///
    /// # Errors
    /// - The file cannot be read from.
    /// - The file's format is invalid.
    ///
    /// # File Format
    /// - `name` - the name of the package manager.
    /// - A table called `install_command` with parameters:
    ///     - `command` - The command to run.
    ///     - `args` - An array of arguments to pass to the command.
    /// - A table called `list_command` with the same parameters as `install_command`.
    ///
    /// ## Format Example
    /// <pre>
    /// name = "pacman"
    ///
    /// [install_command]
    /// command = "sudo"
    /// args = ["pacman", "-S", "--needed"]
    ///
    /// [list_command]
    /// command = "pacman"
    /// args = ["-Qqen"]
    /// </pre>
    //# HELPERS UNIT TESTED
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, String> {
        // try to read the contents of the file
        let Ok(contents) = fs::read_to_string(&path) else {
            return Err(format!("Failed to read package manager file at {}", &path.as_ref().to_string_lossy()));
        };
        Self::from_toml_string(contents)
    }

    /// Helper function to build from a TOML string.
    //# UNIT TESTED
    fn from_toml_string(toml_string: impl AsRef<str>) -> Result<Self, String> {
        let toml_string = toml_string.as_ref();
        // try to convert to a `PackageManagerProxy`
        let Ok(proxy) = toml::from_str::<PackageManagerProxy>(&toml_string) else {
            // return the invalid file contents on failure
            return Err(format!("Invalid package manager file contents:\n{}", toml_string.to_string()));
        };
        Ok(proxy.into())
    }

    /// Attempt to install the provided `packages`.
    ///
    /// # Errors
    /// - The install command fails to run.
    /// - The install command runs, but returns an error code.
    //# INTEGRATION TESTED
    pub fn install(&mut self, packages: impl IntoIterator<Item = impl AsRef<OsStr>>) -> Result<&mut Self, String> {
        self.install_command.args(packages);
        if let Ok(status) = self.install_command.status() {
            if !status.success() {
                return Err("Install command failed".to_string());
            }
        } else {
            return Err("Could not run install command".to_string());
        }
        Ok(self)
    }

    /// Attempt to list the package manager's installed packages. The list
    /// command must return the installed packages separated by whitespace.
    ///
    /// # Errors
    /// - The list command fails.
    /// - The list command returns an invalid package format (i.e. not whitespace-separated).
    //# INTEGRATION TESTED
    pub fn list(&mut self) -> Result<HashSet<String>, String> {
        // run the list command and capture the output
        let Ok(output) = self.list_command.output() else {
            return Err("Failed to list packages".to_string());
        };
        // convert the output to a String
        let Ok(output) = String::from_utf8(output.stdout) else {
            return Err("List command returned invalid format".to_string());
        };
        // convert the output to a list of Strings
        Ok(output.split_whitespace().map(String::from).collect())
    }
}

impl From<PackageManagerProxy> for PackageManager {
    fn from(proxy: PackageManagerProxy) -> Self {
        let mut package_manager = Self::build(Command::new(proxy.install_command.command), Command::new(proxy.list_command.command));
        package_manager.install_command.args(proxy.install_command.args);
        package_manager.list_command.args(proxy.list_command.args);
        package_manager
    }
}

#[cfg(test)]
impl PartialEq for PackageManager {
    fn eq(&self, other: &Self) -> bool {
        // equal if the commands are equal
        self.install_command.get_args().collect::<Vec<_>>() == other.install_command.get_args().collect::<Vec<_>>()
            && self.list_command.get_args().collect::<Vec<_>>() == other.list_command.get_args().collect::<Vec<_>>()
            && self.install_command.get_program() == other.install_command.get_program()
            && self.list_command.get_program() == other.install_command.get_program()
    }
}
