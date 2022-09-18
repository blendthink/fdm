use super::command::FdmCommand;
use crate::services::{list, Channel};
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub struct Releases {}

impl FdmCommand for Releases {
    fn run(self) -> ExitCode {
        match list(Channel::Stable) {
            Ok(versions) => {
                for version in versions {
                    println!("{}", version.to_string());
                }
                ExitCode::SUCCESS
            }
            Err(e) => {
                println!("{:?}", e);
                ExitCode::FAILURE
            }
        }
    }
}
