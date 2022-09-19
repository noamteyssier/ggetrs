use crate::ncbi::types::NcbiResults;
use anyhow::{bail, Result};
use reqwest::blocking::Client;
use serde_json::Value;

/// Query information for NCBI Identifiers
pub fn query_ids(ids: &[usize]) -> Result<NcbiResults> {
    let url = "https://api.ncbi.nlm.nih.gov/datasets/v1/gene/id";

    let query = ids
        .iter()
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("%2C");

    let query_url = format!("{}/{}", url, query);

    let response = Client::new()
        .get(query_url)
        .header("accept", "application/json")
        .header("api-key", "ggetrs")
        .send()?
        .json::<Value>()?;

    match NcbiResults::from_value(&response) {
        Some(result) => {
            if result.0.len() > 0 {
                Ok(result)
            } else {
                bail!(format!("No results found for ids: {:?}", ids))
            }
        }
        None => bail!("Unable to parse response from NCBI"),
    }
}

#[cfg(test)]
mod testing {
    use super::query_ids;

    #[test]
    fn test_ncbi_ids() {
        let ids = vec![1175];
        let response = query_ids(&ids);
        assert!(response.is_ok());
    }

    #[test]
    fn test_ncbi_ids_missing() {
        let ids = vec![0000];
        let response = query_ids(&ids);
        assert!(response.is_err());
    }
}
