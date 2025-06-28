use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::process::Command;

use toml;

pub mod toml_structs {
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
        pub name: String,
        pub install_command: CommandProxy,
        pub list_command: CommandProxy,
    }
}
use toml_structs::*;

/// Represents a system's package manager with methods to list the installed packages and install
/// new ones.
pub struct PackageManager {
    pub name: String,
    pub install_command: Command,
    pub list_command: Command,
}
impl PackageManager {
    /// Build a `PackageManager`.
    ///
    /// # Parameters
    /// - `name` - The package manager's name (and associated folder name).
    /// - `install_command` - The command used to install packages.
    /// - `list_command` - The command used to list installed packages.
    pub fn build(name: impl ToString, install_command: Command, list_command: Command) -> Self {
        PackageManager {
            name: name.to_string(),
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
    pub fn from_toml(path: impl AsRef<Path>) -> Result<Self, String> {
        // try to read the contents of the file
        let Ok(contents) = fs::read_to_string(&path) else {
            return Err(format!(
                "Failed to read package manager file at {}",
                &path.as_ref().to_string_lossy()
            ));
        };
        // try to convert to a `PackageManagerProxy`
        let Ok(proxy) = toml::from_str::<PackageManagerProxy>(&contents) else {
            return Err(format!(
                "Invalid package manager file: {}",
                &path.as_ref().to_string_lossy()
            ));
        };
        Ok(Self::from(proxy))
    }

    /// Attempt to install the provided `packages`.
    ///
    /// # Errors
    /// - The install command fails to run.
    /// - The install command runs, but returns an error code.
    pub fn install(
        &mut self,
        packages: impl IntoIterator<Item = impl AsRef<OsStr>>,
    ) -> Result<&mut Self, String> {
        self.install_command.args(packages);
        if let Ok(status) = self.install_command.status() {
            if !status.success() {
                return Err(format!("Install command failed for {}", &self.name));
            }
        } else {
            return Err(format!("Could not run install command for {}", &self.name));
        }
        Ok(self)
    }

    /// Attempt to list the package manager's installed packages. The list command must return the
    /// installed packages separated by whitespace.
    ///
    /// # Errors
    /// - The list command fails.
    /// - The list command returns an invalid package format (i.e. not whitespace-separated).
    pub fn list(&mut self) -> Result<Vec<String>, String> {
        // run the list command and capture the output
        let Ok(output) = self.list_command.output() else {
            return Err(format!("Failed to list packages for {}", &self.name));
        };
        // convert the output to a String
        let Ok(output) = String::from_utf8(output.stdout) else {
            return Err(format!("{} list command returned invalid format", &self.name));
        };
        // convert the output to a list of Strings
        Ok(output.split_whitespace().map(str::to_string).collect())
    }

    /// Check if `packages` are installed.
    ///
    /// # Returns
    /// Whether all the packages in `packages` are installed.
    ///
    /// # Errors
    /// - Any errors from `list()`.
    pub fn check_for_packages<'a>(
        &mut self,
        packages: impl IntoIterator<Item = impl AsRef<str>>,
    ) -> Result<bool, String> {
        let installed_packages = match self.list() {
            Ok(packages) => packages,
            Err(error_message) => return Err(error_message),
        };
        let installed_packages = installed_packages
            .iter()
            .map(String::as_str)
            .collect::<Vec<&str>>();

        for package in packages {
            let package = package.as_ref();
            if !installed_packages.contains(&package) {
                return Ok(false);
            }
        }
        Ok(true)
    }
}
impl From<PackageManagerProxy> for PackageManager {
    fn from(proxy: PackageManagerProxy) -> Self {
        let mut package_manager = PackageManager {
            name: proxy.name,
            install_command: Command::new(proxy.install_command.command),
            list_command: Command::new(proxy.list_command.command),
        };
        package_manager
            .install_command
            .args(proxy.install_command.args);
        package_manager.list_command.args(proxy.list_command.args);
        package_manager
    }
}
