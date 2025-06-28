use std::fs::{self, File};
use std::io::prelude::*;
use std::path::Path;

use minijinja::{Environment, UndefinedBehavior, context};

#[cfg(test)]
mod tests;

mod package_manager;
mod package_system;
pub mod parser;
pub use package_system::PackageSystem;
pub use parser::*;

/// Create a new package system in `folder`.
///
/// # Parameters
/// - `package_systems_folder` - The full path to the package systems folder.
/// - `name` - The name of the new package manager.
///
/// # Errors
/// - Cannot create the package system folder.
/// - Cannot create any package system files.
/// - The `minijinja` template failed to render (should not happen and is a bug if it does).
/// - Cannot write to package manager file.
pub fn create_package_system(folder: impl AsRef<Path>, name: &str) -> Result<(), String> {
    let folder = Path::new(folder.as_ref()).join(name);
    // create the package system folder
    if let Err(_) = fs::create_dir_all(&folder) {
        return Err(format!(
            "Could not create package system folder: {}",
            folder.to_string_lossy()
        ));
    };
    // create the package files
    for file in [
        package_system::PACKAGES_FILENAME,
        package_system::EXCLUDED_PACKAGES_FILENAME,
    ] {
        let file = folder.join(file);
        if let Err(_) = File::create_new(&file) {
            return Err(format!(
                "Could not create file: {}. Maybe it already exists?",
                &file.to_string_lossy()
            ));
        }
    }
    // create and write to the package manager file
    let package_manager_path = folder.join(package_system::PACKAGE_MANAGER_FILENAME);
    let Ok(mut package_manager_file) = File::create_new(&package_manager_path) else {
        return Err(format!("Could not create package system: {}", name));
    };
    // create a jinja environment
    let mut environment = Environment::new();
    environment.set_undefined_behavior(UndefinedBehavior::Strict); // undefined values = error
    // add and render the template file
    let template_error = "BUG: invalid template file".to_string();
    if let Err(_) =
        environment.add_template("template", include_str!("../templates/package_manager.toml"))
    {
        return Err(template_error);
    }
    let Ok(template) = environment.get_template("template") else {
        return Err(template_error);
    };
    // render the template with the package manager name
    let Ok(text) = template.render(context! {NAME => name}) else {
        return Err(template_error);
    };
    // write the rendered text to the package manager file
    if let Err(_) = write!(package_manager_file, "{}\n", text) {
        return Err(format!(
            "Failed to write to package manager file: {}",
            &package_manager_path.to_string_lossy()
        ));
    }

    Ok(())
}
