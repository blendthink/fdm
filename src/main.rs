use std::process::ExitCode;

mod cli;
mod commands;
mod services;

fn main() -> ExitCode {
    cli::run()
}
