use crate::models::channel::Channel;
use std::fmt;

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub channel: Channel,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_minor: Option<i32>,
    pub pre_patch: Option<i32>,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let version = format!(
            "{major}.{minor}.{patch}",
            major = self.major,
            minor = self.minor,
            patch = self.patch,
        );

        if let Channel::Stable = self.channel {
            return write!(f, "{}", version);
        }

        if self.pre_minor.is_none() || self.pre_patch.is_none() {
            return write!(f, "{}", version);
        }

        write!(
            f,
            "{version}-{minor}.{patch}.{channel}",
            version = version,
            minor = self.pre_minor.unwrap(),
            patch = self.pre_patch.unwrap(),
            channel = self.channel.name(),
        )
    }
}
