use crate::models::{Architecture, Platform, Version};
use crate::utils::console;
use futures_util::StreamExt;
use indicatif::{ProgressFinish, ProgressIterator};
use reqwest::StatusCode;
use std::borrow::Cow;
use std::cmp::min;
use std::fs;
use std::fs::File;
use std::io;
use std::io::Write;

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
    let file = match download_zip(&url) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    extract_zip(file)
}

#[tokio::main]
async fn download_zip(url: &String) -> Result<File, Error> {
    let res = match reqwest::get(url).await {
        Ok(res) => match res.status() {
            StatusCode::OK => res,
            StatusCode::NOT_FOUND => return Err("Not found"),
            _ => return Err("Server error"),
        },
        Err(_) => return Err("Server error"),
    };

    let total_size = res
        .content_length()
        .ok_or(format!("Failed to get content length from {}", url))
        .unwrap();

    let pb = console::progress!(total_size)
        .with_message(format!("Downloading {}", url))
        .with_finish(ProgressFinish::WithMessage(Cow::from("Download complete!")));

    let mut file = tempfile::tempfile().unwrap();
    let mut downloaded: u64 = 0;
    let mut stream = res.bytes_stream();

    while let Some(item) = stream.next().await {
        let chunk = item.unwrap();
        file.write_all(&chunk).unwrap();
        let new = min(downloaded + (chunk.len() as u64), total_size);
        downloaded = new;
        pb.set_position(new);
    }
    pb.finish();
    Ok(file)
}

fn extract_zip(file: File) -> Result<(), Error> {
    let mut archive = zip::ZipArchive::new(&file).unwrap();

    let archive_len = archive.len();
    let total_size = u64::try_from(archive_len).unwrap();
    let pb = console::progress!(total_size)
        .with_message("Archiving")
        .with_finish(ProgressFinish::WithMessage(Cow::from("Archive complete!")));

    for i in (0..archive_len).progress_with(pb) {
        let mut file = archive.by_index(i).unwrap();
        let output_path = match file.enclosed_name() {
            Some(path) => path.to_owned(),
            None => continue,
        };

        {
            let comment = file.comment();
            if !comment.is_empty() {
                console::debug!("File {} comment: {}", i, comment);
            }
        }

        if (*file.name()).ends_with('/') {
            console::debug!("File {} extracted to \"{}\"", i, output_path.display());
            fs::create_dir_all(&output_path).unwrap();
        } else {
            console::debug!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                output_path.display(),
                file.size()
            );
            if let Some(p) = output_path.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = File::create(&output_path).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&output_path, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }
    Ok(())
}
