use super::command::FdmCommand;
use crate::models::Channel;
use crate::services;
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub struct Releases {}

impl FdmCommand for Releases {
    fn run(self) -> ExitCode {
        match services::list(Channel::Stable) {
            Ok(versions) => {
                for version in versions {
                    println!("{}", version);
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
