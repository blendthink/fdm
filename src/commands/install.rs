use super::command::FdmCommand;
use crate::models::{Architecture, Platform, Version};
use crate::services::install;
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
        let platform = match Platform::try_from(env::consts::OS) {
            Ok(supported) => supported,
            Err(message) => {
                println!("{}", message);
                return ExitCode::FAILURE;
            }
        };

        let architecture = match Architecture::try_from(env::consts::ARCH) {
            Ok(supported) => supported,
            Err(message) => {
                println!("{}", message);
                return ExitCode::FAILURE;
            }
        };

        match install(platform, architecture, self.version) {
            Ok(_) => ExitCode::SUCCESS,
            Err(e) => {
                println!("{}", e);
                ExitCode::FAILURE
            }
        }
    }
}
