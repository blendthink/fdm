use crate::commands::{FdmCommand, InstallCommand, ReleasesCommand};
use std::process::ExitCode;

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(version, author, about)]
struct Args {
    #[clap(subcommand)]
    action: Action,
}

#[derive(Subcommand)]
enum Action {
    /// View all Dart SDK releases available for install.
    Releases(ReleasesCommand),

    /// Installs Dart SDK Version.
    Install(InstallCommand),
}

pub fn run() -> ExitCode {
    let args: Args = Args::parse();

    use Action::*;
    match args.action {
        Releases(cmd) => cmd.run(),
        Install(cmd) => cmd.run(),
    }
}
