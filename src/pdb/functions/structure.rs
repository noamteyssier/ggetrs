use anyhow::Result;
use reqwest::blocking::Client;

pub fn structure(pdb_id: &str, header_only: bool) -> Result<Option<String>> {
    let base_url = if header_only {
        "https://files.rcsb.org/header"
    } else {
        "https://files.rcsb.org/view"
    };
    let url = format!("{}/{}.pdb", base_url, pdb_id);
    let response = Client::new()
        .get(url)
        .send()?;
    if response.status().is_success() {
        Ok(Some(response.text()?))
    } else {
        Ok(None)
    }
}
