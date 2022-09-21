use super::command::FdmCommand;
use crate::models::{Channel, SupportedArch, SupportedOs, Version};
use clap::Parser;
use std::env;
use std::process::ExitCode;

#[derive(Parser)]
pub struct InstallCommand {
    #[clap(parse(try_from_str = Version::try_from))]
    version: Version,
}

impl FdmCommand for InstallCommand {
    fn run(self) -> ExitCode {
        let version = self.version;

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
