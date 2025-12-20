use clap::{Args, Subcommand};

/// `pac`-level subcommands.
#[derive(Subcommand)]
pub enum PacCommand {
    /// Install packages from a package list
    #[command(visible_alias = "in")]
    Install(PackageManagerArgs),
    /// Save the package manager state
    #[command(visible_alias = "up")]
    Upload(PackageManagerArgs),
    /// `install`, then `upload`
    Sync(PackageManagerArgs),
    /// Exclude packages from syncing
    #[command(visible_alias = "ex")]
    Exclude {
        /// Packages to exclude
        packages: Vec<String>,
    },
    /// Reinclude previously excluded packages
    #[command(visible_alias = "re")]
    Reinclude {
        /// Packages to reinclude
        #[arg(required = true)]
        packages: Vec<String>,
    },
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
        PacCommand::Exclude { packages } | PacCommand::Reinclude { packages } => println!("{:?}", packages),
    }
    Ok(())
}
