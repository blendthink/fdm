use regex::{Match, Regex};
use reqwest::{blocking, Error};
use serde::Deserialize;

pub struct Version {
    pub channel: Channel,
    pub major: i32,
    pub minor: i32,
    pub patch: i32,
    pub pre_minor: Option<i32>,
    pub pre_patch: Option<i32>,
}

impl ToString for Version {
    fn to_string(&self) -> String {
        let version = format!(
            "{major}.{minor}.{patch}",
            major = self.major,
            minor = self.minor,
            patch = self.patch,
        );

        if let Channel::Stable = self.channel {
            return version;
        }

        if self.pre_minor.is_none() || self.pre_patch.is_none() {
            return version;
        }

        format!(
            "{version}-{minor}.{patch}.{channel}",
            version = version,
            minor = self.pre_minor.unwrap(),
            patch = self.pre_patch.unwrap(),
            channel = self.channel.name(),
        )
    }
}

#[derive(Copy, Clone)]
pub enum Channel {
    Dev,
    Beta,
    Stable,
}

impl Channel {
    fn name(&self) -> &'static str {
        match self {
            Channel::Dev => "dev",
            Channel::Beta => "beta",
            Channel::Stable => "stable",
        }
    }
}

#[derive(Deserialize)]
struct Response {
    prefixes: Vec<String>,
}

pub fn list(channel: Channel) -> Result<Vec<Version>, Error> {
    let request_url = format!(
        "https://storage.googleapis.com/storage/v1/b/dart-archive/o?delimiter=/&prefix=channels/{channel}/release/",
        channel = channel.name(),
    );
    let response: Response = blocking::get(request_url)?.json()?;
    let reg = Regex::new(
        r"^channels/(stable|beta|dev)/release/(?P<major>\d+)\.(?P<minor>\d+)\.(?P<patch>\d+)(-(?P<pre_minor>\d+)\.(?P<pre_patch>\d+)\.(beta|dev))?/$",
    )
    .unwrap();

    let iter = response.prefixes.iter();

    let versions: Vec<Version> = iter
        .filter_map(|prefix| {
            let caps = reg.captures(prefix)?;
            let major = caps.name("major").to_int();
            let minor = caps.name("minor").to_int();
            let patch = caps.name("patch").to_int();
            let pre_minor = caps.name("pre_minor").to_int_or_null();
            let pre_patch = caps.name("pre_patch").to_int_or_null();
            Some(Version {
                channel,
                major,
                minor,
                patch,
                pre_minor,
                pre_patch,
            })
        })
        .collect();

    Ok(versions)
}

trait ToInt {
    fn to_int(self) -> i32;
    fn to_int_or_null(self) -> Option<i32>;
}

impl<'t> ToInt for Option<Match<'t>> {
    fn to_int(self) -> i32 {
        self.unwrap().as_str().parse::<i32>().unwrap()
    }

    fn to_int_or_null(self) -> Option<i32> {
        self.and_then(|m| m.as_str().parse::<i32>().ok())
    }
}
