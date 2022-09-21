use std::fmt::{Display, Formatter};

pub enum Platform {
    MacOs,
    Linux,
    Windows,
}

const PLATFORM_NAME_MACOS: &str = "macos";
const PLATFORM_NAME_LINUX: &str = "linux";
const PLATFORM_NAME_WINDOWS: &str = "windows";

impl TryFrom<&str> for Platform {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Platform::*;
        let platform = match value {
            PLATFORM_NAME_MACOS => MacOs,
            PLATFORM_NAME_LINUX => Linux,
            PLATFORM_NAME_WINDOWS => Windows,
            &_ => {
                let message = format!("{} is not supported.", value);
                return Err(message);
            }
        };
        Ok(platform)
    }
}

impl Display for Platform {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Platform::*;
        let platform_name = match self {
            MacOs => PLATFORM_NAME_MACOS,
            Linux => PLATFORM_NAME_LINUX,
            Windows => PLATFORM_NAME_WINDOWS,
        };
        write!(f, "{}", platform_name)
    }
}
