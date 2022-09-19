use std::fmt::{Display, Formatter};

pub enum SupportedOs {
    MacOs,
    Linux,
    Windows,
}

const OS_NAME_MACOS: &str = "macos";
const OS_NAME_LINUX: &str = "linux";
const OS_NAME_WINDOWS: &str = "windows";

impl TryFrom<&str> for SupportedOs {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use SupportedOs::*;
        let os = match value {
            OS_NAME_MACOS => MacOs,
            OS_NAME_LINUX => Linux,
            OS_NAME_WINDOWS => Windows,
            &_ => {
                let message = format!("{} is not supported.", value);
                return Err(message);
            }
        };
        Ok(os)
    }
}

impl Display for SupportedOs {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use SupportedOs::*;
        let os_name = match self {
            MacOs => OS_NAME_MACOS,
            Linux => OS_NAME_LINUX,
            Windows => OS_NAME_WINDOWS,
        };
        write!(f, "{}", os_name)
    }
}
