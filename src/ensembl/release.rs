use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Releases {
    releases: Vec<usize>,
}
impl Releases {
    pub fn max(&self) -> usize {
        *self.releases.iter().max().expect("No releases recovered")
    }
}

/// Returns the most recent release from Ensembl REST API
pub fn release() -> anyhow::Result<usize> {
    let client = Client::new();
    let url = "https://rest.ensembl.org/info/data";
    let results = client
        .get(url)
        .header("content-type", "application/json")
        .send()?
        .json::<Releases>()?;
    Ok(results.max())
}
