use anyhow::Result;
use reqwest::blocking::Client;

use crate::pdb::types::PdbFormat;

/// requests a pdb structure from rcsb pdb
pub fn structure(pdb_id: &str, header_only: bool, format: &PdbFormat) -> Result<Option<String>> {
    let base_url = if header_only {
        "https://files.rcsb.org/header"
    } else {
        "https://files.rcsb.org/view"
    };
    let url = format!("{}/{}.{}", base_url, pdb_id, format);
    let response = Client::new().get(url).send()?;
    if response.status().is_success() {
        Ok(Some(response.text()?))
    } else {
        Ok(None)
    }
}
