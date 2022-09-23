use indicatif::{ProgressBar, ProgressStyle};

pub fn app_progress(len: u64) -> ProgressBar {
    ProgressBar::new(len).with_style(app_progress_style())
}

fn app_progress_style() -> ProgressStyle {
    ProgressStyle::with_template(
        "{msg}\n{spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({bytes_per_sec}, {eta})"
    )
    .unwrap()
    .progress_chars("#>--")
}
