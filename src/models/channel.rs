use clap::ValueEnum;
use std::fmt::{Display, Formatter};

#[derive(ValueEnum, Copy, Clone, Eq, Ord, PartialEq, PartialOrd)]
pub enum Channel {
    Dev,
    Beta,
    Stable,
}

const CHANNEL_NAME_DEV: &str = "dev";
const CHANNEL_NAME_BETA: &str = "beta";
const CHANNEL_NAME_STABLE: &str = "stable";

impl TryFrom<&str> for Channel {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        use Channel::*;
        let channel = match value {
            CHANNEL_NAME_DEV => Dev,
            CHANNEL_NAME_BETA => Beta,
            CHANNEL_NAME_STABLE => Stable,
            &_ => {
                let message = format!("{} is not supported.", value);
                return Err(message);
            }
        };
        Ok(channel)
    }
}

impl Display for Channel {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        use Channel::*;
        let channel = match self {
            Dev => CHANNEL_NAME_DEV,
            Beta => CHANNEL_NAME_BETA,
            Stable => CHANNEL_NAME_STABLE,
        };
        write!(f, "{}", channel)
    }
}
