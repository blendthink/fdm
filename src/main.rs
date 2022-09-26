use std::process::ExitCode;

mod cli;
mod commands;
mod extensions;
mod models;
mod services;
mod utils;

fn main() -> ExitCode {
    cli::run()
}
