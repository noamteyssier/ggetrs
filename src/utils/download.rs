use anyhow::Result;
use futures::{future::join_all, StreamExt};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use reqwest::Client;
use std::{fs::File, io::Write};

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
    pb.set_message(format!("{}", filename));

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
