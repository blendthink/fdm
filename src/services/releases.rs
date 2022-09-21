use crate::models::{Channel, Version};
use reqwest::{blocking, Error};
use serde::Deserialize;

#[derive(Deserialize)]
struct Response {
    prefixes: Vec<String>,
}

pub fn list(channel: Channel) -> Result<Vec<Version>, Error> {
    let request_url = format!(
        "https://storage.googleapis.com/storage/v1/b/dart-archive/o?delimiter=/&prefix=channels/{channel}/release/",
        channel = channel,
    );
    let response: Response = blocking::get(request_url)?.json()?;
    let mut versions: Vec<Version> = response
        .prefixes
        .iter()
        .filter_map(|prefix| {
            let tmp = prefix
                .strip_prefix(format!("channels/{}/release/", channel).as_str())?
                .strip_suffix('/')?;
            match Version::try_from(tmp) {
                Ok(version) => {
                    if channel == version.channel {
                        Some(version)
                    } else {
                        None
                    }
                }
                Err(_) => None,
            }
        })
        .collect();

    versions.sort();

    Ok(versions)
}
