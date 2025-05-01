use std::fs::File;
use std::io::Write;
use std::time::Duration;

use anyhow::Result;
use futures::{StreamExt, future::join_all};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;

pub const KB_LIMIT: usize = 1024;
pub const MB_LIMIT: usize = 1_048_576;
pub const GB_LIMIT: usize = 1_073_741_824;
pub const TB_LIMIT: usize = 1_099_511_627_776;

/// Converts a bits size to a string representation
#[must_use]
#[allow(clippy::cast_precision_loss, clippy::cast_sign_loss)]
pub fn convert_mem_label(size: usize) -> String {
    let (dividend, units) = if size < KB_LIMIT {
        (1.0, "B")
    } else if size < MB_LIMIT {
        (KB_LIMIT as f64, "K")
    } else if size < GB_LIMIT {
        (MB_LIMIT as f64, "M")
    } else if size < TB_LIMIT {
        (GB_LIMIT as f64, "G")
    } else {
        (TB_LIMIT as f64, "T")
    };
    let repr = (size as f64 / dividend).round() as usize;
    format!("{repr}{units}")
}

/// Download a file from a URL asynchronously
async fn download_url(url: &str, pb: ProgressBar) -> Result<()> {
    let filename = url.split('/').last().unwrap_or("");
    let client = Client::new().get(url).send().await?.error_for_status()?;

    let size = client.content_length().unwrap_or(0);
    pb.set_style(ProgressStyle::default_bar()
        .template(
            "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta}) {msg}")?
        .progress_chars("#>-"));
    pb.set_length(size);
    pb.set_message(filename.to_string());

    let mut file = File::create(filename)?;
    let mut stream = client.bytes_stream();
    while let Some(item) = stream.next().await {
        let chunk = item?;
        pb.inc(chunk.len() as u64);
        file.write_all(&chunk)?;
    }
    pb.finish();
    Ok(())
}

/// Download multiple URLs asynchronously
pub async fn download_multiple(urls: &[&str]) -> Result<()> {
    let mpb = MultiProgress::new();
    let bars = (0..urls.len()).map(|_| mpb.add(ProgressBar::new(0)));
    let handles = urls.iter().zip(bars).map(|(url, pb)| download_url(url, pb));
    join_all(handles).await;
    Ok(())
}

/// Pings a provided url to see if it is accessible.
///
/// Useful for FTP clients which cannot provide a timeout
#[must_use]
pub fn ping(url: &str, timeout: usize) -> bool {
    let response = reqwest::blocking::Client::new()
        .get(url)
        .timeout(Duration::from_secs(timeout as u64))
        .send();
    response.is_ok()
}

#[cfg(test)]
mod testing {
    use super::ping;

    #[test]
    fn test_ping_true() {
        let url = "https://google.com";
        assert!(ping(url, 3));
    }

    #[test]
    fn test_ping_false() {
        let url = "https://aisdjiaowdjanwidanowidnawdawd.com";
        assert!(!ping(url, 3));
    }
}
