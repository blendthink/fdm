use crate::extensions::ToInt;
use crate::models::channel::Channel;
use regex::Regex;
use std::fmt::{Display, Formatter, Result as DisplayResult};

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct Version {
    pub channel: Channel,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_minor: Option<i32>,
    pub pre_patch: Option<i32>,
}

impl TryFrom<&str> for Version {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let reg = Regex::new(
            r"^(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<pre_minor>\d+)\.(?P<pre_patch>\d+)\.(?P<channel>beta|dev))?$",
        ).unwrap();

        let caps = match reg.captures(value) {
            Some(caps) => caps,
            None => return Err(r"[ ^\d+\.\d+\.\d+(-\d+\.\d+\.(dev|beta))?$ ]"),
        };

        let major = caps.name("major").to_int();
        let minor = caps.name("minor").to_int();
        let patch = caps.name("patch").to_int();
        let pre_minor = caps.name("pre_minor").to_int_or_null();
        let pre_patch = caps.name("pre_patch").to_int_or_null();
        let channel = caps
            .name("channel")
            .map_or(Channel::Stable, |m| Channel::try_from(m.as_str()).unwrap());

        Ok(Version {
            channel,
            major,
            minor,
            patch,
            pre_minor,
            pre_patch,
        })
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut Formatter<'_>) -> DisplayResult {
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
            channel = self.channel,
        )
    }
}
