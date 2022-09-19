use crate::models::{Channel, Version};
use regex::{Match, Regex};
use reqwest::{blocking, Error};
use serde::Deserialize;

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

    let mut versions: Vec<Version> = iter
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

    versions.sort();

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
