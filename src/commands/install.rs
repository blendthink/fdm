use super::command::FdmCommand;
use crate::models::{Channel, SupportedArch, SupportedOs};
use clap::Parser;
use std::env;
use std::process::ExitCode;

#[derive(Parser)]
pub struct InstallCommand {
    version: String,

    #[clap(value_enum, default_value_t = Channel::Stable)]
    channel: Channel,
}

impl FdmCommand for InstallCommand {
    fn run(self) -> ExitCode {
        let current_os = match SupportedOs::try_from(env::consts::OS) {
            Ok(supported) => supported,
            Err(message) => {
                println!("{}", message);
                return ExitCode::FAILURE;
            }
        };

        let current_arch = match SupportedArch::try_from(env::consts::ARCH) {
            Ok(supported) => supported,
            Err(message) => {
                println!("{}", message);
                return ExitCode::FAILURE;
            }
        };

        todo!()
    }
}
