use clap::{Parser, Subcommand};
use dotfile::pac;
use dotfile::pac::cli::*;

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
}

fn main() {
    let cli = Cli::parse();

    match &cli.subcommand {
        Command::Pac { subcommand } => pac::cli::process_command(subcommand).unwrap(),
    };
}
