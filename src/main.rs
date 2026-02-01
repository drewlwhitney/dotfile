use clap::{Parser, Subcommand};
use dotfile::dir;
use dotfile::pac::cli::*;
use dotfile::pac::{self, *};

/// Main CLI parser.
#[derive(Parser)]
#[command(version, about, long_about = None, propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    subcommand: Command,
}

/// Primary-level subcommands.
#[derive(Subcommand)]
enum Command {
    /// Manage system packages
    Pac {
        #[command(subcommand)]
        subcommand: PacCommand,
    },
    /// Print the location of the `dotfiles` directory
    Dir,
}

fn main() {
    let cli = Cli::parse();

    if let Err(error_message) = match &cli.subcommand {
        Command::Pac { subcommand } => pac::cli::process_command(subcommand),
        Command::Dir => Ok(dir::print_dir()),
    } {
        println!("{}", error_message);
    }
}
