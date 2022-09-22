use crate::models::{Architecture, Platform, Version};
use reqwest::StatusCode;
use std::fs::File;
use std::path::Path;

type Error = &'static str;

pub fn install(
    platform: Platform,
    architecture: Architecture,
    version: Version,
) -> Result<(), Error> {
    let url = format!(
        "https://storage.googleapis.com/dart-archive/channels/{channel}/release/{version}/sdk/dartsdk-{platform}-{architecture}-release.zip",
        channel = version.channel,
        version = version,
        platform = platform,
        architecture = architecture,
    );
    let mut response = match reqwest::blocking::get(url) {
        Ok(res) => match res.status() {
            StatusCode::OK => res,
            StatusCode::NOT_FOUND => return Err("Not found"),
            _ => return Err("Server error"),
        },
        Err(_) => return Err("Server error"),
    };
    let path = Path::new("./download.zip");
    let mut file = File::create(path).unwrap();
    response.copy_to(&mut file).expect("TODO: panic message");
    Ok(())
}
