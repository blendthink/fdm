use crate::commands;
use crate::commands::FdmCommand;
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
    Releases(commands::Releases),
}

pub fn run() -> ExitCode {
    let args: Args = Args::parse();

    use Action::*;
    match args.action {
        Releases(cmd) => cmd.run(),
    }
}
