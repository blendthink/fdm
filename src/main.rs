use std::process::ExitCode;

mod cli;
mod commands;
mod extensions;
mod models;
mod services;

fn main() -> ExitCode {
    cli::run()
}
