use clap::{Args, Parser, Subcommand};

use super::*;

/// `pac`-level subcommands.
#[derive(Subcommand)]
pub enum PacCommand {
    /// Install packages from a package list
    #[command(alias = "tt")]
    Install(PackageManagerArgs),
    /// Save the package manager state
    Upload(PackageManagerArgs),
    /// `install`, then `upload`
    Sync(PackageManagerArgs),
}

/// Package manager name.
#[derive(Args, Debug)]
pub struct PackageManagerArgs {
    /// The name of the package manager to use. If not specified, the default is used
    #[arg(long = "name", short = 'n')]
    package_manager_name: Option<String>,
}

pub fn process_command(command: &PacCommand) -> Result<(), String> {
    match command {
        PacCommand::Install(args) | PacCommand::Upload(args) | PacCommand::Sync(args) => println!("{:?}", args),
    }
    Ok(())
}
