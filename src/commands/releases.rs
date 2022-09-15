use super::command::FdmCommand;
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub struct Releases {}

impl FdmCommand for Releases {
    fn run(self) -> ExitCode {
        println!("releases command");
        ExitCode::SUCCESS
    }
}
