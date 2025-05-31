#[cfg(test)]
mod tests;

use super::parser::toml_structs::*;

use minijinja::{Environment, UndefinedBehavior, context};
use std::ffi::OsStr;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::process::Command;

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
    pub fn build(name: &str, install_command: Command, list_command: Command) -> PackageManager {
        PackageManager {
            name: name.to_string(),
            install_command,
            list_command,
        }
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
            return Err(format!(
                "{} list command returned invalid format",
                &self.name
            ));
        };
        // convert the output to a list of Strings
        Ok(output.split_whitespace().map(str::to_string).collect())
    }

    /// Check if `packages` are installed.
    ///
    /// # Returns
    /// A list of packages that were from `packages` that are not installed.
    ///
    /// # Errors
    /// - Any errors from `list()`.
    pub fn check_for_packages(&mut self, packages: &Vec<&str>) -> Result<Vec<String>, String> {
        let installed_packages = match self.list() {
            Ok(packages) => packages,
            Err(error_message) => return Err(error_message),
        };

        let mut non_installed_packages: Vec<String> = Vec::new();
        for package in packages {
            let package = String::from(*package);
            if !installed_packages.contains(&package) {
                non_installed_packages.push(package);
            }
        }
        Ok(non_installed_packages)
    }

    /// Add a package manager entry to the package manager file.
    ///
    /// # Parameters
    /// - `package_manager_file` - The full path to the package manger file.
    /// - `name` - The name of the package manager entry.
    ///
    /// # Errors
    /// - Cannot to open the package manager file.
    /// - Cannot parse template file, cannot load template, or cannot render template.
    /// - Cannot write to package manager file.
    pub fn add_package_manager(package_manager_file: &str, name: &str) -> Result<(), String> {
        // open the package manager file in append mode
        let Ok(mut file) = OpenOptions::new().append(true).open(package_manager_file) else {
            return Err(format!(
                "Could not open package manager file: {}",
                package_manager_file
            ));
        };
        // create a jinja environment
        let mut environment = Environment::new();
        environment.set_undefined_behavior(UndefinedBehavior::Strict); // undefined values = error
        if let Err(_) = environment.add_template(
            "template",
            include_str!("../../templates/package_manager.toml"),
        ) {
            return Err(String::from("Could not parse template file"));
        }
        let Ok(template) = environment.get_template("template") else {
            return Err(String::from(
                "Could not load template (error with `minijinja`)",
            ));
        };
        // render the template with the package manager name
        let Ok(entry_text) = template.render(context! {NAME => name}) else {
            return Err(String::from(
                "Could not render template (error in `minijinja`)",
            ));
        };
        // append the new entry to the package manager file
        if let Err(_) = write!(file, "\n\n{}\n", entry_text) {
            return Err(format!(
                "Failed to write to package manager file: {}",
                package_manager_file
            ));
        }

        Ok(())
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
