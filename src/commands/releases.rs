use super::command::FdmCommand;
use crate::models::Channel;
use crate::services;
use clap::Parser;
use std::process::ExitCode;

#[derive(Parser)]
pub struct ReleasesCommand {
    #[clap(parse(try_from_str = Channel::try_from), default_value_t = Channel::Stable)]
    channel: Channel,
}

impl FdmCommand for ReleasesCommand {
    fn run(self) -> ExitCode {
        match services::list(self.channel) {
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
