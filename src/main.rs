use std::process::ExitCode;

mod cli;
mod commands;
mod models;
mod services;

fn main() -> ExitCode {
    cli::run()
}
