use std::process::ExitCode;

mod cli;
mod commands;

fn main() -> ExitCode {
    cli::run()
}
