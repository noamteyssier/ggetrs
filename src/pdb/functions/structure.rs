use anyhow::Result;
use reqwest::blocking::Client;

pub fn structure(pdb_id: &str) -> Result<Option<String>> {
    let url = format!("https://files.rcsb.org/view/{}.pdb", pdb_id);
    let response = Client::new()
        .get(url)
        .send()?;
    if response.status().is_success() {
        Ok(Some(response.text()?))
    } else {
        Ok(None)
    }
}
